"""Databricks profile checks for TACITUS analytics support.

This module intentionally depends only on the Databricks CLI. It does not read
or print access tokens, and it keeps Databricks optional for local DIALECTICA
fixture workflows.
"""

from __future__ import annotations

import argparse
import configparser
import json
import os
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Callable, Sequence


Runner = Callable[[Sequence[str]], subprocess.CompletedProcess[str]]


@dataclass(frozen=True)
class DatabricksProfile:
    """Non-secret Databricks profile fields."""

    name: str
    host: str
    account_id: str | None = None
    auth_type: str | None = None


@dataclass(frozen=True)
class DatabricksCheck:
    """Result of a local Databricks connection check."""

    profile: DatabricksProfile
    cli_available: bool
    profile_valid: bool
    message: str

    def to_json(self) -> str:
        return json.dumps(
            {
                "profile": {
                    "name": self.profile.name,
                    "host": self.profile.host,
                    "account_id": self.profile.account_id,
                    "auth_type": self.profile.auth_type,
                },
                "cli_available": self.cli_available,
                "profile_valid": self.profile_valid,
                "message": self.message,
            },
            indent=2,
            sort_keys=True,
        )


def load_profile(path: Path, name: str) -> DatabricksProfile:
    parser = configparser.ConfigParser()
    with path.open(encoding="utf-8") as handle:
        parser.read_file(handle)
    if name not in parser:
        available = ", ".join(parser.sections()) or "<none>"
        raise ValueError(f"profile '{name}' not found in {path}; available: {available}")
    section = parser[name]
    host = section.get("host", "").strip()
    if not host:
        raise ValueError(f"profile '{name}' in {path} has no host")
    return DatabricksProfile(
        name=name,
        host=host,
        account_id=section.get("account_id"),
        auth_type=section.get("auth_type"),
    )


def default_config_path() -> Path:
    return Path(os.environ.get("DATABRICKS_CONFIG_FILE", Path.home() / ".databrickscfg"))


def subprocess_runner(args: Sequence[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(args, check=False, capture_output=True, text=True)


def check_profile(
    profile_name: str = "tacitus",
    config_path: Path | None = None,
    runner: Runner = subprocess_runner,
) -> DatabricksCheck:
    path = config_path or default_config_path()
    profile = load_profile(path, profile_name)

    version = runner(["databricks", "--version"])
    if version.returncode != 0:
        return DatabricksCheck(
            profile=profile,
            cli_available=False,
            profile_valid=False,
            message=(version.stderr or version.stdout or "Databricks CLI unavailable").strip(),
        )

    profiles = runner(["databricks", "auth", "profiles"])
    profile_lines = (profiles.stdout + "\n" + profiles.stderr).splitlines()
    matching_lines = [line for line in profile_lines if line.strip().startswith(profile_name)]
    valid = any(line.rstrip().endswith("YES") for line in matching_lines)
    if valid:
        message = f"profile '{profile_name}' is valid"
    elif matching_lines:
        message = f"profile '{profile_name}' exists but is not valid"
    else:
        message = f"profile '{profile_name}' was not listed by `databricks auth profiles`"

    return DatabricksCheck(
        profile=profile,
        cli_available=True,
        profile_valid=valid,
        message=message,
    )


def main(argv: Sequence[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Check the local TACITUS Databricks profile.")
    parser.add_argument("--profile", default="tacitus", help="Databricks profile name")
    parser.add_argument("--config", type=Path, default=None, help="Path to .databrickscfg")
    parser.add_argument("--json", action="store_true", help="Print JSON output")
    args = parser.parse_args(argv)

    try:
        result = check_profile(args.profile, args.config)
    except Exception as error:  # noqa: BLE001 - CLI should report cleanly.
        print(f"databricks_check=error message={error}", file=sys.stderr)
        return 2

    if args.json:
        print(result.to_json())
    else:
        print(f"profile={result.profile.name}")
        print(f"host={result.profile.host}")
        if result.profile.account_id:
            print(f"account_id={result.profile.account_id}")
        if result.profile.auth_type:
            print(f"auth_type={result.profile.auth_type}")
        print(f"cli_available={str(result.cli_available).lower()}")
        print(f"profile_valid={str(result.profile_valid).lower()}")
        print(f"message={result.message}")
    return 0 if result.profile_valid else 1


if __name__ == "__main__":
    raise SystemExit(main())

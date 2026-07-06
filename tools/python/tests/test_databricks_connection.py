from pathlib import Path
from subprocess import CompletedProcess
import unittest

from dialectica_tools.databricks_connection import check_profile, load_profile


class DatabricksConnectionTests(unittest.TestCase):
    def test_load_profile_reads_non_secret_fields(self):
        with self._temp_config(
            [
                "[tacitus]",
                "host = https://dbc-example.cloud.databricks.com",
                "account_id = account-123",
                "auth_type = databricks-cli",
                "token = should-not-be-returned",
            ]
        ) as config:
            profile = load_profile(config, "tacitus")

        self.assertEqual(profile.name, "tacitus")
        self.assertEqual(profile.host, "https://dbc-example.cloud.databricks.com")
        self.assertEqual(profile.account_id, "account-123")
        self.assertEqual(profile.auth_type, "databricks-cli")
        self.assertFalse(hasattr(profile, "token"))

    def test_check_profile_reports_invalid_auth_profile(self):
        with self._temp_config(
            [
                "[tacitus]",
                "host = https://dbc-example.cloud.databricks.com",
                "auth_type = databricks-cli",
            ]
        ) as config:

            def runner(args):
                if args == ["databricks", "--version"]:
                    return CompletedProcess(args, 0, "Databricks CLI v0.298.0\n", "")
                if args == ["databricks", "auth", "profiles"]:
                    return CompletedProcess(
                        args,
                        0,
                        "Name               Host                                      Valid\n"
                        "tacitus (Default)  https://dbc-example.cloud.databricks.com  NO\n",
                        "",
                    )
                raise AssertionError(args)

            result = check_profile("tacitus", config, runner=runner)

        self.assertTrue(result.cli_available)
        self.assertFalse(result.profile_valid)
        self.assertEqual(result.message, "profile 'tacitus' exists but is not valid")

    def test_check_profile_reports_valid_auth_profile(self):
        with self._temp_config(
            [
                "[tacitus]",
                "host = https://dbc-example.cloud.databricks.com",
                "auth_type = databricks-cli",
            ]
        ) as config:

            def runner(args):
                if args == ["databricks", "--version"]:
                    return CompletedProcess(args, 0, "Databricks CLI v0.298.0\n", "")
                if args == ["databricks", "auth", "profiles"]:
                    return CompletedProcess(
                        args,
                        0,
                        "Name               Host                                      Valid\n"
                        "tacitus (Default)  https://dbc-example.cloud.databricks.com  YES\n",
                        "",
                    )
                raise AssertionError(args)

            result = check_profile("tacitus", config, runner=runner)

        self.assertTrue(result.cli_available)
        self.assertTrue(result.profile_valid)
        self.assertEqual(result.message, "profile 'tacitus' is valid")

    def _temp_config(self, lines: list[str]):
        import tempfile

        class TempConfig:
            def __enter__(self_inner):
                self_inner.dir = tempfile.TemporaryDirectory()
                path = Path(self_inner.dir.name) / ".databrickscfg"
                path.write_text("\n".join(lines), encoding="utf-8")
                return path

            def __exit__(self_inner, exc_type, exc, tb):
                self_inner.dir.cleanup()

        return TempConfig()


if __name__ == "__main__":
    unittest.main()

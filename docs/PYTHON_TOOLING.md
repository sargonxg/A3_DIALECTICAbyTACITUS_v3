# Python Tooling

Status: auxiliary tooling only.

Python supports DIALECTICA. It does not own the production backend, canonical
schema, review gates, or bundle writer.

## Allowed Uses

Python tools may:

- render fixture reports;
- inspect source packs;
- produce eval summaries;
- run research utilities;
- sanity-check graph exports;
- prototype marketplace analytics;
- check optional analytics-plane credentials without printing secrets.

Python tools must not:

- bypass Rust validation;
- write promoted capsule truth directly;
- require cloud credentials for local fixture workflows;
- become the API service layer;
- silently normalize graph classes outside `docs/GRAPH_PROFILE_REGISTRY.md`.

## Current Layout

```text
tools/python/
  pyproject.toml
  dialectica_tools/
    capsule_report.py
    databricks_connection.py
  tests/
    test_capsule_report.py
    test_databricks_connection.py
```

## Current Gate

```powershell
python -m compileall tools/python
python -m unittest discover tools/python/tests
```

Optional Databricks profile check:

```powershell
cd tools/python
python -m dialectica_tools.databricks_connection --profile tacitus
```

Add `ruff`, `mypy`, and `pytest` when the Python toolchain becomes large enough
to justify dependencies.

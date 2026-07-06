# Python Tooling

Python is a support toolchain for DIALECTICA. Use it for reports, fixture
inspection, analysis utilities, and research scripts that do not belong in the
Rust production spine.

Current package:

```text
dialectica_tools
  capsule_report.py          small report helpers for capsule manifests
  databricks_connection.py   non-secret Databricks CLI/profile checker
```

Run syntax checks:

```powershell
python -m compileall tools/python
python -m unittest discover tools/python/tests
```

Check the optional TACITUS Databricks profile:

```powershell
cd C:\Users\giuli\A3_DIALECTICAbyTACITUS_v3\tools\python
python -m dialectica_tools.databricks_connection --profile tacitus
```

Future commands should be added only when they are useful to a repeatable build
or eval workflow.

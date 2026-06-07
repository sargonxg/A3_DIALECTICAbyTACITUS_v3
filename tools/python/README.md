# Python Tooling

Python is a support toolchain for DIALECTICA. Use it for reports, fixture
inspection, analysis utilities, and research scripts that do not belong in the
Rust production spine.

Current package:

```text
dialectica_tools
  capsule_report.py   small report helpers for capsule manifests
```

Run syntax checks:

```powershell
python -m compileall tools/python
python -m unittest discover tools/python/tests
```

Future commands should be added only when they are useful to a repeatable build
or eval workflow.

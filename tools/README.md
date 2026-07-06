# Tools

This directory is for developer and research tooling that supports the Rust
backend without becoming the backend.

Current tool lanes:

- `python/`: eval reports, fixture inspection, source-pack analysis, and graph
  sanity checks, plus optional Databricks profile verification for TACITUS
  analytics.

Rules:

- Tools may inspect and propose capsule records.
- Tools must not bypass Rust validation, review gates, or bundle checksums.
- Tools must not require cloud credentials for local fixture workflows.
- Generated local artifacts should stay under ignored output directories.

See `docs/PYTHON_TOOLING.md` and `docs/ENGINEERING_BASELINE.md`.

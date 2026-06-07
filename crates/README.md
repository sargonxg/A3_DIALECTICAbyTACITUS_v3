# Crates

Rust crates live here.

Active crates:

- `dialectica-capsule`: capsule bundle types, schema, validation;
- `dialectica-store`: PostgreSQL access and migrations;
- `dialectica-compiler`: deterministic bundle compiler;
- `dialectica-eval`: contract and quality eval helpers;
- `dialectica-cli`: local validation and fixture generation.

The first executable slice is in `dialectica-capsule` and `dialectica-cli`:
bundle structs, directory loading, validation findings, JSON Schema export, and
CLI `validate`/`inspect`/`schema-export`.

Continue implementing them in the order listed in `docs/CODING_LEDGER.md`.

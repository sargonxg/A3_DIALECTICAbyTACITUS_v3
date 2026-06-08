# Crates

Rust crates live here.

Active crates:

- `dialectica-capsule`: v3 package and legacy bundle types, schema, validation;
- `dialectica-graph`: required embedded Ladybug projection planner, validator,
  and feature-gated builder/query adapter;
- `dialectica-extractor`: planned crate for source packs, LLM proposal records,
  model receipts, and review-trigger routing;
- `dialectica-store`: PostgreSQL access and migrations;
- `dialectica-compiler`: deterministic v3 package and `.capsule` compiler;
- `dialectica-eval`: contract and quality eval helpers;
- `dialectica-cli`: local validation and fixture generation.

The first executable slice is in `dialectica-capsule` and `dialectica-cli`:
canonical v3 fixture validation, legacy bundle compatibility, directory
loading, validation findings, JSON Schema export, and CLI
`validate`/`inspect`/`schema-export`. The Ladybug projection slice adds
`ladybug-plan`, `ladybug-check`, and feature-gated `ladybug-build` /
`ladybug-query`.

`dialectica-extractor` does not exist yet. `dialectica-compiler`,
`dialectica-store`, and `dialectica-eval` remain scaffolds until the next build
phases land.

Continue implementing them in the order listed in `docs/CODING_LEDGER.md`.

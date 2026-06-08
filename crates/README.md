# Crates

Rust crates live here.

Active crates:

- `dialectica-capsule`: v3 package and legacy bundle types, schema, validation;
- `dialectica-graph`: required embedded Ladybug projection planner, validator,
  and feature-gated builder/query adapter;
- `dialectica-extractor`: source packs, fixture-mode LLM proposal records,
  model receipts, build-plan typing, and review-trigger routing;
- `dialectica-store`: PostgreSQL access and migrations;
- `dialectica-compiler`: deterministic v3 package and `.capsule` compiler;
- `dialectica-eval`: contract and quality eval helpers;
- `dialectica-cli`: local validation and fixture generation.

The first executable slices are in `dialectica-capsule`, `dialectica-graph`,
`dialectica-extractor`, and `dialectica-cli`: canonical v3 fixture validation,
legacy bundle compatibility, Ladybug projection validation, fixture-mode
source/proposal/build-plan validation, directory loading, validation findings,
JSON Schema export, and CLI
`validate`/`inspect`/`source-pack-check`/`proposal-check`/`build-plan`/
`schema-export`. The Ladybug projection slice adds `ladybug-plan`,
`ladybug-check`, and feature-gated `ladybug-build` / `ladybug-query`.

`dialectica-extractor` now owns the first local input/proposal contract.
`dialectica-compiler`, `dialectica-store`, and `dialectica-eval` remain
scaffolds until the next build phases land.

Continue implementing them in the order listed in `docs/CODING_LEDGER.md`.

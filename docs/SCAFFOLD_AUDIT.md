# Scaffold Audit

Date: 2026-06-07

Readiness: **72/100 for coding readiness; not app-production readiness**.

The repository is ready to begin the first functional implementation pass. It
is not yet a working backend. The gap is now explicit: the workspace exists,
docs are aligned, and command gates are available, but the domain crates still
need real schema, storage, compiler, API, and fixture logic.

## Evidence Checked

- `README.md`
- `AGENTS.md`
- `Cargo.toml`
- `.github/workflows/docs.yml`
- `crates/`
- `services/`
- `fixtures/`
- `tests/`
- `docs/CODING_LEDGER.md`
- `docs/IMPLEMENTATION_BLUEPRINT.md`
- `docs/API_CONTRACT.md`
- `docs/CAPSULE_SPEC.md`
- `docs/DATA_MODEL.md`
- `docs/DEPLOYMENT.md`
- `docs/BUILD_LEDGER.md`

## Ready

- Repository identity is clear: DIALECTICA is the capsule intelligence engine
  for PRAXIS.
- Visual identity assets are present in `assets/`.
- Documentation has a source-of-truth order.
- Cargo workspace is present and testable.
- Initial crate and service boundaries exist.
- Contract-test crate exists.
- Cloud Run first deployment decision is recorded.
- PostgreSQL-first operational store decision is recorded.
- Embedded graph, semantic layer, review, rights, and marketplace concepts are
  documented before implementation.
- Command gates are defined in `docs/CODING_LEDGER.md`.
- Lane A acceptance is explicit in `docs/LANE_A_ACCEPTANCE.md`.
- API Slice 1 is explicit in `docs/API_SLICE_1.md`.
- Graph vocabulary is centralized in `docs/GRAPH_PROFILE_REGISTRY.md`.
- Python tooling is auxiliary and governed by `docs/PYTHON_TOOLING.md`.

## Not Ready Yet

- Capsule bundle structs are only scaffold primitives.
- JSON Schema generation is not implemented.
- Golden fixture source pack does not exist.
- Validator commands are not implemented.
- PostgreSQL migrations do not exist.
- API routes are scaffold binaries, not HTTP services.
- Cloud Tasks handler is a scaffold binary, not an HTTP target.
- Bundle signing/checksum logic is not implemented.
- PRAXIS context-pack export is not implemented.
- Evals are only planned.
- Cloud infrastructure has no Terraform/OpenTofu state yet.
- API service still has no HTTP framework.
- Python tooling is intentionally small and should not be treated as backend
  capability.

## Blockers Before Calling It Functional

1. Implement real capsule schema structs and validation.
2. Add golden fixture source pack and expected bundle.
3. Implement CLI `validate` and `inspect`.
4. Add bundle checksums and deterministic compiler output.
5. Add store migrations for capsules, sources, claims, graph, review, rights,
   and bundle exports.
6. Add local API health and manifest/context-pack routes.
7. Add fixture evals that compare raw, loose-doc, and capsule-augmented output.

## High-Value Fixes

- Add schema snapshots with stable diff output.
- Add `justfile` or `xtask` after command flow stabilizes.
- Add Dockerfile only after API and task handler expose real ports.
- Add local Postgres compose only when the first migration exists.
- Add OpenAPI after the API route types are implemented.

## Risk Notes

- The repo has strong architecture docs, but the coding surface must stay
  narrow or it will drift into another planning repository.
- The first coding pass must resist model-powered extraction. Schema, fixture,
  validation, and review gates must work first.
- The graph layer should remain bundle/PostgreSQL-native until eval evidence
  proves that a dedicated graph adapter is needed.

## Recommendation

Start coding with Lane A from `docs/CODING_LEDGER.md`: capsule contract types,
schema generation, validation errors, and fixture snapshots. That creates the
stable backbone needed for the store, compiler, API, and PRAXIS integration.

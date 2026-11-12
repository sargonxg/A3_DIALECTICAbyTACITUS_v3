# Local verification · 12 November 2026

The Rust 1.95.0 workspace passes formatting, locked check, Clippy with warnings denied and 81 tests (one explicitly ignored integration test). Doctor, both canonical/legacy fixture readers, source/proposal/review/promotion gates, build/archive/context-pack/schema export, Python compilation and five auxiliary Python tests pass. These are local contract/fixture results, not a cloud service or analytical-quality evaluation.

## Optional native graph reproducibility

The locked `lbug` Rust wrapper is 0.17.1. Its build script downloads a floating native `latest` unless `LBUG_VERSION` is provided. In this Linux environment that latest library failed to link with unresolved SSL symbols. Selecting the matching native 0.17.1 release fixed the failure without changing the wrapper, graph schema, fixture or architecture.

`.cargo/config.toml` now defaults `LBUG_VERSION` to `0.17.1`; an explicit operator environment override remains possible and requires its own verification. The upstream release is https://github.com/LadybugDB/ladybug/releases/tag/v0.17.1 . This is a version pin, not an independent authenticity or reproducible-native-build certification.

```sh
cargo run --locked -p dialectica-cli --features ladybug -- ladybug-query fixtures/canonical-capsules/conflict-situation-capsule 'MATCH (n:CapsuleNode) RETURN count(n) AS node_count;'
```

The fixture returns `node_count` = `9`. CI now runs this exact native smoke check in addition to the existing workspace and fixture gates. The ordinary source, proposal, rights and review boundaries remain unchanged. Hosted PostgreSQL/store, ingestion/provider, tenant and PRAXIS/App integration acceptance remain separate. No production deployment, model call, database migration or schema rewrite was performed.

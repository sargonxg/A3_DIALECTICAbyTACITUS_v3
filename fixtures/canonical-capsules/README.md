# Canonical Capsule Fixtures

This directory contains extracted v3 `.capsule` package fixtures. These are the
fixtures future compiler, API, and PRAXIS integration work should target.

Current fixture:

- `conflict-situation-capsule/`: minimal canonical Situation Capsule with
  `manifest.json.type = "situation"`, source-backed claim, `graph.jsonld`,
  embedded `graph/ladybug/capsule.lbug` projection, episode, reasoning device,
  trap, annotation, review file, runtime contract, `agent_context.md`, and
  `operations.md`.

Run:

```powershell
cargo run -p dialectica-cli -- validate fixtures/canonical-capsules/conflict-situation-capsule
cargo run -p dialectica-cli -- inspect fixtures/canonical-capsules/conflict-situation-capsule
cargo run -p dialectica-cli -- ladybug-check fixtures/canonical-capsules/conflict-situation-capsule
```

# ADR-013: Draft cognitive interchange and native package bridge

Date: 2026-10-03
Status: accepted for local draft compilation under the Cognitive Capsule master plan

The October PRAXIS master plan requires DIALECTICA to compile native Cognitive Capsules. Existing promoted spec-3.x bundles retain their format, review gates, Ladybug projection and operational store boundary.

Add a separate `dialectica-cognitive-draft/1` input contract for explicitly supplied typed node, statement and knowledge proposals plus the existing Rust `SourcePack`. The Rust compiler binds spans to immutable document content versions and verified quote digests, rejects duplicate or unresolved sources and non-proposed input, and emits `cognitive-capsule/vNext` draft JSON. It never invents missing conditions, original documents, review decisions or authors.

For the first native archive exporter, the Rust CLI invokes the explicitly supplied local PRAXIS checkout's CLI `capsule pack` command. PRAXIS's shared runtime schema, graph validator and deterministic native archive writer remain the final compatibility authority. This prevents a second divergent JSON-LD/native ZIP implementation. A standalone shared contract package can replace this local bridge when packaging warrants it.

The command requires `--allow-write`, a new output file and an explicit `--praxis-root`; it performs no network requests and has no cloud/store/credential path. All output semantic records stay proposed. Rights stay private; original documents are excluded. The exporter does not create a promoted DIALECTICA bundle, publish to a registry, or write PRAXIS canonical state. PRAXIS's existing review/promotion paths remain required for canonical use.

Tests must establish deterministic draft output, source version/quote binding, state and reference rejection, and a real Rust-to-PRAXIS native artifact roundtrip. A valid digest or structural check does not establish factual truth, human review quality or production deployment.

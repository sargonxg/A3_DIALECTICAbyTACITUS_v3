# Security and Privacy

## Security Posture

DIALECTICA handles high-value policy context, institutional knowledge, expert
reasoning, and source material. Treat every input as sensitive until classified.

## Assets

Protected assets:

- user documents;
- source artifacts;
- capsule bundles;
- review decisions;
- expert reasoning notes;
- institutional context;
- embeddings;
- graph and ontology records;
- model prompts and outputs;
- signing keys and credentials.

## Threats

Initial threats:

- prompt injection inside source documents;
- poisoned sources;
- unsupported model extraction;
- cross-tenant data leakage;
- stale context reused as current;
- reviewer identity leakage;
- secret exposure;
- artifact tampering;
- malicious capsule import;
- over-trusting optional graph adapters.

## Controls

Required foundation build controls:

- tenant and project scoping on every record;
- immutable source artifact hashes;
- model extraction receipts;
- review ledger for promotion;
- capsule bundle checksums;
- signature file for promoted bundles;
- least-privilege service accounts;
- secrets in Secret Manager;
- no secrets in logs, fixtures, or capsule exports;
- explicit stale and disputed claim warnings;
- validation before PRAXIS consumes a capsule.

## Prompt Injection Handling

Source documents may contain instructions aimed at the model.

Extraction and compilation prompts must:

- treat source text as evidence, not instruction;
- isolate system/developer instructions from source content;
- preserve suspicious instructions as source content only;
- record extraction confidence and source spans;
- require citation for derived claims.

## Privacy Rules

- Do not export private user context unless the capsule usage contract allows it.
- Minimize personal data in retrieval packs.
- Keep reviewer notes scoped to intended audiences.
- Redact secrets from source artifacts before bundle export.
- Support future capsule deletion or tombstone workflows.

## Supply Chain

Before production:

- pin dependency versions;
- run dependency audit;
- scan container images;
- verify CI provenance;
- restrict deployment credentials;
- review generated code before merge.

## Security Review Gates

Security review is required before:

- production deploy;
- PRAXIS production integration;
- adding a new model provider;
- adding a new external source connector;
- changing capsule export format;
- changing tenant isolation logic;
- making graph or semantic adapters required infrastructure.

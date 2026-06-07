# Research Backlog

This backlog keeps future research organized without blocking the foundation build.

## R1: Temporal Context Graphs

Question: Which Graphiti/Zep concepts should become required capsule semantics?

Initial answer:

- validity windows;
- episodes/provenance;
- incremental updates;
- hybrid retrieval;
- custom ontology.

Gate:

- implement these first in Postgres and bundle exports;
- only add a Graphiti adapter if evals improve.

## R2: Policy Ontologies

Question: Which ontology layers matter first for policy capsules?

Candidates:

- jurisdiction;
- institution;
- actor;
- policy instrument;
- authority;
- obligation;
- risk;
- claim;
- event;
- decision.

Gate:

- ontology terms must improve retrieval or output structure.

## R3: Intellectual Tool Library

Question: Which reasoning devices create the biggest quality lift?

foundation build candidates:

- sourceability check;
- decision clock;
- actor and incentive map;
- causal hypothesis check;
- red-team counterargument.

Later:

- ACH;
- scenario trees;
- proportionality;
- institutional capacity;
- distributional analysis;
- legitimacy analysis;
- mediation interest mapping.

## R4: PRAXIS Agent Receipts

Question: What exact read-receipt fields should PRAXIS store when a capsule is
used?

Candidate fields:

- capsule id;
- bundle digest;
- context pack version;
- source ids;
- claim ids;
- reasoning device ids;
- output contract id;
- review state;
- stale warnings triggered;
- unsupported claims returned.

## R5: MCP Exposure

Question: Should DIALECTICA expose capsules through MCP resources?

Initial answer:

- useful later for agent clients;
- should be read-only first;
- must not expose secrets or unreviewed private records;
- must preserve URI-addressed provenance.

## R6: Marketplace And Shared Capsules

Question: How should teams share expert-reviewed capsules?

Initial answer:

- signed bundles;
- public/private scopes;
- review expiry;
- fork and diff;
- source access warnings;
- reviewer reputation and institution metadata.

## R7: Deployment Promotion

Question: When does DIALECTICA need GKE?

Initial answer:

- only after Cloud Run limits block real workloads;
- likely triggers are long-running graph workloads, custom autoscaling, GPUs, or
  private multi-tenant network requirements.

## R8: Semantic Standards Profile

Question: Which standards should become formal capsule export profiles?

Initial answer:

- JSON-LD for linked graph serialization;
- PROV-O for source, model, compiler, and reviewer provenance;
- SKOS for controlled concept schemes;
- SHACL-inspired constraints for graph validation;
- ODRL for permissions, prohibitions, duties, and sharing rules;
- VC/DID-inspired reviewer attestations only when marketplace trust requires
  portable credentials.

Gate:

- standard export must improve interoperability without making the local bundle
  hard to read, validate, or debug.

## R9: Capsule Visualization

Question: What graph views does PRAXIS need first?

Initial answer:

- actor/institution map;
- source-to-claim proof view;
- contradiction clusters;
- temporal timeline;
- reasoning-device trace;
- review-state overlay.

Gate:

- graph preview should help a policy user catch source, time, or reasoning risk
  faster than a flat source list.

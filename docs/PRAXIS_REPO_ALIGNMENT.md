# PRAXIS Repo Alignment

Date: 2026-06-07

Reference checked:

- `https://github.com/sargonxg/A4_PRAXIS_CLOUD_Work`
- Remote HEAD observed during this pass: `329a3a3d8b21a9664047cdfbe56372dd9bdeed1f`

## What PRAXIS Currently Says

The PRAXIS README describes PRAXIS as a ContextCapsule-native policy workspace
for source-grounded research, memo drafting, graph work, and durable agent runs.

The visible product should stay simple:

- public Home;
- signed-in cockpit;
- Ask PRAXIS;
- capsule/core library;
- memo/source/canvas/agent panels inside the existing shell.

The runtime substrate is already GCP-shaped:

- Cloud Run;
- Cloud SQL Postgres;
- Firestore;
- Cloud Tasks;
- Pub/Sub;
- Cloud Scheduler;
- Vertex AI;
- Cloud Storage;
- Secret Manager;
- Cloud Monitoring.

## Important PRAXIS Seams

High-signal files in the PRAXIS reference repo:

- `src/lib/product/context-capsules.ts`
- `src/lib/context/capsule-service.ts`
- `src/lib/capsule-plus/index.ts`
- `src/lib/capsule-plus/postgres.ts`
- `src/lib/agent-runtime/schema.ts`
- `src/lib/agent-runtime/worker.ts`
- `src/lib/agent-runtime/postgres.ts`
- `src/lib/agent-runtime/cloud-tasks.ts`
- `src/lib/agent-runtime/capsule-read-receipt.ts`
- `src/lib/ai/ask-orchestrator.ts`
- `src/components/praxis/tasklet-praxis-workbench.tsx`
- `docs/product/CONTEXT_CAPSULE_POLICY_RESEARCH_GUIDE.md`
- `docs/adr/0050-context-capsule-unified-schema.md`
- `docs/adr/0052-graphlite-capsule-plus-embedded-graph.md`

## Alignment Rules

### 1. PRAXIS Owns The User Surface

DIALECTICA should not create a new public cockpit. PRAXIS remains the front door
and workbench.

### 2. Firestore ContextCapsules Stay PRAXIS-Canonical

For current PRAXIS user-owned capsule state, Firestore remains canonical.
DIALECTICA should compile capsule bundles and serve context packs; PRAXIS can
mirror or adapt metadata into its existing store.

### 3. Capsule+ Is The Portable Graph Bridge

PRAXIS already has Capsule+ as a portable, review-gated graph bundle with
Postgres and GraphLite seed posture. DIALECTICA should output compatible graph
slice semantics rather than invent a second graph export model.

### 4. Agent Runs Need Receipts

PRAXIS has durable agent runtime files and tests around worker execution,
Cloud Tasks, sourceability ledgers, evidence packs, runtime proof, and capsule
read receipts. DIALECTICA context packs must preserve enough identifiers for
PRAXIS to record which capsule, source, claim, and review records affected an
agent output.

### 5. Review Proposals Stay Reviewable

AI-generated graph, memory, ontology, or source-gap improvements should arrive
as proposed records or review cards. They should not silently mutate canonical
PRAXIS state.

## First Integration Contract

DIALECTICA should give PRAXIS:

- capsule manifest;
- PRAXIS context pack;
- graph slice compatible with Capsule+ semantics;
- source ledger and source ids;
- temporal warnings;
- review state;
- output contract;
- capsule health;
- read-receipt identifiers.

PRAXIS should give DIALECTICA:

- tenant/project/user identity;
- source artifact references;
- intended workflow;
- active ContextCapsule id if enriching an existing capsule;
- review policy;
- callback or polling path for job state.

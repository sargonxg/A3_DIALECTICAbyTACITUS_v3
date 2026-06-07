# About DIALECTICA

## Product Definition

DIALECTICA is the TACITUS context-capsule engine for PRAXIS.

It builds durable knowledge-work objects that can be used by humans and AI
agents interchangeably. A capsule is not a prompt, not a chat memory, and not a
document summary. It is a signed bundle that carries the situation, sources,
time, ontology, embedded graph, reasoning devices, review state, rights, and
output rules needed for serious work.

## The Backbone Idea

Most AI workflows pass loose context into a model and hope the model keeps the
right facts, source status, time horizon, caveats, and reasoning method in
mind. DIALECTICA turns those elements into a reusable object.

```text
policy work today:
  documents + notes + expert memory + prompts -> one-off answer

DIALECTICA:
  documents + notes + expert review + graph + reasoning -> capsule
                                                     |
                                                     v
                              humans and AI agents use the same object
```

The long-term aim is a capsule library and marketplace where expert-reviewed
knowledge objects can be shared, forked, combined, retired, and improved.

## What A Capsule Must Carry

| Layer | Human value | AI value |
| --- | --- | --- |
| Identity | who the work is for and under what mandate | voice, scope, and authorization context |
| Situation | what is happening and why it matters | task-specific grounding |
| Sources | where claims come from | citation and retrieval anchors |
| Time | what is current, stale, superseded, or forecast | temporal answer discipline |
| Ontology | the concepts and categories that make the issue legible | stable semantic labels |
| Embedded graph | actors, claims, events, sources, risks, decisions | structured traversal and conflict detection |
| Reasoning devices | expert methods, heuristics, and failure modes | guided analysis beyond retrieval |
| Review | human approval, caveats, expiry, and rejection | trust boundary and promotion gate |
| Output contract | what artifact is allowed and how it should be shaped | response schema and refusal rules |
| Rights | who can use, export, fork, or list the capsule | sharing and policy enforcement |

## What PRAXIS Gets

PRAXIS should be able to load one or many capsules and know:

- what context is approved for the user workflow;
- which claims are source-backed, stale, contested, or reviewer-caveated;
- which graph nodes and edges influenced the answer;
- which expert thinking device should shape the analysis;
- which outputs are allowed, blocked, or require escalation;
- which capsule versions and bundle digests were used.

That gives PRAXIS a stronger substrate than generic retrieval or chat memory.

## What DIALECTICA Does Not Pretend

- It is not a public standalone app in this repository.
- It is not a replacement for PRAXIS.
- It is not an autonomous truth engine.
- It does not make machine extraction canonical without review state.
- It does not require a graph database before the bundle and PostgreSQL
  contract prove useful.

## Build Posture

Start with a boring, inspectable backend:

- Rust workspace for contract, compiler, store, API, task handler, CLI, and
  eval crates.
- PostgreSQL for operational records.
- Signed bundle exports for portability.
- Cloud Run for services and task handlers.
- Python tooling only where it earns its place: eval reports, research scripts,
  source adapters, or analysis utilities that should not live in the Rust core.

The first valuable product is a validated capsule that a human reviewer can
inspect and PRAXIS can use.

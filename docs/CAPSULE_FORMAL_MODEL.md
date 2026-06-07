# Capsule Formal Model

## Definition

A PRAXIS Capsule is a portable knowledge-work object for human and AI use:

```text
C = <I, S, E, T, O, G, R, L, A, P, U, V, M, X>
```

Where:

- `I` is identity context;
- `S` is situation context;
- `E` is evidence and source ledger;
- `T` is temporal state;
- `O` is the ontology blueprint plus local semantic layer;
- `G` is graph slice;
- `R` is reasoning playbook;
- `L` is human-gated language profile;
- `A` is agent guidance;
- `P` is PRAXIS retrieval/context pack;
- `U` is usage and output contract;
- `V` is human review and validation ledger;
- `M` is marketplace, rights, compatibility, and lineage metadata;
- `X` is export metadata, checksums, and signature.

## Capsule Layers

| Layer | Question answered | foundation build representation |
| --- | --- | --- |
| Identity | Who is working, for whom, and under what mandate? | `capsule.json.identity_context` |
| Situation | What is happening and what decision horizon matters? | `capsule.json.situation_context` |
| Evidence | What sources support each claim? | `source_ledger.jsonl` |
| Time | What is true now, stale, superseded, or contested? | `temporal_ledger.jsonl` |
| Ontology | What concepts, categories, lenses, and local meanings make this capsule legible? | `ontology_slice.json`, `ontology-plan` CLI |
| Graph | What source-backed objects and relationships does this capsule need PRAXIS to traverse? | `graph_slice.json` |
| Reasoning | How should an expert think through the situation? | `reasoning_playbook.json` |
| Language | Which terms, caveats, voice, and framings are approved? | `language_profile.json` |
| Agent Guidance | What may PRAXIS agents do, cite, refuse, and hand off? | `agent_guidance.json` |
| Retrieval | What compact context should PRAXIS inject? | `retrieval_pack.jsonl` |
| Output | What artifacts should be produced and under what rules? | `output_contracts.json` |
| Review | What has a human approved, rejected, caveated, or escalated? | `review_ledger.jsonl` |
| Market | How can the capsule be shared, forked, listed, or combined? | `rights_profile.json`, `marketplace_listing.json` |
| Export | Is the bundle complete, signed, and compatible? | `manifest.json`, checksums, signature |

## Invariants

Every valid capsule must satisfy:

- every claim has provenance;
- every capsule has a capsule-specific ontology blueprint before graph
  extraction is promoted;
- every graph edge has provenance and review state;
- every temporal claim has at least one time dimension;
- every retrieval record points to sources or review notes;
- every agent guidance policy names allowed workflows, tool rules, graph-use
  rules, stop conditions, and required receipts;
- every promoted language rule has review state, scope, and rationale;
- every output contract declares citation and uncertainty rules;
- every promoted capsule has passed required review gates;
- every marketplace capsule declares usage rights, lineage, caveats, and
  freshness;
- every bundle has deterministic checksums.

## Canonical Versus Derived

Canonical:

- source artifacts;
- PostgreSQL records;
- review decisions;
- signed bundle exports.

Derived:

- embeddings;
- graph projections;
- MCP resources;
- search indexes;
- advisory memory;
- PRAXIS UI mirrors.

Derived data can be rebuilt from canonical records and bundle artifacts.

## Capsule Health

Capsule health should be computed from:

- source coverage;
- unsupported claim count;
- stale claim count;
- contested claim severity;
- ontology coverage;
- graph provenance coverage;
- review coverage;
- output contract completeness;
- agent guidance completeness;
- language profile completeness;
- PRAXIS eval performance.

Health is not a cosmetic score. It decides whether PRAXIS can rely on a capsule
for a high-stakes workflow.

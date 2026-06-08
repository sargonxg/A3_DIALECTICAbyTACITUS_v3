# Capsule Formal Model

Status: aligned with [Capsule Spec v3.0](CAPSULE_SPEC.md).

## Definition

A PRAXIS Capsule is a portable knowledge-work object for human and AI use:

```text
C = <M, E, C, G, T, O, R, V, U, A, X>
```

Where:

- `M` is the manifest and identity envelope;
- `E` is evidence: sources, spans, hashes, rights, provenance;
- `C` is claims: atomic typed facts with trust, temporality, and source spans;
- `G` is the connected graph: entities, relations, named layer graphs, and
  substrate-guidance links;
- `T` is temporal and episodic state;
- `O` is ontology and semantic typing: ACO plus capsule-specific cores;
- `R` is reasoning guidance: devices, heuristics, salience, traps, precedents,
  annotations, and executed reasoning plans;
- `V` is governance: trust layers, review, signoff, dissent, corroboration;
- `U` is runtime contract: citation, retrieval, composition, refusal, and
  engine-less operation rules;
- `A` is agent-facing generated view: `agent_context.md` and `operations.md`;
- `X` is integrity: Merkle root, signature, version, cache regeneration state.

## Canonical Layers

| Layer | Question answered | v3 representation |
| --- | --- | --- |
| Evidence | What source material supports the capsule? | `evidence/sources.jsonl`, `evidence/blobs/`, `g:evidence` |
| Claims | What are the atomic assertions? | `claims.jsonl`, `g:claims` |
| Graph / relation | How do entities, claims, sources, and reasoning objects relate? | `graph.jsonld`, `g:situation` |
| Temporal / episodic | What happened, when, and what changed? | `episodes.json`, `g:temporal` |
| Semantic / ontology | Which meanings, types, cores, and validation rules apply? | `g:ontology`, SHACL-ready core files when added |
| Reasoning / guidance | How should experts and agents reason with the substrate? | `reasoning/*.json`, `g:reasoning` |
| Memory | How was the capsule built and used? | `g:memory`, optional first-build files |
| Governance / trust | What did humans approve, caveat, dispute, or reject? | `review/review.json`, `g:governance` |
| Runtime / contract | What may an agent retrieve, cite, combine, and produce? | `runtime.json`, `g:runtime`, `operations.md` |

## Invariants

Every valid promoted capsule must satisfy:

- `manifest.json.type` is exactly `user`, `situation`, `tool`, or `output`;
- every Situation claim has provenance, trust layer, temporality, and source
  span or review lineage;
- substrate and guidance objects are connected in `graph.jsonld`;
- the capsule remains auditable from open files and queryable through the
  required embedded Ladybug projection;
- non-required caches are optional, regenerable, and excluded from promotion
  decisions;
- `agent_context.md` and `operations.md` are generated from canonical files;
- every agent-proposed improvement enters as T3 until human gated;
- conflicts are surfaced and never silently merged;
- every promoted capsule has deterministic integrity and review evidence.

## Canonical Versus Derived

Canonical:

- immutable source artifacts;
- PostgreSQL operational records;
- v3 `.capsule` package files;
- human review decisions;
- signed exports.

Derived:

- Ladybug embedded projection;
- Oxigraph cache;
- embeddings;
- graph projections;
- MCP resources;
- search indexes;
- advisory memory;
- PRAXIS Firestore visibility mirrors.

Derived data can be rebuilt from canonical records and capsule artifacts.
Ladybug is the required derived graph artifact for promoted capsules, not a
backdoor write path for promoted claims.

## Capsule Health

Capsule health should be computed from:

- source coverage;
- unsupported claim count;
- stale or superseded claim count;
- disputed claim severity;
- ontology coverage;
- graph connectivity and provenance coverage;
- reasoning-guidance coverage;
- review coverage;
- runtime contract completeness;
- PRAXIS eval performance.

Health is not a cosmetic score. It decides whether PRAXIS can rely on a capsule
for a high-stakes workflow.

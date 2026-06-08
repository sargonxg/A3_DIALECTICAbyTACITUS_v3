# Capsule Structure Guide

Status: implementation guide for the v3 `.capsule` package. The normative
contract is [Capsule Spec v3.0](CAPSULE_SPEC.md).

## Purpose

This guide explains what a PRAXIS Capsule contains and how agents should use
it. It bridges product story, Rust validators, fixture examples, and future
compiler/API work.

Every capsule should be useful to three readers:

- a human analyst inspecting the work;
- a human reviewer deciding whether it can be trusted;
- a PRAXIS agent that needs structured context, graph guidance, citations, and
  output rules.

## Canonical Stack

```text
PRAXIS Capsule (.capsule)
  |
  +-- manifest.json          id, spec version, type, category, cores, hash
  +-- claims.jsonl           atomic typed claims and trust state
  +-- graph.jsonld           connected graph; named graphs are layers
  +-- episodes.json          temporal and episodic model
  +-- evidence/sources.jsonl sources, spans, hashes, rights
  +-- reasoning/             devices, heuristics, salience, traps, precedents,
  |                          annotations, plan/results
  +-- payload.<type>.json    user/tool/output payload; situation may omit
  +-- review/review.json     trust layers, review, dissent, open questions
  +-- runtime.json           verbs, citation, composition, refusal rules
  +-- agent_context.md       bounded self-citing context block for LLMs
  +-- operations.md          engine-less operating card
  +-- cache/                 optional Ladybug/Oxigraph/vector/FTS caches
```

`cache/` is optional and regenerable. PRAXIS must be able to load the capsule
from canonical files with no graph engine running.

## Required Layer Contract

| Layer | Required questions | Primary consumers |
| --- | --- | --- |
| Evidence | Which source spans, hashes, rights, and retrieval receipts exist? | citation engine, reviewers |
| Claims | Which atomic facts are asserted, disputed, stale, or proposed? | PRAXIS Ask, evaluators |
| Graph / relation | How do sources, claims, actors, episodes, tools, and review notes connect? | graph preview, context planner |
| Temporal / episodic | What changed, when, and what is current or superseded? | decision clock, warnings |
| Ontology / semantic | Which core, domain, method, or output schema gives meaning? | compiler, validators, PRAXIS lenses |
| Reasoning / guidance | Which expert devices, heuristics, traps, and annotations guide thinking? | analysts, reviewers, agents |
| Memory | What build or use history should travel with the capsule? | future agent memory adapters |
| Governance / trust | What has a human approved, rejected, caveated, or escalated? | promotion gate, audit |
| Runtime / contract | What may the agent retrieve, cite, combine, refuse, and output? | PRAXIS agents |

## Embedded Graph Requirements

The embedded graph is not a decoration. `graph.jsonld` is the canonical map of
the capsule, and its named graphs correspond to the layer model:

```text
g:evidence
g:claims
g:situation
g:temporal
g:ontology
g:reasoning
g:memory
g:governance
g:runtime
```

Every promoted graph object needs:

- source spans, review actions, or explicit expert notes;
- temporal scope when the fact can change;
- review state and reviewer linkage;
- a human-readable explanation;
- enough stable identifiers for PRAXIS graph preview and composition.

Graph engines such as Ladybug may accelerate traversal, full-text search, and
vector search, but they remain optional caches.

## Semantic Layer Requirements

The semantic layer should be practical first and standards-compatible second:

- JSON-LD is the canonical graph serialization.
- PROV-O anchors source, extraction, review, and compile provenance.
- SKOS anchors controlled concepts.
- SHACL validates loaded cores.
- ODRL can anchor usage rights.
- OWL remains optional for richer ontology inference once the simple slice
  works.

Do not treat `actor`, `claim`, and `institution` as the default ontology for
all capsules. Those classes are central for situation and conflict work. User,
Tool, and Output Capsules need different local semantic layers. The shared graph
registry gives PRAXIS stable export names; each capsule's cores and reasoning
objects provide its expert lens.

## Capsule Type Profiles

PRAXIS imports exactly four top-level macro types:

| Type | Must emphasize | Agent behavior |
| --- | --- | --- |
| `user` | preferences, expertise, voice, authority, privacy | personalize only inside explicit scope |
| `situation` | sources, live facts, actors, claims, stakeholders, risks, caveats, domain meaning, decision clock | answer with temporal and source discipline |
| `tool` | method steps, intellectual lenses, failure modes, examples, review criteria | structure reasoning before drafting |
| `output` | artifact lineage, citations, caveats, reuse rules | reuse or update only within contract |

All other labels are internal specialization. A conflict Situation Capsule may
contain stakeholder, source-proof, scenario, and ontology lenses; it is still a
`situation` capsule.

## Runtime Contract

`runtime.json` and `operations.md` must tell PRAXIS agents:

- which workflows are allowed;
- which verbs are available;
- which claims require citation;
- how to traverse or ignore optional graph caches;
- which reasoning devices to apply first;
- which traps must trigger critique;
- which output contract controls the answer;
- which warnings block or downgrade the answer;
- when to ask a human reviewer instead of proceeding.

## Build Workflow

```text
source pack
  -> source and span records
  -> claim atoms
  -> episodes
  -> ontology cores
  -> graph.jsonld
  -> reasoning guidance
  -> review gate
  -> runtime contract
  -> agent_context.md + operations.md
  -> .capsule archive
  -> PRAXIS context pack
```

Each arrow must leave receipts. Model extraction can propose records, but the
review layer decides what becomes usable for grounding.

## Fixtures

Canonical fixture:

- `fixtures/canonical-capsules/conflict-situation-capsule`

Legacy single-file examples remain useful for product shape but should be
migrated to the v3 manifest vocabulary:

- `fixtures/example-capsules/user-capsule.example.json`
- `fixtures/example-capsules/situation-capsule.example.json`
- `fixtures/example-capsules/tool-capsule.example.json`
- `fixtures/example-capsules/output-capsule.example.json`

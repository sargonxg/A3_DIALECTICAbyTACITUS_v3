# Capsule Structure Guide

Status: implementation guide for bundle shape, examples, and future schema work.

## Purpose

This guide explains what a PRAXIS Capsule contains and how agents should use it.
It is the bridge between the product story, the JSON schema work, and the
fixture examples under `fixtures/example-capsules/`.

Every capsule should be useful to three readers:

- a human analyst inspecting the work;
- a human reviewer deciding whether the work can be trusted;
- a PRAXIS agent that needs structured context, graph guidance, citations, and
  output rules.

## Capsule Stack

```text
PRAXIS Capsule
  |
  +-- manifest              identity, type, status, digest, compatibility
  +-- capsule               user/team/situation/policy context
  +-- source ledger         documents, spans, hashes, provenance
  +-- temporal ledger       validity windows, stale/superseded/contested facts
  +-- ontology slice        local concepts, mappings, frames, deprecations
  +-- embedded graph        nodes, edges, communities, review state
  +-- graph semantics       JSON-LD / PROV-O / SKOS / ODRL export view
  +-- graph constraints     SHACL-like profile and graph validation rules
  +-- reasoning playbook    expert methods, steps, failure modes
  +-- language profile      reviewed terms, voice, framing, translation rules
  +-- agent guidance        model-facing instructions and tool policy
  +-- retrieval pack        ranked source/context snippets
  +-- output contracts      allowed artifacts and refusal rules
  +-- review ledger         human decisions, caveats, expiry, promotion gates
  +-- rights profile        permission, prohibition, duties, sharing policy
  +-- marketplace listing   discoverability and trust metadata
  +-- capsule health        quality, risk, coverage, freshness
  +-- eval report           measured outcome and regression signals
  +-- checksums/signature   portability and integrity
```

The production bundle stores these as individual files. The examples in
`fixtures/example-capsules/*.example.json` use a single-file envelope with the
same sections so reviewers and agents can inspect the full shape quickly.

## Required Layer Contract

| Layer | Required questions | Primary consumers |
| --- | --- | --- |
| `manifest` | What is this capsule, can it be used, and what digest identifies it? | PRAXIS library, compiler, marketplace |
| `capsule` | Who or what is modeled, and under what mandate, scope, and boundary? | analysts, PRAXIS Ask, reviewers |
| `source_ledger` | Which source spans support the claims and graph edges? | citation engine, source inspector |
| `temporal_ledger` | What is current, stale, superseded, forecast, or contested? | answer planner, decision clock |
| `ontology_slice` | What terms, frames, and mappings make the issue legible? | graph builder, concept inspector |
| `graph_slice` | How do actors, claims, sources, events, risks, tools, and outputs relate? | PRAXIS graph UI, context planner |
| `graph_semantics` | How can the capsule be exported to linked-data systems? | interoperability adapters |
| `graph_constraints` | Which graph classes, fields, and review rules must validate? | Rust validator, reviewer |
| `reasoning_playbook` | Which expert method should structure the analysis? | analyst, reviewer, agent planner |
| `language_profile` | Which terms, voice, caveats, and framings are approved or blocked? | analyst, reviewer, PRAXIS agents |
| `agent_guidance` | What may the model do, cite, ask, refuse, and hand off? | PRAXIS agents |
| `retrieval_pack` | Which compact source/context units should enter the model context? | retrieval and context pack API |
| `output_contracts` | Which artifacts are allowed and how must they be shaped? | memo/brief builders |
| `review_ledger` | Who approved or caveated which objects and when does review expire? | trust layer, promotion gate |
| `rights_profile` | Who may use, fork, export, or list the capsule? | marketplace, sharing, policy |
| `marketplace_listing` | How is the capsule discovered without leaking private content? | capsule library |
| `capsule_health` | What is weak, stale, unsupported, or risky? | reviewers, CI, evals |
| `eval_report` | Did this capsule improve a real workflow? | product and quality gates |

## Embedded Graph Requirements

The embedded graph is not a decoration. It is a compact, reviewable map of the
knowledge object. Every promoted graph object needs:

- a registered node or edge class from `docs/GRAPH_PROFILE_REGISTRY.md`;
- source spans or review actions that justify it;
- temporal scope when the fact can change;
- review state and reviewer linkage;
- a human-readable explanation;
- PRAXIS visualization hints.

Graph engines such as LadybugDB may accelerate projection, exploration, and
graph algorithms, but the signed bundle and PostgreSQL ledger remain canonical.

## Semantic Layer Requirements

The semantic layer should be practical first and standards-compatible second:

- JSON-LD gives a JSON-compatible linked-data export.
- PROV-O anchors provenance for source, extraction, review, and compile runs.
- SKOS anchors controlled policy vocabularies and concept schemes.
- SHACL inspires graph validation constraints.
- ODRL anchors usage rights, prohibitions, duties, and sharing policies.
- OWL remains optional for richer ontology inference once the simple slice works.

Do not require a full RDF stack in the first validator. Design fields so a
later RDF/OWL/SHACL adapter can be built without changing the capsule contract.

## Capsule Type Profiles

| Type | Must emphasize | Graph profile | Agent behavior |
| --- | --- | --- | --- |
| User Capsule | preferences, expertise, voice, permission boundary | `user_context_graph_v1` | personalize only inside explicit scope |
| Situation Capsule | live facts, actors, claims, risks, decision clock | `situation_graph_v1` | answer with temporal and source discipline |
| Thinking Device Capsule | method steps, failure modes, examples, review criteria | `reasoning_device_graph_v1` | structure reasoning before drafting |
| Output Capsule | artifact lineage, citations, caveats, reuse rules | `output_trace_graph_v1` | reuse or update only within contract |

Other capsule types follow the same layer structure and specialize the profile,
not the bundle format.

## Language Profile Contract

`language_profile.json` captures the human-gated language layer. It should not
be reduced to "tone." Policy work needs reviewed terminology and framing rules
because words can imply legal status, responsibility, certainty, legitimacy, or
scope.

The profile should include:

- primary language and supported secondary languages;
- approved terms, aliases, definitions, and deprecated terms;
- terms that require caveats, jurisdictions, or date ranges;
- forbidden framings, overclaims, euphemisms, or identity labels;
- audience register, voice, and institutional style rules;
- translation notes and terms that must not be translated literally;
- citation and uncertainty language;
- review state for every material language rule.

Agents must treat rejected or unreviewed language rules the same way they treat
unreviewed graph edges: visible in audit views, blocked from promoted outputs
unless a workflow explicitly asks for draft material.

## Agent Guidance Contract

`agent_guidance.json` should tell PRAXIS agents:

- which workflows are allowed;
- which tools or connectors may be used;
- which claims require source citation;
- how to traverse the embedded graph;
- which reasoning devices to apply first;
- which language profile rules to enforce;
- which output contract controls the answer;
- which warnings block or downgrade the answer;
- when to ask a human reviewer instead of proceeding.

Example policies:

```json
{
  "allowed_workflows": ["decision_brief", "stakeholder_map"],
  "citation_policy": "cite_source_span_for_every_nontrivial_claim",
  "graph_use_policy": "prefer approved current edges; hide rejected edges unless asked for audit",
  "language_profile_refs": ["language:policy-brief-en-v1"],
  "stop_conditions": ["missing_source_for_material_claim", "rights_policy_blocks_workflow"],
  "handoff_policy": "ask reviewer for approval when output would become public"
}
```

## Build Workflow

```text
source pack
  -> source ledger
  -> temporal ledger
  -> ontology slice
  -> embedded graph
  -> reasoning playbook
  -> language profile
  -> agent guidance
  -> review gate
  -> signed bundle
  -> PRAXIS context pack
```

Each arrow must leave receipts. Model extraction can propose records, but the
review ledger decides what becomes usable for grounding.

## Example Fixtures

Use these examples when implementing Lane A and Lane B:

- `fixtures/example-capsules/user-capsule.example.json`
- `fixtures/example-capsules/situation-capsule.example.json`
- `fixtures/example-capsules/thinking-device-capsule.example.json`
- `fixtures/example-capsules/output-capsule.example.json`

They are intentionally small. Their purpose is to lock the shape before larger
policy fixtures are added.

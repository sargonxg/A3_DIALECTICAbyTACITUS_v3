# LLM Context Extraction Architecture

Status: target architecture; not yet implemented.

This document defines how DIALECTICA should use LLMs to build PRAXIS Capsules
without letting model output become unchecked truth. The goal is not a generic
RAG pipeline. The goal is a capsule-building engine that can read documents,
user conversations, expert notes, and review decisions; build a source-backed
understanding of a user, situation, tool, or output; and package that
understanding into a portable capsule PRAXIS can use.

## Core Rule

LLMs propose. Rust normalizes, validates, and compiles. Humans promote when the
object requires judgment.

```text
untrusted inputs
  -> source spans
  -> LLM proposal records
  -> deterministic normalization
  -> validator and cross-check agents
  -> human review gates
  -> canonical records
  -> v3 .capsule compiler
  -> PRAXIS context pack
```

No model-generated claim, graph edge, language rule, reasoning rule, rights
rule, or expert-style instruction becomes canonical unless it is source-backed
or review-backed.

## Four Capsule Targets

PRAXIS imports exactly four top-level capsule types:

| Type | What the extractor builds |
| --- | --- |
| `user` | User preferences, role, organization context, writing style, recurring review standards, language constraints, known caveats, and allowed workflows. |
| `situation` | Issue state, source-backed claims, temporal episodes, actor maps, causal hypotheses, constraints, uncertainty, stakeholder positions, and situation-specific ontology. |
| `tool` | Expert method, intellectual tool, reasoning playbook, procedural steps, failure modes, evidence requirements, and examples of expert use. |
| `output` | Output schema, audience, format, acceptance rules, citation requirements, review standards, language constraints, and handoff contract. |

Specialized concepts such as stakeholder maps, conflict maps, causal graphs,
source packs, expert lenses, and language rules are internal layers. They are
not additional top-level capsule types.

## Engine Shape

```text
                         +------------------------------+
                         |        PRAXIS asks for       |
                         |  user/situation/tool/output  |
                         +---------------+--------------+
                                         |
documents, PDFs, notes, chat turns       v
expert examples, source URLs     +---------------+
        +----------------------->| Source Intake |
        |                        +-------+-------+
        |                                |
        |                                v
        |                      +------------------+
        |                      | Source Span Map  |
        |                      | hashes + locators|
        |                      +--------+---------+
        |                               |
        |                               v
        |                  +---------------------------+
        |                  | Capsule Intent Planner    |
        |                  | type + domain + workflows |
        |                  +-------------+-------------+
        |                                |
        |                                v
        |                  +---------------------------+
        |                  | Ontology/Semantic Creator |
        |                  | capsule-specific lenses   |
        |                  +-------------+-------------+
        |                                |
        |                                v
        |         +-----------------------------------------------+
        |         | LLM Extraction Passes                         |
        |         | claims, time, graph, reasoning, language,     |
        |         | review triggers, output guidance              |
        |         +----------------------+------------------------+
        |                                |
        |                                v
        |        +------------------------------------------------+
        |        | Cross-Check Agents                             |
        |        | sourceability, temporal, contradiction, graph,  |
        |        | ontology, rights, security, human-gate routing  |
        |        +----------------------+-------------------------+
        |                               |
        |                               v
        |                  +---------------------------+
        |                  | Human Review Queue        |
        |                  | approve, reject, caveat   |
        |                  +-------------+-------------+
        |                                |
        |                                v
        |                  +---------------------------+
        +------------------| Rust Compiler             |
                           | v3 package + .capsule     |
                           +-------------+-------------+
                                         |
                                         v
                           +---------------------------+
                           | PRAXIS Context Pack       |
                           | agent_context + graph     |
                           +---------------------------+
```

## Rust Ownership

Current code:

- `dialectica-capsule` owns v3 package types, legacy compatibility types,
  validators, schema export, and inspection summaries.
- `dialectica-cli` owns local proof commands and switches between canonical v3
  packages and legacy migration bundles.
- `dialectica-compiler` is only a scaffold with a legacy review-gate helper.
- `dialectica-store`, `dialectica-eval`, `dialectica-api`, and
  `dialectica-task-handler` are scaffolds.

Required next ownership:

```text
dialectica-extractor
  owns source-pack inputs, model proposal schemas, extraction orchestration,
  model invocation receipts, review-trigger routing, and provider traits

dialectica-capsule
  owns canonical capsule records, validator, schema, context-pack types, and
  promotion rules

dialectica-compiler
  owns deterministic v3 package writing, .capsule archive assembly, hashes,
  signatures, and compiler receipts

dialectica-store
  owns PostgreSQL records for sources, spans, proposals, reviews, canonical
  records, graph objects, exports, and jobs

dialectica-api
  owns PRAXIS-facing HTTP routes and operator build/review routes

dialectica-task-handler
  owns queued ingestion, extraction, review-check, compile, export, and eval
  jobs
```

Dependency rule:

```text
api/task-handler -> store/extractor/compiler/capsule/eval
extractor        -> capsule
compiler         -> capsule
store            -> capsule-compatible IDs and records
eval             -> capsule
capsule          -> no project-local dependencies
```

Provider clients should be injected through service configuration. The core
record schemas must not hardcode one model provider.

## Proposal Records

The extractor should emit proposal records first. A proposal record is not a
canonical claim.

Every proposal must include:

- `proposal_id`;
- `capsule_id`;
- `capsule_type`;
- `proposal_type`;
- normalized payload;
- `source_span_ids`;
- model name and provider;
- prompt or prompt-template version;
- extraction run id;
- confidence;
- uncertainty reason;
- review trigger list;
- created time;
- status: `proposed`, `needs_review`, `approved`, `approved_with_caveats`,
  `rejected`, or `superseded`.

Core proposal families:

| Proposal family | Becomes canonical only after |
| --- | --- |
| `proposed_claim` | source span validation and, for material claims, review approval |
| `proposed_episode` | temporal validation and date/source checks |
| `proposed_graph_node` | ontology compatibility and duplicate/entity merge checks |
| `proposed_graph_edge` | source span or review action plus graph integrity checks |
| `proposed_ontology_term` | domain scope and expert/language review where needed |
| `proposed_reasoning_device` | method-source or expert review |
| `proposed_language_rule` | human language review |
| `proposed_output_rule` | output owner or reviewer approval |
| `proposed_caveat` | reviewer approval or deterministic trigger |
| `proposed_rights_rule` | rights/privacy review |

## Extraction Passes

### 1. Source Intake

Input may include uploaded documents, PDFs, web captures, notes, transcripts,
and user/assistant conversations. Intake should:

- preserve the original file;
- hash the file;
- extract text and page/paragraph/time locators;
- classify source type, language, access rights, and trust state;
- quarantine prompt-injection-like instructions found inside sources;
- create stable `source_id` and `source_span_id` values.

### 2. Capsule Intent Planner

The system identifies the target capsule:

- type: `user`, `situation`, `tool`, or `output`;
- domain and time window;
- audience and workflows;
- expected output from PRAXIS;
- needed semantic layers;
- review strictness.

This planner can use an LLM, but it must produce a reviewable plan. The plan
does not change source records.

### 3. Ontology And Semantic Layer Creator

Each capsule needs its own semantic structure. A conflict Situation Capsule, an
analyst User Capsule, and a stakeholder-analysis Tool Capsule should not share a
single rigid ontology.

The creator proposes:

- domain terms and definitions;
- actor, institution, event, claim, risk, decision, source, and concept classes;
- graph lenses needed for the capsule;
- semantic layers such as source proof, stakeholder power, scenario causality,
  legal authority, economic instrument, conflict map, language register, or
  review constraints;
- extraction questions for each layer.

The result is an ontology blueprint. It guides extraction, validation, graph
construction, and PRAXIS context packing.

### 4. Claim And Evidence Extraction

The LLM reads bounded source spans and proposes atomic claims.

Each claim proposal must answer:

- What exactly is asserted?
- Which source span supports it?
- Is it direct, inferred, contested, stale, forecast, or unknown?
- What is the valid time?
- What domain term or ontology class does it use?
- What uncertainty or caveat must travel with it?

If the source span does not support the claim, the model must abstain.

### 5. Temporal Episode Extraction

The extractor proposes events, decision clocks, validity periods, supersession,
and stale claims. Policy work breaks when time is flattened, so every temporal
proposal should carry:

- observed time;
- publication time;
- valid-from and valid-until where known;
- status;
- supersedes and superseded-by links;
- source spans.

### 6. Graph Construction

The graph is built from proposals, not invented as a decorative diagram.

Graph records should include:

- nodes for source spans, claims, actors, institutions, events, concepts,
  risks, decisions, reasoning devices, output rules, and review actions;
- edges for support, contradiction, mention, influence, dependency,
  causality, supersession, regulation, reasoning use, review, and output rules;
- graph lenses for stakeholder map, conflict map, issue map, causal map,
  authority map, source map, and decision map;
- confidence, temporal scope, source spans, and review state on material edges.

Every graph edge that affects analysis must have source spans or review actions.

### 7. Reasoning And Intellectual Tool Extraction

DIALECTICA must capture how an expert would approach the issue, not just what
the source says.

For Tool Capsules and Situation Capsules, the extractor proposes:

- reasoning devices;
- step-by-step method;
- source requirements for each step;
- failure modes;
- common traps;
- examples;
- stop conditions;
- escalation rules;
- which graph lens each reasoning step uses.

Example: stakeholder analysis should not only list stakeholders. It should
encode how an expert identifies hidden actors, separates power from salience,
distinguishes stated interests from incentives, maps constraints, detects
missing affected groups, and flags speculative influence claims.

### 8. Human-Gated Language Extraction

Policy teams need precise language. The extractor should propose:

- preferred terms;
- blocked phrases;
- uncertainty wording;
- caveat phrasing;
- audience register;
- multilingual notes;
- citation style;
- organization-specific style rules.

Language rules should be human-gated because wording can change policy meaning,
political sensitivity, legal exposure, and trust.

### 9. Review Router

The review router decides what needs human review.

Always gate:

- unsourced material claims;
- causal claims;
- graph edges that imply influence, responsibility, authority, or causality;
- legal, regulatory, fiscal, or security-sensitive claims;
- private or user-specific profile facts;
- language rules;
- rights rules;
- tool reasoning steps that represent expert method;
- low-confidence proposals;
- contradictions between sources;
- stale or superseded claims that might still affect output;
- any proposal used in a promoted PRAXIS context pack.

Can be auto-accepted only after deterministic validation:

- source file hash records;
- span locators;
- non-material metadata;
- rejected duplicate candidates;
- formatting-normalized records that do not change meaning.

## Conflict Situation Example

For a conflict Situation Capsule, the extractor should build:

- source ledger: agreements, statements, incident reports, economic data,
  stakeholder notes, expert memos;
- claim ledger: atomic facts and contested assertions;
- temporal ledger: triggering events, ceasefires, negotiations, deadlines,
  stale claims, observed versus published dates;
- ontology: actors, institutions, armed groups, mediators, constituencies,
  constraints, grievances, incentives, instruments, risks, red lines;
- graph: actor relationships, influence channels, dependencies,
  contradictions, escalation pathways, source support, decision points;
- reasoning layer: conflict mapping, stakeholder analysis, scenario branching,
  incentive analysis, sourceability check, escalation trigger analysis;
- language layer: sensitive labels, uncertainty language, audience register;
- review layer: which claims and edges were expert-approved, caveated, rejected,
  or expired;
- runtime layer: what PRAXIS may do, how it must cite, when it must stop, and
  when it must request human review.

## Quality Gates

Before a capsule can be promoted:

1. Every material claim has source spans or review actions.
2. Every graph edge that influences reasoning has source spans or review
   actions.
3. Every temporal claim carries date status.
4. Every ontology term has scope, language, and source/review grounding.
5. Every reasoning device has steps, source requirements, and failure modes.
6. Every language rule is reviewed.
7. Every rights rule is explicit.
8. Every context-pack item has a provenance receipt.
9. The compiler can reproduce the same output from the same records.
10. The eval harness shows source fidelity and temporal correctness.

## Build Order

1. Add source-pack and proposal schemas in Rust.
2. Add a local fixture with documents, source spans, extraction proposals, and
   review decisions.
3. Add deterministic proposal-to-canonical normalization.
4. Add review-trigger routing.
5. Add deeper v3 validators for cross-layer references.
6. Add deterministic compiler and `.capsule` archive writer.
7. Add PRAXIS context-pack export.
8. Add local API routes.
9. Add PostgreSQL migrations.
10. Add model-provider integration behind provider traits.
11. Add document/PDF/conversation ingestion.
12. Add PRAXIS frontend integration.

## Non-Goals For The First Build

- Do not let LLMs write canonical records directly.
- Do not add a required graph database.
- Do not require Kubernetes.
- Do not add autonomous memory promotion.
- Do not fine-tune a model before the proposal/eval dataset exists.
- Do not wire PRAXIS production calls before local fixture build, validation,
  inspection, context-pack export, and API preview work.

# Ontology Blueprints

Date: 2026-06-08

Status: active contract for capsule-specific semantic planning.

## Purpose

DIALECTICA must not force every capsule into one actor-claim ontology. Actor,
claim, event, risk, stakeholder, source-proof, and scenario structures are
essential for many Situation Capsules, but they are only one family of meaning.
A User Capsule, Tool Capsule, and Output Capsule need different semantic
layers.

Every capsule needs its own ontology blueprint: a concise plan for what meaning
must be captured, what evidence is required, what reasoning lenses apply, and
how PRAXIS should use the capsule in an agentic workflow.

## Core Rule

The PRAXIS-importable capsule classes are fixed:

- `user_capsule`
- `situation_capsule`
- `tool_capsule`
- `output_capsule`

The embedded graph registry gives shared interoperability classes. It is not
the whole ontology. Specialized source, domain, stakeholder, scenario, expert,
and graph concepts are created as capsule-specific semantic layers, graph
lenses, local terms, and review metadata inside the four classes.

```text
capsule class + workflow + source pack + expert review
  -> ontology blueprint
  -> ontology_slice.json
  -> graph_slice.json
  -> reasoning_playbook.json
  -> language_profile.json
  -> agent_guidance.json
  -> PRAXIS context pack
```

The blueprint is generated before or during capsule build so an LLM, human
reviewer, or compiler can decide which semantic layers matter for this specific
capsule.

## Executable Tool

The first callable tool is local:

```powershell
cargo run -p dialectica-cli -- ontology-plan fixtures/golden-policy-capsule/expected-bundle
```

It loads a bundle and returns a JSON `CapsuleOntologyBlueprint` with:

- `ontology_family`;
- `semantic_layers`;
- `suggested_node_classes`;
- `suggested_edge_classes`;
- `reasoning_lenses`;
- `extraction_questions`;
- `praxis_context_guidance`;
- `review_gates`.

Current implementation note: the blueprint is generated from the manifest or
bundle and exported as `ontology_blueprint.schema.json`. It is not yet a
required signed bundle file. The deterministic compiler should later decide
whether to persist `ontology_blueprint.json` or store it as a compile receipt.

## Four Ontology Families

| Capsule class | Ontology family | Primary semantic concern |
| --- | --- | --- |
| User Capsule | `user_context_ontology` | role, preferences, authority boundaries, privacy, output style, organization context |
| Situation Capsule | `situation_policy_ontology` | sources, actors, claims, risks, events, domain meaning, stakeholder power, causal hypotheses, temporal state |
| Tool Capsule | `tool_method_ontology` | intellectual tool, method steps, input requirements, failure modes, expert caveats, philosophical lens |
| Output Capsule | `output_trace_ontology` | artifact sections, claim lineage, reuse rules, source receipts, caveats, review state |

## Situation Capsule Semantic Layers

A conflict or policy Situation Capsule can carry many specialized layers while
remaining one `situation_capsule`:

| Layer | Captures | Why PRAXIS needs it |
| --- | --- | --- |
| `source_proof_layer` | source spans, claim support, contradictions, trust state, scope of proof | prevents unsupported synthesis |
| `actor_claim_temporal_graph` | actors, institutions, claims, events, risks, decisions, valid-time state | keeps the situation inspectable and current |
| `domain_semantic_layer` | local terms, authorities, instruments, frames, contested meanings | lets PRAXIS understand the issue's vocabulary |
| `stakeholder_power_layer` | incentives, constraints, influence, legitimacy, missing affected groups | supports stakeholder analysis and conflict mapping |
| `scenario_causality_layer` | causal hypotheses, assumptions, indicators, branches, triggers | separates current facts from possible futures |
| `policy_instrument_layer` | instruments, authority, implementation constraints, tradeoffs | supports feasibility and policy design |
| `risk_and_decision_layer` | uncertainty, escalation triggers, decision clocks, review risks | tells PRAXIS when to caveat or hand off |

For example, a conflict capsule may need actors, armed groups, agencies,
international organizations, field reports, negotiations, ceasefire events,
jurisdictional labels, humanitarian constraints, confidence levels, and
language rules. Those are all internal ontology objects and graph lenses of the
Situation Capsule.

## Tool Capsule Semantic Layers

A Tool Capsule captures how experts think through a class of work. It may model:

- required inputs;
- method sequence;
- source standards;
- epistemic stance;
- philosophical or legal distinctions;
- failure modes and anti-patterns;
- output hooks;
- review criteria;
- examples and counterexamples.

Examples include stakeholder analysis, conflict mapping, ACH, red-team review,
legitimacy analysis, proportionality analysis, decision-clock analysis,
feasibility analysis, sourceability audit, and narrative-risk review.

Tool Capsules should guide reasoning without inventing situation facts. A Tool
Capsule becomes powerful when paired with a Situation Capsule.

## User And Output Ontology Layers

User Capsules can model:

- person, team, or organization identity;
- mandate and authority boundary;
- confidentiality and privacy rules;
- preferred artifacts and review habits;
- language and register preferences;
- institutional memory that is approved for this user context.

Output Capsules can model:

- artifact sections;
- claims and supporting sources;
- methods used;
- reviewer caveats;
- reuse scope;
- update requirements;
- audience and language constraints;
- downstream handoff rules.

## Universal Layers

All capsule ontologies must carry these safeguards:

- sourceability layer: source ids, source spans, hashes, review actions;
- temporal validity layer: current, stale, superseded, forecast, contested, or
  unknown status;
- language semantics layer: approved terms, deprecated terms, blocked phrases,
  caveats, translation notes, register;
- review and rights layer: human gates, caveats, expiry, permissions,
  prohibitions, duties;
- PRAXIS agent guidance layer: allowed workflows, tool policy, citation policy,
  stop conditions, handoff rules.

These universal layers do not replace the local ontology. The local ontology is
what tells PRAXIS that a capsule is about a user's authority boundary, a live
conflict, a stakeholder-analysis method, or a memo's reuse lineage.

## LLM Build Guidance

When an LLM helps build a capsule, it should use the ontology blueprint before
extracting or drafting:

1. Identify one of the four capsule classes and intended PRAXIS workflows.
2. Select the ontology family and capsule-specific semantic layers.
3. Ask extraction questions from the blueprint.
4. Propose ontology terms, graph nodes, graph edges, reasoning lenses, and
   language rules.
5. Attach source spans or review actions to every material object.
6. Mark missing evidence as a gap, not a generated fact.
7. Stop or hand off when review gates fail.

The ontology blueprint is therefore both a planning tool for the LLM and a
contract for the compiler.

## Standards Posture

The blueprint should stay JSON-first and standards-shaped:

- SKOS-shaped concepts for labels, synonyms, broader/narrower terms, notes, and
  concept-scheme mappings;
- PROV-O-shaped lineage for source, extraction, review, compile, and export
  events;
- SHACL-shaped constraints for required graph classes, properties, and review
  rules;
- ODRL-shaped rights for permissions, prohibitions, duties, and sharing
  constraints;
- JSON-LD export for linked-data interoperability;
- OWL/RDF inference only when a downstream adapter needs formal semantic
  reasoning.

This preserves a path to serious semantic systems without making the first
capsule engine depend on a full RDF stack.

## PRAXIS Use

When PRAXIS loads one or more capsules, it should use each capsule's ontology
blueprint to decide:

- which semantic layers enter context;
- which graph lens to display;
- which Tool Capsule reasoning devices to apply first;
- which language rules govern output;
- which stale, rejected, unsupported, or rights-blocked objects must be hidden
  or warned;
- which review gates require human handoff.

This is how DIALECTICA hands PRAXIS a meaningful knowledge object rather than a
flat bundle of text.

## Source Anchors

- SKOS: <https://www.w3.org/TR/skos-reference/>
- PROV-O: <https://www.w3.org/TR/prov-o/>
- SHACL: <https://www.w3.org/TR/shacl/>
- JSON-LD 1.1: <https://www.w3.org/TR/json-ld11/>
- ODRL: <https://www.w3.org/TR/odrl-model/>
- OWL 2 overview: <https://www.w3.org/TR/owl2-overview/>
- RDF 1.2 concepts: <https://www.w3.org/TR/rdf12-concepts/>

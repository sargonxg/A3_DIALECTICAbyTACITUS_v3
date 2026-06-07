# Ontology Blueprints

Date: 2026-06-07

Status: active contract for capsule-specific semantic planning.

## Purpose

DIALECTICA must not force every capsule into one actor-claim ontology. Actor,
claim, event, risk, and decision graphs are appropriate for a situation
capsule, but they are only one profile.

Every capsule needs its own ontology blueprint: a concise plan for what meaning
must be captured, what evidence is required, what reasoning lenses apply, and
how PRAXIS should use the capsule in an agentic workflow.

## Core Rule

The embedded graph registry gives shared interoperability classes. It is not the
whole ontology.

```text
capsule type + source pack + expert review + intended PRAXIS workflow
  -> ontology blueprint
  -> ontology_slice.json
  -> graph_slice.json
  -> reasoning_playbook.json
  -> language_profile.json
  -> agent_guidance.json
  -> PRAXIS context pack
```

The blueprint is generated before or during capsule build so an LLM, human
reviewer, or compiler can decide which semantic layers matter for this
specific capsule.

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

This is deliberately generic enough to support user capsules, situation
capsules, thinking-device capsules, output capsules, source capsules, domain
capsules, expert-pick capsules, and future capsule categories.

Current implementation note: the blueprint is generated from the manifest or
bundle and exported as `ontology_blueprint.schema.json`. It is not yet a
required signed bundle file. The deterministic compiler should later decide
whether to persist `ontology_blueprint.json` or store it as a compile receipt.

## Capsule-Specific Ontology Families

| Capsule type | Ontology family | Primary semantic concern |
| --- | --- | --- |
| User Capsule | `user_context_ontology` | role, preferences, authority boundaries, privacy, output style |
| Team Capsule | `team_memory_ontology` | mandate, shared sources, output standards, review authority |
| Situation Capsule | `situation_policy_ontology` | actors, claims, risks, events, decision clocks, temporal state |
| Source Capsule | `source_proof_ontology` | what the source supports, contradicts, qualifies, or does not prove |
| Domain Capsule | `domain_semantic_ontology` | concepts, authorities, instruments, frames, synonyms, mappings |
| Thinking Device or Tool Capsule | `expert_method_ontology` | method steps, required inputs, failure modes, review caveats |
| Stakeholder Capsule | `stakeholder_power_ontology` | incentives, constraints, influence, legitimacy, missing actors |
| Scenario Capsule | `scenario_causality_ontology` | assumptions, indicators, causal hypotheses, branches, triggers |
| Output Capsule | `output_trace_ontology` | artifact sections, claim lineage, reuse rules, source receipts |
| Expert Pick Capsule | `expert_trust_ontology` | reviewer judgment, caveats, freshness, rights, marketplace trust |
| Graph/Ontology Capsule | `semantic_module_ontology` | reusable terms, aliases, mappings, constraints, compatibility |
| Unknown or new capsule type | `capsule_specific_ontology` | the local meaning map needed for that capsule's workflows |

## Universal Layers

All capsule ontologies must carry these layers:

- sourceability layer: source ids, source spans, hashes, review actions;
- temporal validity layer: current, stale, superseded, forecast, contested, or
  unknown status;
- language semantics layer: approved terms, deprecated terms, blocked phrases,
  caveats, translation notes, register;
- review and rights layer: human gates, caveats, expiry, permissions,
  prohibitions, duties;
- PRAXIS agent guidance layer: allowed workflows, tool policy, citation policy,
  stop conditions, handoff rules.

These universal layers are safeguards. They do not replace the local ontology.
The local ontology is what tells PRAXIS that a capsule is about a user's
authority boundary, a budget-rule interpretation, a stakeholder map, a source
pack, a reasoning method, or an output lineage.

## Situation Graph Is One Profile

The actor/claim/time graph is valuable for situation and stakeholder work:

```text
source span -> claim -> actor -> risk -> decision
      |          |        |       |
      v          v        v       v
  provenance   time   incentives caveats
```

But a user capsule may instead need:

```text
user role -> authority boundary -> output preference -> language rule
```

A thinking-device capsule may need:

```text
method input -> reasoning step -> failure mode -> output rule
```

An output capsule may need:

```text
artifact section -> claim -> source receipt -> review caveat -> reuse rule
```

The shared graph classes make these profiles interoperable. The ontology
blueprint decides which profile is correct for the capsule.

## LLM Build Guidance

When an LLM helps build a capsule, it should use the ontology blueprint before
extracting or drafting:

1. Identify capsule type and intended PRAXIS workflows.
2. Select ontology family and semantic layers.
3. Ask extraction questions from the blueprint.
4. Propose ontology terms, graph nodes, graph edges, reasoning lenses, and
   language rules.
5. Attach source spans or review actions to every material object.
6. Mark missing evidence as a gap, not a generated fact.
7. Stop or hand off when review gates fail.

The ontology blueprint is therefore a planning tool for the LLM and a contract
for the compiler.

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
- OWL/RDF/JSON-LD export only when a downstream adapter needs formal semantic
  interoperability.

This preserves a path to serious semantic systems without making the first
capsule engine depend on a full RDF stack.

## PRAXIS Use

When PRAXIS loads one or more capsules, it should use each capsule's ontology
blueprint to decide:

- which semantic layers enter context;
- which graph lens to display;
- which reasoning devices to apply first;
- which language rules govern output;
- which stale, rejected, or rights-blocked objects must be hidden or warned;
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

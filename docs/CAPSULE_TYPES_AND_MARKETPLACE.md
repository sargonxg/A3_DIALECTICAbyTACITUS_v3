# Capsule Types And Marketplace

## Purpose

DIALECTICA should produce capsules that PRAXIS can load, visualize, combine,
inspect, and trust. A capsule is not only a document summary. It is a portable
unit of analytical context with evidence, time, ontology, graph structure,
reasoning tools, output rules, review history, and usage rights.

```text
       TACITUS / DIALECTICA
  +--------------------------------+
  | source -> claim -> time        |
  |              \                 |
  |               graph -> review  |
  |                  \             |
  |                capsule -> PRAXIS
  +--------------------------------+
```

The marketable unit is **PRAXIS Capsule**. The internal engine is
**DIALECTICA**.

## Capsule Type System

Capsules should be typed because policy teams do not need one generic blob.
They need composable context objects with predictable guarantees.

| Type | Main question answered | Required layers | Typical PRAXIS use |
| --- | --- | --- | --- |
| User Capsule | Who is the user, what do they know, and how do they work? | identity, preferences, prior outputs, review rules | personalized Ask PRAXIS, writing handover |
| Team Capsule | What is the team mandate, workflow, and institutional memory? | identity, authorities, shared sources, output standards | team briefs, recurring analysis |
| Situation Capsule | What is happening in this issue right now? | sources, time, actors, claims, risks, graph | policy memos, crisis updates, scenario work |
| Source Capsule | What does this source pack actually say? | source ledger, spans, claims, trust, temporal state | grounded retrieval, citation packs |
| Domain Capsule | What concepts, authorities, and frames define this policy domain? | ontology, semantic layer, institutions, instruments | domain onboarding, expert context |
| Thinking Device Capsule | How should an expert reason through this class of problem? | reasoning playbook, method steps, failure modes, examples | structured analysis, red teams |
| Stakeholder Capsule | Who matters, what do they want, and how can they act? | actors, incentives, constraints, graph, uncertainty | stakeholder maps, negotiation planning |
| Scenario Capsule | What futures are plausible and what indicators matter? | temporal layer, causal hypotheses, signals, branches | foresight, contingency planning |
| Output Capsule | What artifact was produced and how should it be reused? | output contract, source trail, caveats, quality checks | memo reuse, brief updates, handover |
| Expert Pick Capsule | What has a trusted expert approved, caveated, or recommended? | review ledger, reviewer credentials, usage rights | marketplace discovery, high-trust workflows |
| Graph/Ontology Capsule | What reusable semantic model should other capsules inherit? | ontology, graph schema, term mappings, SHACL-like constraints | visualization, cross-capsule reasoning |

Capsules can be combined when their usage contracts, source policies, ontology
versions, and review states are compatible. PRAXIS should warn when combining
capsules with conflicting claims, incompatible rights, stale temporal state, or
different reviewer standards.

## Type Contract

Every capsule type declares:

- `capsule_type`: one of the approved type names;
- `scope`: issue, geography, time horizon, institution, and user boundary;
- `allowed_workflows`: PRAXIS workflows where the capsule may be used;
- `required_layers`: bundle files that must be present;
- `graph_profile`: node and edge classes required for visualization;
- `reasoning_profile`: required thinking devices;
- `review_profile`: reviewer roles and promotion rules;
- `freshness_profile`: staleness and recertification rules;
- `rights_profile`: permitted use, prohibited use, and sharing policy;
- `compatibility_profile`: what other capsule types it may combine with.

Example:

```json
{
  "capsule_type": "stakeholder_capsule",
  "scope": {
    "issue": "industrial electricity price support",
    "geography": "EU",
    "valid_from": "2026-06-01",
    "valid_until": "2026-09-30"
  },
  "required_layers": [
    "source_ledger",
    "temporal_ledger",
    "ontology_slice",
    "graph_slice",
    "reasoning_playbook",
    "review_ledger"
  ],
  "graph_profile": "stakeholder_graph_v1",
  "reasoning_profile": "stakeholder_analysis_v1",
  "review_profile": "expert_review_required",
  "rights_profile": "team_internal_reviewed"
}
```

## Graph Profile Matrix

Use [Graph Profile Registry](GRAPH_PROFILE_REGISTRY.md) as the canonical
implementation source for per-type graph profiles. Summary:

| Capsule type | Graph profile | PRAXIS lens |
| --- | --- | --- |
| User Capsule | `user_context_graph_v1` | user context |
| Team Capsule | `team_memory_graph_v1` | team memory |
| Situation Capsule | `situation_graph_v1` | situation map |
| Source Capsule | `source_proof_graph_v1` | source proof |
| Domain Capsule | `domain_ontology_graph_v1` | ontology explorer |
| Thinking Device Capsule | `reasoning_device_graph_v1` | method trace |
| Stakeholder Capsule | `stakeholder_graph_v1` | stakeholder map |
| Scenario Capsule | `scenario_graph_v1` | scenario tree |
| Output Capsule | `output_trace_graph_v1` | artifact trace |
| Expert Pick Capsule | `expert_pick_graph_v1` | trust receipt |

## Capsule Marketplace Vision

The marketplace is not a prompt marketplace. It is a trusted context market for
expert-reviewed analytical objects.

Marketplace capsules should be:

- discoverable by topic, institution, region, source base, expert, and output
  type;
- inspectable before use, including graph preview, source counts, review state,
  freshness, and caveats;
- forkable into team-private variants while retaining lineage;
- versioned with semantic compatibility rules;
- signed and checksummed;
- review-gated before promotion;
- governed by explicit usage rights;
- evaluated against task-specific baselines.

## Marketplace Trust Levels

| Level | Meaning | PRAXIS behavior |
| --- | --- | --- |
| Draft | Machine-generated or author-created, not reviewed. | visible only with warning; cannot silently ground high-stakes outputs |
| Reviewed | Human reviewer approved the declared scope. | usable in normal workflows with review receipt |
| Expert Pick | Named expert or institution recommends it for a class of work. | highlighted in library and carries expert rationale |
| Certified | Passed stricter source, security, legal, and eval gates. | allowed for sensitive team workflows |
| Deprecated | Superseded, stale, or withdrawn. | retained for lineage; blocked by default |

## Why This Matters For Expert Reasoning

Policy expertise is often tacit. Experts know which sources matter, which terms
are overloaded, which institutional constraints are real, what a misleading
statistic looks like, and when a causal story is too clean. DIALECTICA captures
that tacit layer as structured reasoning devices, review notes, caveats, and
failure modes.

The goal is not to make an LLM impersonate an expert. The goal is to provide
PRAXIS with enough reviewed context that an agent can reason under expert-like
constraints:

- cite the right source class;
- ask the right policy question;
- separate legal authority from political feasibility;
- distinguish live claims from stale claims;
- surface missing actors and incentives;
- avoid outputs the reviewer has forbidden;
- explain why a method was chosen.

## Marketplace Object Model

```text
CapsuleListing
  id
  capsule_id
  title
  capsule_type
  domain_tags
  geography
  language
  source_count
  review_level
  reviewer_summary
  freshness_status
  usage_rights
  compatible_capsules[]
  fork_policy
  latest_version
```

The listing is only an index. The signed capsule bundle remains the product
artifact.

## Scaling Categories

Start with a few policy-native categories, then let the marketplace expand
through ontology tags:

- economic policy;
- fiscal and budget analysis;
- industrial policy;
- energy and climate;
- security and defense;
- health policy;
- technology regulation;
- electoral and political analysis;
- trade and sanctions;
- institutional reform;
- crisis response;
- legislative analysis;
- procurement and implementation;
- narrative and communications risk.

Each category should have reusable ontology modules, source standards,
thinking-device templates, and eval fixtures.

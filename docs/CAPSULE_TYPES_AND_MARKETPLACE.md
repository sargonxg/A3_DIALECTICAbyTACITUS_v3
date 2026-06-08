# Capsule Types And Marketplace

## Purpose

DIALECTICA produces PRAXIS Capsules: portable context objects that humans and
agents can inspect, combine, improve, review, and reuse. A capsule is not a
summary and not a prompt. It is a signed knowledge-work unit with evidence,
time, ontology, embedded graph structure, reasoning tools, output rules, human
review, and usage rights.

The PRAXIS-facing contract has exactly four top-level capsule categories:

```text
                 DIALECTICA CONTEXT ENGINE

      +-----------+      +--------------+
      |   USER    |      |  SITUATION   |
      | who thinks|      | what is true |
      +-----+-----+      +------+-------+
            \                  /
             \                /
              v              v
        +--------------------------------+
        |       PRAXIS CONTEXT PACK      |
        | source/time/ontology/graph     |
        | reasoning/language/review      |
        +--------------------------------+
              ^              ^
             /                \
            /                  \
      +-----+-----+      +------+-------+
      |   TOOL    |      |   OUTPUT     |
      | how to    |      | what was     |
      | reason    |      | produced     |
      +-----------+      +--------------+
```

This segmentation mirrors how analysts build real context:

- **User**: who is doing the work, under what mandate, with what preferences,
  expertise, language, permissions, and privacy boundaries.
- **Situation**: what is happening, what is sourced, what is stale or disputed,
  who matters, what concepts define the issue, what caveats and causal claims
  must travel with the context.
- **Tool**: how to think through the work, including stakeholder analysis,
  conflict mapping, decision clocks, ACH, red-team checks, feasibility lenses,
  philosophical distinctions, and failure modes.
- **Output**: what was produced, why it was produced, what it cites, what can be
  reused, and which caveats or review gates control reuse.

Other names such as source pack, stakeholder map, scenario branch, domain
ontology, expert pick, and graph module are internal layers, lenses, modules, or
marketplace metadata inside one of the four capsule classes. They are not
PRAXIS-importable top-level capsule types.

## Four Capsule Classes

| Capsule class | Question answered | Ontology family | Default graph profile | Typical PRAXIS use |
| --- | --- | --- | --- | --- |
| User Capsule | Who is this analyst, team member, or organization in this workflow? | `user_context_ontology` | `user_context_graph_v1` | personalization, authority boundaries, style, privacy, handover |
| Situation Capsule | What is the state of the policy, conflict, market, institution, or research matter? | `situation_policy_ontology` | `situation_graph_v1` | source-grounded analysis, conflict map, stakeholder map, situation brief |
| Tool Capsule | Which reviewed intellectual tool should structure the reasoning? | `tool_method_ontology` | `tool_method_graph_v1` | stakeholder analysis, conflict mapping, ACH, red team, feasibility checks |
| Output Capsule | What artifact exists and how can it be reused or updated? | `output_trace_ontology` | `output_trace_graph_v1` | memo reuse, brief update, decision log, audit handover |

The four classes share the same bundle structure. Their difference is the
ontology blueprint and the semantic layers each capsule must build.

## Internal Lenses, Not Capsule Types

Specialized knowledge still matters. It is captured as internal structure:

| Internal concept | Lives inside | Why |
| --- | --- | --- |
| Source pack / source proof | Situation, Tool, Output | sources are evidence for a context object, not a separate PRAXIS context class |
| Domain ontology | Situation or Tool | every matter may need its own terms, authorities, instruments, and mappings |
| Stakeholder map | Situation or Tool | actors and incentives are a lens over a situation or a method for analysis |
| Scenario branch | Situation or Output | futures belong to a situation graph or to an artifact's reasoning lineage |
| Expert pick | Marketplace listing and review ledger | expert recommendation is trust metadata, not a new bundle shape |
| Graph/ontology module | Ontology blueprint and graph constraints | reusable semantics are imported into the capsule-specific model |

This keeps the product simple while preserving analytical depth. PRAXIS can ask
for "the Situation Capsule for this conflict" and still visualize a stakeholder
lens, source-proof lens, scenario lens, and ontology lens inside that capsule.

## Type Contract

Every PRAXIS-importable capsule declares:

- `capsule_type`: one of `user_capsule`, `situation_capsule`, `tool_capsule`,
  or `output_capsule`;
- `scope`: issue, geography, time horizon, institution, user/team boundary, and
  intended workflow;
- `ontology_family`: the semantic family selected by the ontology blueprint;
- `graph_profile`: the embedded graph profile and any local graph lenses;
- `required_layers`: source ledger, temporal ledger, ontology slice, graph
  slice, reasoning playbook, language profile, agent guidance, review ledger,
  rights profile, health, eval, and signature;
- `allowed_workflows`: PRAXIS workflows where the capsule may be used;
- `reasoning_profile`: tools or methods that must sequence the agent's work;
- `review_profile`: reviewer roles, caveats, expiry, promotion rules;
- `freshness_profile`: staleness, supersession, and recertification rules;
- `rights_profile`: permissions, prohibitions, duties, and sharing policy;
- `compatibility_profile`: which other capsule classes may be composed.

Example:

```json
{
  "capsule_type": "situation_capsule",
  "scope": {
    "issue": "border-region electricity infrastructure conflict",
    "geography": "synthetic fixture region",
    "valid_from": "2026-06-01",
    "valid_until": "2026-09-30"
  },
  "required_layers": [
    "source_ledger",
    "temporal_ledger",
    "ontology_blueprint",
    "ontology_slice",
    "graph_slice",
    "reasoning_playbook",
    "language_profile",
    "agent_guidance",
    "review_ledger"
  ],
  "graph_profile": "situation_graph_v1",
  "graph_lenses": [
    "source_proof_lens",
    "stakeholder_power_lens",
    "scenario_causality_lens",
    "domain_semantic_lens"
  ],
  "reasoning_profile": "conflict_mapping_v1",
  "review_profile": "expert_review_required",
  "rights_profile": "team_internal_reviewed"
}
```

## Embedded Graph

Each capsule contains an embedded graph in the signed bundle. The graph is
canonical at the bundle layer; graph databases are adapters.

```text
Capsule
  |
  +-- ontology blueprint        what meaning must be captured
  +-- ontology slice            terms, frames, mappings, caveats
  +-- graph slice               nodes, edges, communities, review state
  +-- graph semantics           JSON-LD / PROV-O / SKOS / ODRL export view
  +-- graph constraints         validation rules and allowed lenses
```

LadybugDB, Neo4j, Oxigraph, PostgreSQL, Graphiti, or GraphRAG systems may be
used as projections for exploration, algorithms, temporal graph memory, or
visualization. They do not replace the capsule contract. The bundle must remain
portable as JSON/JSONL plus checksums and review receipts.

## Example: Conflict Situation Capsule

A conflict Situation Capsule can include:

- source proof for official documents, interviews, field notes, media, datasets,
  and expert notes;
- temporal ledger for incidents, negotiations, legal deadlines, stale claims,
  superseded facts, and contested assertions;
- domain semantic layer for legal terms, institutional authority, local labels,
  geography, treaty concepts, and policy instruments;
- stakeholder power lens for actors, coalitions, incentives, constraints,
  legitimacy, affected groups, veto points, and missing voices;
- scenario causality lens for triggers, assumptions, indicators, escalation
  paths, de-escalation opportunities, and warning signals;
- reasoning links to Tool Capsules such as conflict mapping, stakeholder
  analysis, decision-clock analysis, or red-team review;
- language rules for uncertainty, contested claims, identity references,
  jurisdictional caveats, and terms that require careful translation.

When PRAXIS loads this capsule, an agent does not receive a flat document dump.
It receives a structured context pack: what is known, how it is known, when it
was valid, what it means, what is uncertain, how experts would reason, what
must be cited, and where a human must approve.

## Tool Capsules

Tool Capsules capture intellectual tools, not software tools. Examples:

- stakeholder analysis;
- conflict mapping;
- ACH / competing hypotheses;
- decision-clock analysis;
- political feasibility analysis;
- sourceability and contradiction audit;
- red-team or premortem method;
- philosophical lenses such as legitimacy, harm, proportionality, or agency.

A Tool Capsule should specify required inputs, reasoning steps, output hooks,
failure modes, evidence requirements, examples, language caveats, and review
gates. It can be combined with any Situation Capsule when compatibility rules
allow it.

## Marketplace Vision

The marketplace is not a prompt marketplace. It is a trusted context market for
expert-reviewed analytical objects.

Marketplace listings index four capsule classes. Expert picks, certifications,
and reviewer endorsements are listing metadata and review-ledger state.

Marketplace capsules should be:

- discoverable by topic, institution, region, source base, expert, output type,
  workflow, and capsule class;
- inspectable before use, including graph preview, source counts, review state,
  freshness, caveats, ontology lenses, and rights;
- forkable into team-private variants while retaining lineage;
- versioned with semantic compatibility rules;
- signed and checksummed;
- review-gated before promotion;
- governed by explicit usage rights;
- evaluated against task-specific baselines.

## Marketplace Trust Levels

| Level | Meaning | PRAXIS behavior |
| --- | --- | --- |
| Draft | Machine-proposed or author-created, not reviewed. | visible only with warning; cannot silently ground sensitive outputs |
| Reviewed | Human reviewer approved the declared scope. | usable in normal workflows with review receipt |
| Expert Pick | Named expert or institution recommends it for a class of work. | highlighted in library and carries expert rationale |
| Certified | Passed stricter source, security, legal, and eval gates. | allowed for sensitive team workflows |
| Deprecated | Superseded, stale, or withdrawn. | retained for lineage; blocked by default |

## Why This Helps Human And Agent Work

Human analysts naturally ask four questions: who is working, what is the
situation, how should we think, and what are we producing. DIALECTICA encodes
those questions into reusable, inspectable artifacts. PRAXIS can then combine
capsules without losing sourceability, temporality, meaning, reasoning, review,
or language discipline.

This is how PRAXIS Augmented Generation by TACITUS differs from ordinary LLM
generation: the model is not asked to invent context from a chat transcript. It
is given a reviewed context backbone that humans and agents can share.

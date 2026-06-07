# Expert Review And Marketplace

## Purpose

DIALECTICA should make human review a data model, not a comment thread. The
review layer is what turns machine-extracted knowledge into a capsule that
policy teams can trust, share, fork, and reuse.

```text
machine proposal -> expert inspection -> caveat/reject/approve
       |                    |                    |
       v                    v                    v
  draft graph        review ledger        promoted capsule
```

## Review Objects

Review can apply to the whole capsule or to individual objects:

- source trust decisions;
- source spans;
- extracted claims;
- temporal facts;
- ontology terms;
- graph nodes and edges;
- reasoning-device outputs;
- output contracts;
- rights and sharing policies;
- final bundle promotion.

The review ledger must say exactly what was reviewed, by whom, under which
scope, when the decision expires, and what caveats apply.

## Reviewer Roles

| Role | What they review | Example decision |
| --- | --- | --- |
| Source Reviewer | source quality, access rights, provenance | "Use this dataset, but mark as partial coverage." |
| Domain Expert | terminology, actors, policy instruments | "This agency is a regulator, not an implementer." |
| Methods Reviewer | reasoning device and analytic method | "Use ACH because evidence conflicts across sources." |
| Legal/Compliance Reviewer | rights, privacy, regulated material | "Do not export interview notes outside tenant." |
| Red-Team Reviewer | blind spots, misuse, adversarial framing | "Actor incentives omit organized labor." |
| Final Promoter | bundle promotion and marketplace listing | "Approved with caveat until the next budget update." |

## Review States

```text
draft
  -> machine_proposed
  -> needs_review
  -> approved
  -> approved_with_caveats
  -> rejected
  -> superseded
  -> promoted
```

Rejected and superseded objects remain in lineage. They are not silently
deleted, because future reviewers need to know what the engine considered and
why it was blocked.

## Promotion Gates

A capsule cannot be promoted until it passes required gates:

1. Source gate: required source classes are present and access rights are known.
2. Provenance gate: factual claims and graph edges trace back to source spans or
   expert notes.
3. Temporal gate: time-sensitive claims have valid time, publication time, and
   freshness state.
4. Ontology gate: core terms map to approved ontology terms or are flagged as
   local.
5. Reasoning gate: required thinking devices have method outputs and failure
   modes.
6. Rights gate: usage, sharing, and export rules are explicit.
7. Evaluation gate: capsule-augmented output is compared against baseline
   output for the declared workflow.
8. Human promotion gate: a reviewer accepts the declared scope.

## Review Ledger Shape

```json
{
  "review_id": "rev_2026_06_07_domain_001",
  "capsule_id": "cap_eu_energy_stakeholders_2026_q3",
  "reviewer_id": "expert:energy-policy:123",
  "reviewer_role": "domain_expert",
  "reviewed_object_type": "graph_edge",
  "reviewed_object_id": "edge:commission-influences-state-aid-guidelines",
  "decision": "approved_with_caveats",
  "scope": {
    "geography": "EU",
    "valid_until": "2026-09-30",
    "workflows": ["stakeholder_map", "decision_brief"]
  },
  "notes": "Valid for current policy debate; recertify after new guidance.",
  "created_at": "2026-06-07T15:00:00Z",
  "expires_at": "2026-09-30T23:59:59Z"
}
```

## Expert Reasoning Capture

Experts should not only click approve. DIALECTICA should capture what made the
expert decision useful:

- what source hierarchy they applied;
- what actors they expected a novice to miss;
- what term definitions changed the analysis;
- what causal story they rejected;
- what missing evidence would change their mind;
- what output pattern they recommend;
- what red flags PRAXIS should surface to users.

This becomes the capsule's reasoning layer. PRAXIS can then guide an AI agent
to reason with expert constraints instead of simply retrieving expert facts.

## Marketplace Listing Review

Marketplace review is stricter than team-private review. A listed capsule
should include:

- summary of scope;
- source base;
- data rights;
- reviewer role and credential type;
- review level;
- freshness and expiry;
- compatibility notes;
- known caveats;
- sample PRAXIS workflows;
- evaluation snapshot;
- lineage and fork policy.

## Forking And Lineage

Teams must be able to fork a capsule without losing trust history.

```text
Expert Capsule v1
       |
       +-- Team Fork A: local sources + private notes
       |
       +-- Team Fork B: translated + narrowed geography
```

Each fork keeps:

- parent capsule id;
- parent bundle digest;
- inherited review objects;
- local modifications;
- new review requirements;
- export rights.

Inherited expert review does not automatically approve local changes.

## Marketplace Safety

Capsules can influence high-stakes work, so the marketplace must prevent:

- stale capsules appearing current;
- private sources leaking through public listings;
- expert identity being implied where no review exists;
- unsupported graph edges appearing authoritative;
- incompatible capsules being combined silently;
- model-generated reasoning devices being sold as expert method;
- outputs being used outside declared scope.

## Evaluation For Trust

Marketplace trust should include measured behavior:

- citation fidelity;
- source coverage;
- temporal correctness;
- unsupported-claim rate;
- contradiction handling;
- reasoning-device adherence;
- human reviewer agreement;
- PRAXIS output improvement for declared workflow.

The marketplace should prefer capsules that improve work under tests, not only
capsules with polished descriptions.

# Intellectual Tools

## Purpose

DIALECTICA should capture how experts think, not only what they know.

In policy work, the same sources can produce weak or strong outputs depending on
which mental tools are applied. A capsule should therefore carry explicit
reasoning devices that PRAXIS agents can use, cite, and explain.

## What Counts As An Intellectual Tool

An intellectual tool is a reusable reasoning device with:

- a name;
- a purpose;
- required inputs;
- method steps;
- failure modes;
- source needs;
- output expectations;
- review criteria.

## Core Tool Families

### Sourceability

Purpose: prevent unsupported claims.

Captured as:

- required source types;
- citation density;
- source hierarchy;
- unsupported claim policy;
- source gap questions.

### Temporal Reasoning

Purpose: keep the analysis aware of time, freshness, sequencing, and deadlines.

Captured as:

- event timeline;
- validity windows;
- stale claim rules;
- supersession relationships;
- decision clock and review dates.

### Actor and Incentive Mapping

Purpose: explain what actors want, can do, and are likely to resist.

Captured as:

- actors;
- institutions;
- incentives;
- constraints;
- leverage points;
- likely reactions.

### Causal Analysis

Purpose: distinguish correlation, influence, mechanism, and uncertainty.

Captured as:

- causal hypotheses;
- evidence strength;
- alternative explanations;
- necessary conditions;
- second-order effects;
- uncertainty labels.

### Institutional Analysis

Purpose: account for authorities, procedures, legitimacy, capacity, and veto
points.

Captured as:

- authorities;
- mandates;
- procedural constraints;
- capacity limits;
- accountability channels;
- legitimacy concerns.

### Red Teaming

Purpose: find blind spots, adversarial interpretations, and failure modes.

Captured as:

- strongest counterargument;
- misuse risks;
- stakeholder objections;
- evidence weaknesses;
- escalation triggers.

### ACH And Competing Hypotheses

Purpose: compare explanations without prematurely collapsing uncertainty.

Captured as:

- hypotheses;
- discriminating evidence;
- consistency matrix;
- rejected or weakened hypotheses;
- confidence posture.

### Scenario And Path Dependency

Purpose: reason about possible futures and lock-in.

Captured as:

- scenarios;
- triggers;
- branches;
- path dependencies;
- indicators to watch;
- reversible and irreversible choices.

### Normative And Philosophical Lenses

Purpose: make value tradeoffs explicit.

Captured as:

- rights;
- duties;
- legitimacy;
- distributional effects;
- consent;
- precaution;
- proportionality;
- institutional trust.

## Representation In Capsules

Each reasoning device should compile into `reasoning_playbook.json`:

```json
{
  "device_id": "actor_incentive_map",
  "label": "Actor and incentive mapping",
  "purpose": "Explain actor motives, constraints, and likely reactions.",
  "inputs_required": ["actors", "constraints", "source_spans"],
  "method_steps": [
    "List actors and institutions",
    "Map incentives and constraints",
    "Identify leverage and veto points",
    "Record uncertainty and source gaps"
  ],
  "failure_modes": ["mind-reading", "single-actor bias", "missing informal power"],
  "review_gate": "policy_expert",
  "output_hooks": ["stakeholder_map", "decision_brief"]
}
```

## Capture Workflow

1. Detect relevant policy path and output goal.
2. Select candidate reasoning devices.
3. Ask extraction workers for required inputs.
4. Record device outputs with provenance.
5. Mark weak or missing inputs as source gaps.
6. Ask reviewer to approve, caveat, or reject device outputs.
7. Export accepted device guidance in the PRAXIS context pack.

## MVP Device Set

The MVP should implement five devices:

- sourceability check;
- decision clock;
- actor and incentive map;
- causal hypothesis check;
- red-team counterargument.

These cover the minimum policy difference between generic generation and
capsule-augmented generation.

# MVP Definition

## MVP Name

Internal: DIALECTICA v3 Capsule Compiler MVP.

Public PRAXIS language: PRAXIS Capsules.

## One-Sentence Goal

Compile a small, source-grounded, human-reviewable policy context capsule that
PRAXIS can use to produce a better policy artifact than raw LLM generation.

## MVP Demo

The first demo should show this path:

```text
fixture source pack
  -> source ledger
  -> extracted entities, claims, dates, and frames
  -> human review decision
  -> signed capsule bundle
  -> PRAXIS context pack
  -> capsule-augmented policy answer with citations and warnings
```

## Required Capabilities

### 1. Capsule Bundle

The system must create a portable capsule bundle with:

- manifest;
- situation context;
- source ledger;
- temporal ledger;
- ontology slice;
- graph slice;
- reasoning playbook;
- retrieval pack;
- output contracts;
- review ledger;
- checksums.

### 2. Source Grounding

Every factual claim in the capsule must be connected to:

- a source id;
- a source span or locator;
- extraction run metadata;
- trust status.

### 3. Temporal Awareness

The capsule must distinguish:

- event time;
- publication time;
- ingestion time;
- review time;
- current, stale, superseded, contested, forecast, and unknown claims.

### 4. Human Gate

The MVP must support at least one reviewer decision that can block promotion.

Review states:

- draft;
- machine-proposed;
- needs review;
- approved;
- approved with caveats;
- rejected;
- promoted.

### 5. PRAXIS Context Pack

The MVP must export a compact context pack with:

- capsule summary;
- relevant retrieval records;
- citation hints;
- temporal warnings;
- reasoning devices;
- output contract;
- forbidden claims or cautions.

### 6. Evaluation

The MVP must compare:

- raw prompt response;
- loose-document response;
- capsule-augmented response.

The capsule-augmented response should improve citation fidelity, temporal
correctness, and policy reasoning.

## Explicit Non-Goals

The MVP does not need:

- Kubernetes;
- a required graph database;
- a required vector database;
- a public standalone DIALECTICA product surface;
- live ingestion from every source connector;
- autonomous promotion without human review;
- full AGON or KAIROS services;
- enterprise multi-region deployment;
- fine-tuned models.

## Golden Policy Fixture

The first fixture should include:

- five to ten source documents;
- one outdated source;
- one source conflict;
- one actor map;
- one temporal sequence;
- one causal hypothesis;
- one reviewer correction;
- one target output such as a decision brief.

## Definition of Done

The MVP is done when:

1. a local command builds a valid fixture capsule;
2. the capsule validates against schema;
3. source ledger references resolve;
4. review gate can block promotion;
5. PRAXIS can consume the context pack;
6. eval output shows whether the capsule improved the policy answer;
7. staging Cloud Run can compile or serve one fixture capsule.

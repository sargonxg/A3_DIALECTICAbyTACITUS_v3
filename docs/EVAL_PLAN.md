# Evaluation Plan

## Goal

DIALECTICA must prove that PRAXIS Capsules produce better policy work than raw
LLM prompting.

The eval plan measures whether a capsule improves:

- source fidelity;
- temporal accuracy;
- reasoning quality;
- uncertainty handling;
- task completion;
- reviewer trust;
- PRAXIS workflow usefulness.

## Eval Types

### Contract Evals

Check that capsule bundles are structurally valid.

Examples:

- required files exist;
- manifest fields are valid;
- source ledger references resolve;
- graph edges have provenance;
- review gates block unapproved promotion;
- checksums match.

### Source Fidelity Evals

Check that generated claims are supported.

Examples:

- every factual claim maps to a source id;
- unsupported claims are flagged;
- citation spans point to the correct document region;
- model summaries do not erase caveats.

### Temporal Evals

Check policy time awareness.

Examples:

- stale claims are marked stale;
- superseded claims are not presented as current;
- event time and publication time are not confused;
- forecast claims are labeled as forecasts.

### Reasoning Transfer Evals

Check whether expert mental tools affect output quality.

Examples:

- stakeholder incentives appear in the memo;
- second-order effects are considered;
- uncertainty is disclosed;
- conflicting evidence is not flattened;
- philosophical or institutional lenses are applied when relevant.

### PRAXIS Outcome Evals

Compare outputs from:

1. raw model prompt without capsule;
2. model prompt with loose source documents;
3. PRAXIS workflow with capsule context pack.

Score:

- accuracy;
- citation quality;
- policy usefulness;
- completeness;
- risk awareness;
- concision;
- reviewer acceptance.

### Extractor Distillation Evals

Check whether a fine-tuned extractor adapter is better than the prompted
teacher baseline before it can propose records for promoted capsules.

Examples:

- JSON/schema validity on source-pack chunks;
- source-span grounding for every proposed claim, term, edge, and language
  rule;
- temporal status classification;
- graph node and edge class validity;
- ontology-blueprint compatibility;
- review action coverage;
- rejected/unsupported object suppression;
- reviewer acceptance rate and correction distance.

Minimum gate:

- the fine-tuned extractor must beat the prompted baseline on the golden
  fixture and at least one held-out policy fixture;
- failures must be visible in the eval artifact;
- teacher model fallback remains available;
- no fine-tuned output can self-promote without human review state.

## Golden Fixture

The first golden fixture should be a small policy analysis problem with:

- 5 to 10 source documents;
- at least one outdated source;
- at least one conflicting claim;
- at least one actor map;
- at least one causal hypothesis;
- at least one reviewer correction;
- one target output contract.

## Promotion Gate

A capsule should not be promoted if:

- required schema validation fails;
- source ledger has unresolved references;
- review gate is required but missing;
- temporal ledger contains unresolved high-impact stale claims;
- eval report falls below threshold;
- bundle signature or checksum validation fails.

## Initial Metrics

| Metric | Target |
| --- | --- |
| Schema validity | 100 percent |
| Source reference resolution | 100 percent |
| High-impact unsupported claims | 0 |
| Required review coverage | 100 percent |
| Temporal warning precision | measured, then thresholded |
| PRAXIS improvement over raw baseline | positive in golden fixture |

## Eval Artifacts

Store eval artifacts under:

```text
fixtures/evals/
```

Each eval run should produce:

- input capsule id;
- model/provider metadata if used;
- output artifact;
- scores;
- failures;
- reviewer notes;
- run timestamp;
- git commit SHA.

Extractor eval artifacts should also include:

- teacher model alias;
- student model alias;
- adapter id;
- training dataset digest;
- held-out dataset digest;
- prompt/template version;
- schema version;
- confusion matrix for object classes and temporal statuses;
- reviewer correction summary.

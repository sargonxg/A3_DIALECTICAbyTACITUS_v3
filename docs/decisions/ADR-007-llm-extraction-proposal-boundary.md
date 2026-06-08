# ADR-007: LLM Extraction Is Proposal-Only Until Validation And Review

## Status

Accepted.

## Date

2026-06-08

## Context

DIALECTICA needs LLMs to help build powerful capsules from documents,
conversations, expert examples, and policy workflows. The engine must extract
claims, source spans, temporal episodes, ontology terms, graph records,
reasoning devices, language rules, caveats, and output guidance.

The same capability creates a risk: model output can look coherent while being
unsupported, temporally wrong, overgeneralized, or misaligned with expert
judgment. PRAXIS Capsules are meant to improve policy work, so the system cannot
treat model output as canonical truth.

## Decision

LLMs may only produce extraction proposals. A proposal is an intermediate record
with source spans, model invocation receipt, confidence, uncertainty, and review
triggers.

Canonical capsule records are produced only after:

1. deterministic normalization;
2. Rust validation;
3. review-trigger routing;
4. human approval where required;
5. compiler receipt generation.

The compiler must reject promoted output when required review is missing.

## Alternatives Considered

### Direct LLM-To-Capsule Compilation

Rejected. It is fast, but it lets a model write source-backed claims, graph
edges, and reasoning guidance without deterministic gates. That breaks the
sourceability and review model.

### No LLM Extraction

Rejected. Manual-only capsule creation would be too slow and would fail the
product goal of helping teams build rich capsules from large policy context.

### Python-Only Extraction Pipeline

Rejected as the canonical path. Python may help with reports and experiments,
but Rust must own the production contract, validation, compilation, and
promotion rules.

### Fine-Tuned Extractor First

Deferred. Fine-tuning is useful after the system has enough reviewed proposal
and correction data. It should not precede the source-pack, proposal, review,
and eval contracts.

## Consequences

- Add a planned `dialectica-extractor` crate for source-pack inputs, proposal
  schemas, model receipts, review-trigger routing, and provider traits.
- Keep model provider clients configurable and replaceable.
- Store model outputs as proposals, not truth.
- Require source spans or review actions for every material promoted record.
- Preserve rejected and superseded proposals in lineage.
- Make human-gated language, expert reasoning, causal graph edges, rights rules,
  and sensitive claims first-class review objects.
- Build evals around source fidelity, temporal correctness, graph integrity,
  reasoning-device adherence, and language-rule adherence.

## Follow-Up Work

- Implement proposal schemas and fixtures.
- Add review-router tests.
- Add compiler tests proving unreviewed proposals cannot be promoted.
- Add context-pack tests proving rejected records are hidden by default and
  caveats are preserved.

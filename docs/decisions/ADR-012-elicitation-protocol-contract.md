# ADR-012: Typed Elicitation Protocols Are Engine Contracts

## Status
Accepted

## Date
2026-06-10

## Context
DIALECTICA's BUILD lifecycle requires PRAXIS Capsule Studio and Codex/MCP
clients to ask structured questions instead of improvising prompts. The engine
must own the interview content because derived capsule records still need
source provenance, validation, and human review under ADR-007.

The existing JSONL discussion capture path already makes a transcript usable
as source material. What is missing is the typed protocol layer that tells a
client which stages to render, what records each stage is trying to fill, and
how complete-enough a session is before proposal generation.

## Decision
Add `elicitation_protocol.schema.json`, `elicitation_session.schema.json`, and
`elicitation_completeness_score.schema.json` as extractor-owned contracts.

An elicitation protocol contains:

- ordered stages;
- question templates;
- target record families;
- completion criteria;
- follow-up hints;
- completeness thresholds.

Four fixture protocols ship with the repo: `user.v1`, `situation.v1`,
`tool.v1`, and `output.v1`. Sessions record transcript-backed answers and
derived-record counts. Completeness scoring is deterministic and only reports
readiness; it does not promote derived records to canonical capsule truth.

The first runtime surface is fixture-backed:

- REST reads protocols and scores sessions;
- local MCP reads protocols and scores sessions;
- schema export publishes the contracts.

## Alternatives Considered

### Let PRAXIS Own The Prompts
Rejected because prompt drift would make Capsule Studio behavior diverge from
the engine's validation and review gates.

### Generate Protocols Dynamically With A Model
Rejected for the first slice because protocols are product contracts and must
be deterministic fixtures before any live extraction provider is trusted.

### Treat Completeness As Natural-Language Judgment
Rejected for v1. The local proof uses explicit derived-record counts so tests
can prove completeness without calling a model.

## Consequences
Positive:

- PRAXIS can render BUILD interviews from engine-owned content.
- Codex/MCP can inspect protocols locally.
- Tool Capsule expertise capture has concrete device/trap/precedent thresholds.

Negative:

- v1 completeness scoring depends on structured counts supplied by the caller
  or a future proposal generator.
- Transcript-to-proposal conversion is still a separate implementation slice
  and remains review-gated by ADR-007.

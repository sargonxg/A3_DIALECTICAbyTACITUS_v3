# API Contract

Status: draft for Phase 1 and Phase 2 implementation.

API Slice 1 is defined in [API Slice 1](API_SLICE_1.md). That file is
authoritative for the first local HTTP implementation.

## Design Principles

- Keep PRAXIS-facing APIs compact.
- Make every long-running operation job-based.
- Use idempotency keys for mutating requests.
- Return source, temporal, and review warnings explicitly.
- Expose embedded graph previews without exposing internal graph-engine details.
- Expose reviewed language profiles so PRAXIS can enforce terms, caveats, and
  audience register.
- Never expose model-provider internals unless requested by an audit endpoint.

## Common Headers

Required for mutating requests:

```http
Authorization: Bearer <token>
Idempotency-Key: <uuid>
X-Tacitus-Tenant: <tenant-id>
X-Tacitus-Project: <project-id>
```

## Status Model

Capsule job states:

- `queued`
- `ingesting`
- `extracting`
- `awaiting_review`
- `compiling`
- `validating`
- `ready`
- `failed`
- `cancelled`

Capsule states:

- `draft`
- `machine_proposed`
- `needs_review`
- `approved`
- `approved_with_caveats`
- `rejected`
- `promoted`
- `archived`

## Endpoints

### Create Capsule Job

```http
POST /v1/capsule-jobs
```

Request:

```json
{
  "title": "Energy security decision brief",
  "type": "situation",
  "spec_version": "3.0",
  "intended_workflows": ["decision_brief", "stakeholder_map"],
  "sources": [
    {
      "source_type": "uploaded_document",
      "uri": "gs://bucket/source.pdf",
      "title": "Source title"
    }
  ],
  "review_policy": {
    "required": true,
    "reviewer_roles": ["policy_expert"]
  }
}
```

Response:

```json
{
  "job_id": "job_123",
  "status": "queued",
  "next_poll_after_seconds": 5
}
```

### Get Capsule Job

```http
GET /v1/capsule-jobs/{job_id}
```

Response includes:

- status;
- current phase;
- created capsule id when available;
- blocking errors;
- review requirements;
- warnings.

### Get Capsule Manifest

```http
GET /v1/capsules/{capsule_id}/manifest
```

Response includes the public manifest plus:

- compatibility;
- freshness;
- review state;
- source count;
- warning count.

### Get PRAXIS Context Pack

```http
GET /v1/capsules/{capsule_id}/praxis-context-pack?workflow=decision_brief
```

Response:

```json
{
  "capsule_id": "cap_123",
  "spec_version": "3.0",
  "summary": "...",
  "agent_context": "...",
  "operations": "...",
  "retrieval_records": [],
  "citation_hints": [],
  "temporal_warnings": [],
  "reasoning_devices": [],
  "language_profile": {},
  "runtime_contract": {},
  "output_contract": {},
  "graph_focus": [],
  "graph_warnings": [],
  "capsule_health": {},
  "read_receipt_hints": {
    "bundle_digest": "sha256:...",
    "source_ids": [],
    "claim_ids": [],
    "graph_node_ids": [],
    "graph_edge_ids": [],
    "reasoning_device_ids": [],
    "language_rule_ids": [],
    "agent_guidance_ids": []
  },
  "forbidden_claims": [],
  "review_state": "approved"
}
```

### Get Elicitation Protocol

```http
GET /v1/protocols/{type}
```

`type` is one of `user`, `situation`, `tool`, or `output`.

Response is `elicitation_protocol.schema.json`: ordered stages, question
templates, target record families, follow-up hints, and completeness rules.
The local fixture-backed API serves `*.v1` protocols from
`fixtures/elicitation-protocols`.

### Score Elicitation Session

```http
POST /v1/protocols/{type}/score
```

Request is `elicitation_session.schema.json`; response is
`elicitation_completeness_score.schema.json`. Scoring only reports
complete-enough status. It does not promote derived records; transcript-derived
records still enter the proposal/review path.

## MCP Lanes

The local Codex bridge is `dialectica-mcp` over stdio. It accepts local
filesystem paths for package build, validation, archive, and export because it
runs inside the operator's machine and can be constrained with
`DIALECTICA_MCP_ROOTS`.

Hosted MCP is a separate future lane:

```http
POST /mcp
GET /mcp
```

Hosted `/mcp` uses Streamable HTTP and must follow the same API trust model as
the REST endpoints:

- require OAuth or service-to-service authentication;
- validate token audience and tenant/project ownership;
- address work by `build_id`, `capsule_id`, and artifact IDs;
- store artifacts in Cloud Storage and state in Cloud SQL PostgreSQL;
- return PRAXIS context packs by authenticated API/MCP call or signed artifact
  URL;
- reject raw filesystem paths in all hosted inputs.

The REST/API context-pack contract remains the first PRAXIS production
integration path. Hosted MCP is an adapter over the same handler core, not a
parallel source of truth.

### Combine Capsules

```http
POST /v1/capsule-sets
```

Request:

```json
{
  "capsule_ids": ["cap_123", "cap_456"],
  "workflow": "scenario_analysis"
}
```

Response includes:

- compatibility status;
- combined retrieval plan;
- merged graph preview;
- conflicts;
- stale claims;
- review warnings;
- rights and sharing conflicts;
- recommended context budget.

### Get Graph Preview

```http
GET /v1/capsules/{capsule_id}/graph-preview
```

Response:

```json
{
  "schema_version": "graph_preview_v1",
  "capsule_id": "cap_123",
  "graph_profile": "situation_graph_v1",
  "graph_lens": "stakeholder_power_lens",
  "nodes": [
    {
      "id": "actor:european-commission",
      "label": "European Commission",
      "node_type": "institution",
      "rank": 0.98,
      "why_surfaced": "central authority node for subsidy feasibility",
      "review_state": "approved"
    }
  ],
  "edges": [
    {
      "id": "edge:guidelines-regulated-by-commission",
      "from": "policy:state-aid-guidelines",
      "to": "actor:european-commission",
      "edge_type": "regulated_by",
      "review_state": "approved_with_caveats",
      "source_receipt_links": ["source:commission_guidance#span:12"],
      "temporal_status": "current"
    }
  ],
  "clusters": [],
  "review_styles": {
    "approved": "solid",
    "approved_with_caveats": "dashed",
    "needs_review": "muted",
    "rejected": "hidden_by_default"
  },
  "temporal_filters": ["current", "stale", "superseded", "forecast", "contested"],
  "source_receipt_links": [],
  "warnings": []
}
```

### Submit Review Decision

```http
POST /v1/capsules/{capsule_id}/reviews
```

Request:

```json
{
  "reviewed_object_type": "claim",
  "reviewed_object_id": "claim_123",
  "decision": "approved_with_caveats",
  "scope": "valid for Q2 2026 EU energy context",
  "notes": "Use with stale-source warning."
}
```

### Get Marketplace Listing

```http
GET /v1/capsules/{capsule_id}/marketplace-listing
```

Response includes:

- listing status;
- capsule type;
- review level;
- reviewer summary;
- freshness;
- rights summary;
- known caveats;
- lineage;
- compatible capsules;
- eval snapshot.

### Fork Capsule

```http
POST /v1/capsules/{capsule_id}/forks
```

Use when a team needs a private derivative capsule.

Response includes:

- fork capsule id;
- parent capsule id;
- parent bundle digest;
- inherited review scope;
- required new review gates.

### Export Bundle

```http
POST /v1/capsules/{capsule_id}/exports
```

Response:

```json
{
  "export_id": "export_123",
  "status": "queued",
  "format": "directory_or_archive"
}
```

### Record Capsule Read Receipt

```http
POST /v1/capsules/{capsule_id}/read-receipts
```

Use when PRAXIS uses a capsule in an answer, agent run, memo, or handover.

Request:

```json
{
  "praxis_run_id": "run_123",
  "workflow": "decision_brief",
  "bundle_digest": "sha256:...",
  "source_ids": ["source_1"],
  "claim_ids": ["claim_1"],
  "graph_node_ids": ["actor:european-commission"],
  "graph_edge_ids": ["edge:guidelines-regulated-by-commission"],
  "reasoning_device_ids": ["actor_incentive_map"],
  "language_rule_ids": ["language:caveat-legal-status"],
  "agent_guidance_ids": ["agent_guidance:decision_brief_v1"],
  "warnings_triggered": ["stale_claim"]
}
```

## Error Shape

```json
{
  "error": {
    "code": "review_required",
    "message": "Capsule cannot be promoted until required review gates pass.",
    "details": {
      "capsule_id": "cap_123",
      "missing_reviews": ["claim_456"]
    }
  }
}
```

## Compatibility Rule

PRAXIS must reject:

- unsupported major schema versions;
- unpromoted capsules in production workflows unless explicitly allowed;
- capsules with failing checksum validation;
- capsules with blocking review gates;
- stale high-impact claims without explicit user warning.
- graph previews with unreviewed critical edges unless explicitly marked.

## Local Health Routes

The first implementation uses:

- `GET /health`
- `GET /version`

Do not introduce `/healthz` or `/readyz` until deployment needs separate
liveness and readiness probes. Keep `/health` for local development.

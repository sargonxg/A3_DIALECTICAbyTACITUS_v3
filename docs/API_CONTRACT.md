# API Contract

Status: draft for Phase 1 and Phase 2 implementation.

## Design Principles

- Keep PRAXIS-facing APIs compact.
- Make every long-running operation job-based.
- Use idempotency keys for mutating requests.
- Return source, temporal, and review warnings explicitly.
- Expose embedded graph previews without exposing internal graph-engine details.
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
  "capsule_type": "policy_analysis",
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
  "schema_version": "0.1.0",
  "summary": "...",
  "retrieval_records": [],
  "citation_hints": [],
  "temporal_warnings": [],
  "reasoning_devices": [],
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
    "reasoning_device_ids": []
  },
  "forbidden_claims": [],
  "review_state": "approved"
}
```

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
  "capsule_id": "cap_123",
  "graph_profile": "actor_incentive_v1",
  "node_count": 84,
  "edge_count": 196,
  "review_state_counts": {
    "approved": 121,
    "approved_with_caveats": 18,
    "needs_review": 9
  },
  "hotspots": [],
  "contradiction_clusters": [],
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
  "graph_edge_ids": ["edge:guidelines-constrain-subsidy"],
  "reasoning_device_ids": ["actor_incentive_map"],
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

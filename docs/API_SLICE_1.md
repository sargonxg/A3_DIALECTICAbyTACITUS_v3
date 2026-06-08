# API Slice 1

Status: required API contract before implementing `services/dialectica-api`.

API Slice 1 exposes health, manifest, graph preview, context pack, and read
receipt surfaces over fixture data. It must not require production PRAXIS
credentials or cloud resources.

## Route Names

Use these names consistently:

```text
GET /health
GET /version
GET /v1/capsules/{capsule_id}/manifest
GET /v1/capsules/{capsule_id}/graph-preview
GET /v1/capsules/{capsule_id}/praxis-context-pack?workflow=decision_brief
POST /v1/capsules/{capsule_id}/read-receipts
```

Do not add `/healthz` or `/readyz` until deployment readiness requires separate
liveness and readiness probes. If those are added later, keep `/health` as the
developer-friendly local route.

## Auth Posture

For local fixture mode:

- no cloud credentials required;
- no production PRAXIS calls;
- no private user data;
- optional static local token only if a test needs auth behavior.

For staging:

- service-to-service auth required;
- tenant and project headers required for mutating routes;
- read-only routes may be exposed only to authenticated PRAXIS services.

## `GET /health`

Response:

```json
{
  "status": "ok",
  "service": "dialectica-api",
  "version": "0.1.0",
  "mode": "fixture",
  "dependencies": {
    "postgres": "not_configured",
    "cloud_storage": "not_configured",
    "task_queue": "not_configured"
  }
}
```

## `GET /version`

Response:

```json
{
  "service": "dialectica-api",
  "version": "0.1.0",
  "capsule_spec_version": "3.0",
  "graph_preview_schema_version": "graph_preview_v1",
  "commit": "local"
}
```

## `GET /v1/capsules/{capsule_id}/manifest`

Response must include:

- `capsule_id`;
- `spec_version`;
- `title`;
- `type`;
- `category`;
- `cores`;
- `status`;
- `review_state`;
- `freshness`;
- `source_count`;
- `graph_profile`;
- `provenance_root_hash`;
- `signature`;
- `compiled_at`;
- `compatible_workflows`;
- `warnings`.

`type` must be one of `user`, `situation`, `tool`, or `output`.

## `GET /v1/capsules/{capsule_id}/graph-preview`

Response schema: `graph_preview_v1` from `docs/GRAPH_PROFILE_REGISTRY.md`.

Minimum response fields:

- `schema_version`;
- `capsule_id`;
- `graph_profile`;
- `nodes`;
- `edges`;
- `clusters`;
- `review_styles`;
- `temporal_filters`;
- `source_receipt_links`;
- `warnings`.

## `GET /v1/capsules/{capsule_id}/praxis-context-pack`

Response must include:

- capsule summary;
- selected retrieval records;
- source and citation hints;
- temporal warnings;
- graph focus nodes and edge warnings;
- reasoning devices;
- runtime contract and operations;
- output contract;
- forbidden claims;
- read receipt hints.

## `POST /v1/capsules/{capsule_id}/read-receipts`

Request:

```json
{
  "praxis_run_id": "run_fixture_001",
  "workflow": "decision_brief",
  "bundle_digest": "sha256:fixture",
  "source_ids": ["source:commission_guidance"],
  "claim_ids": ["claim:state-aid-guidance-current"],
  "graph_node_ids": ["actor:european-commission"],
  "graph_edge_ids": ["edge:guidelines-regulated-by-commission"],
  "reasoning_device_ids": ["stakeholder_analysis_v1"],
  "agent_guidance_ids": ["agent_guidance:decision_brief_v1"],
  "warnings_triggered": ["approved_with_caveats"]
}
```

Response:

```json
{
  "receipt_id": "receipt_fixture_001",
  "status": "recorded",
  "capsule_id": "cap_eu_energy_stakeholders_2026_q3",
  "bundle_digest": "sha256:fixture"
}
```

## Done Gate

API Slice 1 is done when:

- the API boots locally;
- `/health` and `/version` return without cloud credentials;
- manifest, graph preview, and context-pack routes can serve fixture data;
- read receipt accepts fixture payload and returns deterministic receipt id;
- route tests are added;
- `docs/API_CONTRACT.md` and `docs/PRAXIS_INTEGRATION.md` match this file.

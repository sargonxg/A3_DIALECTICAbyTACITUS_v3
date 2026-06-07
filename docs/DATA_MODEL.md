# Data Model

Status: draft PostgreSQL-first model.

## Principle

PostgreSQL is the first operational source of truth. The capsule bundle is the
portable export contract. Graph, ontology, and vector systems are derived
adapters until an ADR promotes them.

## Core Tables

### `tenants`

- `id`
- `name`
- `created_at`

### `projects`

- `id`
- `tenant_id`
- `name`
- `created_at`

### `capsule_jobs`

- `id`
- `tenant_id`
- `project_id`
- `requested_by`
- `status`
- `phase`
- `idempotency_key`
- `created_at`
- `updated_at`
- `error_code`
- `error_message`

### `capsules`

- `id`
- `tenant_id`
- `project_id`
- `title`
- `capsule_type`
- `schema_version`
- `status`
- `freshness`
- `compiled_at`
- `promoted_at`
- `bundle_digest`
- `created_at`
- `updated_at`

### `sources`

- `id`
- `tenant_id`
- `project_id`
- `capsule_id`
- `source_type`
- `title`
- `uri`
- `publisher`
- `published_at`
- `retrieved_at`
- `language`
- `license_or_access`
- `trust_status`
- `artifact_hash`
- `created_at`

### `source_spans`

- `id`
- `source_id`
- `locator`
- `text_hash`
- `char_start`
- `char_end`
- `page`
- `section`
- `created_at`

### `extraction_runs`

- `id`
- `capsule_id`
- `source_id`
- `extractor_kind`
- `model_provider`
- `model_alias`
- `prompt_version`
- `input_digest`
- `output_digest`
- `started_at`
- `completed_at`
- `status`

### `entities`

- `id`
- `capsule_id`
- `entity_type`
- `name`
- `canonical_name`
- `confidence`
- `review_state`
- `created_by_run_id`

### `claims`

- `id`
- `capsule_id`
- `claim_text`
- `claim_type`
- `confidence`
- `review_state`
- `created_by_run_id`

### `claim_sources`

- `claim_id`
- `source_id`
- `source_span_id`
- `support_type`

### `temporal_facts`

- `id`
- `capsule_id`
- `claim_id`
- `event_time_start`
- `event_time_end`
- `published_at`
- `observed_at`
- `status`
- `supersedes_claim_id`
- `superseded_by_claim_id`

### `ontology_terms`

- `id`
- `capsule_id`
- `term_type`
- `label`
- `definition`
- `parent_term_id`
- `review_state`

### `ontology_mappings`

- `id`
- `capsule_id`
- `entity_id`
- `term_id`
- `confidence`
- `review_state`
- `source_span_id`

### `graph_nodes`

- `id`
- `capsule_id`
- `node_type`
- `label`
- `ref_table`
- `ref_id`
- `review_state`

### `graph_edges`

- `id`
- `capsule_id`
- `from_node_id`
- `to_node_id`
- `edge_type`
- `confidence`
- `review_state`
- `source_span_id`
- `created_by_run_id`

### `graph_constraints`

- `id`
- `capsule_id`
- `graph_profile`
- `node_classes_json`
- `edge_classes_json`
- `required_fields_json`
- `validation_errors_json`
- `created_at`

### `review_decisions`

- `id`
- `capsule_id`
- `reviewer_id`
- `reviewer_role`
- `reviewed_object_type`
- `reviewed_object_id`
- `decision`
- `scope`
- `notes`
- `created_at`
- `expires_at`

### `rights_profiles`

- `id`
- `capsule_id`
- `owner_id`
- `allowed_workflows_json`
- `prohibited_workflows_json`
- `export_policy`
- `sharing_policy`
- `source_license_summary`
- `sensitive_fields_json`
- `redaction_rules_json`
- `marketplace_policy`
- `expires_at`
- `created_at`

### `marketplace_listings`

- `id`
- `capsule_id`
- `listing_status`
- `review_level`
- `reviewer_summary`
- `domain_tags_json`
- `geography`
- `language`
- `freshness_status`
- `rights_summary`
- `known_caveats_json`
- `compatible_capsules_json`
- `fork_policy`
- `eval_snapshot_json`
- `created_at`
- `updated_at`

### `capsule_lineage`

- `id`
- `capsule_id`
- `parent_capsule_id`
- `parent_bundle_digest`
- `lineage_kind`
- `inherited_review_json`
- `local_change_summary`
- `created_at`

### `reasoning_devices`

- `id`
- `capsule_id`
- `device_type`
- `label`
- `purpose`
- `method_steps_json`
- `failure_modes_json`
- `review_state`
- `created_by_run_id`

### `capsule_health_reports`

- `id`
- `capsule_id`
- `source_coverage`
- `unsupported_claim_count`
- `stale_claim_count`
- `contested_claim_count`
- `review_coverage`
- `reasoning_device_coverage`
- `praxis_eval_score`
- `blocking_warnings_json`
- `recommended_next_actions_json`
- `created_at`

### `capsule_read_receipts`

- `id`
- `capsule_id`
- `praxis_run_id`
- `workflow`
- `bundle_digest`
- `source_ids_json`
- `claim_ids_json`
- `reasoning_device_ids_json`
- `warnings_triggered_json`
- `created_at`

### `bundle_exports`

- `id`
- `capsule_id`
- `format`
- `storage_uri`
- `bundle_digest`
- `signature_ref`
- `created_at`

## Indexing Strategy

Initial indexes:

- tenant/project/capsule composite indexes;
- capsule job status and phase;
- source artifact hash;
- claim review state;
- temporal status;
- graph edge from/to node ids;
- graph profile and validation errors;
- review object lookup;
- marketplace listing status;
- capsule lineage parent lookup;
- bundle digest.
- capsule read receipts by capsule and PRAXIS run.

Optional later indexes:

- full-text search over source spans and claims;
- pgvector embeddings for retrieval records;
- trigram indexes for entity resolution.

## Migration Rules

- Every schema change needs a migration.
- Migrations must be reversible until production deploy.
- Bundle schema compatibility must be documented with database schema changes.
- Backfills must preserve user edits and review decisions.

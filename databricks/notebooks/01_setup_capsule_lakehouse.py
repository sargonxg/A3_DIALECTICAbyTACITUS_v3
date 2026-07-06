# Databricks notebook source
# MAGIC %md
# MAGIC # TACITUS Context Capsule Lakehouse Setup
# MAGIC
# MAGIC Creates a governed lakehouse shape for generic Context Capsule building.
# MAGIC Databricks owns evidence, lineage, analytics, and evals. DIALECTICA owns
# MAGIC validation/compilation. PRAXIS owns the visible cockpit.

# COMMAND ----------

dbutils.widgets.text("catalog", "dialectica")
catalog = dbutils.widgets.get("catalog")

schemas = [
    "capsule_registry",
    "capsule_bronze",
    "capsule_silver",
    "capsule_gold",
    "capsule_exports",
    "capsule_evals",
]

for schema in schemas:
    spark.sql(f"CREATE SCHEMA IF NOT EXISTS {catalog}.{schema}")

# COMMAND ----------

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_registry.domain_packs (
  domain_pack_id STRING NOT NULL,
  name STRING,
  market STRING,
  ontology_focus ARRAY<STRING>,
  capsule_kinds ARRAY<STRING>,
  created_at TIMESTAMP,
  notes STRING
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_registry.ontology_blueprints (
  ontology_id STRING NOT NULL,
  domain_pack_id STRING,
  name STRING,
  version STRING,
  target_capsule_kind STRING,
  core_classes ARRAY<STRING>,
  temporal_model STRING,
  causal_model STRING,
  review_rubric STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_registry.source_registry (
  source_id STRING NOT NULL,
  source_kind STRING,
  title STRING,
  source_uri STRING,
  rights_profile STRING,
  license_status STRING,
  allowed_uses ARRAY<STRING>,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_registry.open_data_source_catalog (
  source_key STRING NOT NULL,
  label STRING,
  category STRING,
  connector_kind STRING,
  access_pattern STRING,
  endpoint_uri STRING,
  license_summary STRING,
  auth_profile STRING,
  refresh_cadence STRING,
  default_scope STRING,
  tacitus_use_case STRING,
  user_value STRING,
  reliability_notes STRING,
  review_policy STRING,
  created_at TIMESTAMP
) USING DELTA
""")

# COMMAND ----------

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_bronze.raw_inputs (
  input_id STRING NOT NULL,
  capsule_kind STRING,
  domain_pack_id STRING,
  source_id STRING,
  input_kind STRING,
  title STRING,
  content STRING,
  collected_at TIMESTAMP,
  metadata_json STRING
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_bronze.human_knowledge_records (
  knowledge_id STRING NOT NULL,
  domain_pack_id STRING,
  source_id STRING,
  expert_role STRING,
  statement STRING,
  review_state STRING,
  private_scope STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_bronze.capsule_builder_sessions (
  session_id STRING NOT NULL,
  requested_by STRING,
  target_surface STRING,
  requested_capsule_kind STRING,
  user_prompt STRING,
  consent_profile STRING,
  source_policy_json STRING,
  status STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_bronze.builder_interview_turns (
  turn_id STRING NOT NULL,
  session_id STRING,
  speaker STRING,
  message STRING,
  captured_signal_kind STRING,
  proposed_capsule_field STRING,
  confidence DOUBLE,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_bronze.source_connector_requests (
  connector_request_id STRING NOT NULL,
  session_id STRING,
  connector_kind STRING,
  connector_label STRING,
  data_scope STRING,
  status STRING,
  consent_required BOOLEAN,
  notes STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_bronze.open_data_observations (
  observation_id STRING NOT NULL,
  source_key STRING,
  topic STRING,
  title STRING,
  url STRING,
  observed_at TIMESTAMP,
  geography STRING,
  actors ARRAY<STRING>,
  signal_kind STRING,
  source_payload_json STRING,
  rights_profile STRING,
  review_state STRING,
  created_at TIMESTAMP
) USING DELTA
""")

# COMMAND ----------

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_silver.evidence_spans (
  span_id STRING NOT NULL,
  source_id STRING,
  input_id STRING,
  text STRING,
  locator STRING,
  source_hash STRING,
  rights_profile STRING,
  trust_state STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_silver.graph_partitions (
  partition_id STRING NOT NULL,
  capsule_id STRING,
  partition_kind STRING,
  ontology_id STRING,
  node_count BIGINT,
  edge_count BIGINT,
  review_state STRING,
  materialization_policy STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_silver.capsule_similarity_matches (
  match_id STRING NOT NULL,
  session_id STRING,
  candidate_capsule_id STRING,
  match_kind STRING,
  similarity_score DOUBLE,
  match_rationale STRING,
  reuse_policy STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_silver.ontology_terms (
  term_id STRING NOT NULL,
  domain_pack_id STRING,
  label STRING,
  definition STRING,
  parent_term_id STRING,
  capsule_kind STRING,
  review_state STRING,
  source_span_ids ARRAY<STRING>,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_silver.semantic_nodes (
  node_id STRING NOT NULL,
  capsule_id STRING,
  domain_pack_id STRING,
  node_type STRING,
  label STRING,
  source_span_ids ARRAY<STRING>,
  properties_json STRING,
  review_state STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_silver.semantic_edges (
  edge_id STRING NOT NULL,
  capsule_id STRING,
  edge_type STRING,
  from_node_id STRING,
  to_node_id STRING,
  source_span_ids ARRAY<STRING>,
  confidence DOUBLE,
  temporal_scope_json STRING,
  causal_scope_json STRING,
  review_state STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_silver.temporal_relations (
  temporal_relation_id STRING NOT NULL,
  capsule_id STRING,
  relation_type STRING,
  subject_id STRING,
  object_id STRING,
  valid_from TIMESTAMP,
  valid_until TIMESTAMP,
  known_at TIMESTAMP,
  allen_relation STRING,
  uncertainty STRING,
  source_span_ids ARRAY<STRING>,
  review_state STRING
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_silver.causal_relations (
  causal_relation_id STRING NOT NULL,
  capsule_id STRING,
  cause_id STRING,
  effect_id STRING,
  mechanism STRING,
  confidence DOUBLE,
  assumptions ARRAY<STRING>,
  source_span_ids ARRAY<STRING>,
  review_state STRING
) USING DELTA
""")

# COMMAND ----------

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.capsule_manifests (
  capsule_id STRING NOT NULL,
  capsule_kind STRING,
  domain_pack_id STRING,
  title STRING,
  summary STRING,
  review_state STRING,
  freshness STRING,
  bundle_digest STRING,
  source_count BIGINT,
  claim_count BIGINT,
  graph_node_count BIGINT,
  graph_edge_count BIGINT,
  compiled_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.capsule_claims (
  claim_id STRING NOT NULL,
  capsule_id STRING,
  claim_text STRING,
  stance STRING,
  confidence DOUBLE,
  source_span_ids ARRAY<STRING>,
  review_state STRING,
  uncertainty STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.review_verdicts (
  review_id STRING NOT NULL,
  capsule_id STRING,
  reviewer_role STRING,
  decision STRING,
  caveats ARRAY<STRING>,
  reviewed_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.reasoning_devices (
  device_id STRING NOT NULL,
  capsule_id STRING,
  label STRING,
  device_kind STRING,
  procedure_steps ARRAY<STRING>,
  traps ARRAY<STRING>,
  required_primitives ARRAY<STRING>,
  reviewer_role STRING,
  review_state STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.memory_build_history (
  decision_id STRING NOT NULL,
  capsule_id STRING,
  build_phase STRING,
  actor STRING,
  decision STRING,
  rationale STRING,
  source_refs ARRAY<STRING>,
  decided_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.runtime_contracts (
  contract_id STRING NOT NULL,
  capsule_id STRING,
  target_surface STRING,
  trust_rules_json STRING,
  citation_policy STRING,
  composition_rules_json STRING,
  stop_conditions ARRAY<STRING>,
  review_state STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.capsule_improvement_proposals (
  proposal_id STRING NOT NULL,
  session_id STRING,
  capsule_id STRING,
  proposal_kind STRING,
  target_layer STRING,
  title STRING,
  proposal_json STRING,
  review_state STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.agent_guidance_frames (
  frame_id STRING NOT NULL,
  session_id STRING,
  capsule_id STRING,
  target_agent STRING,
  deterministic_controls_json STRING,
  retrieval_contract_json STRING,
  allowed_generation_space_json STRING,
  review_required BOOLEAN,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_exports.context_pack_exports (
  export_id STRING NOT NULL,
  capsule_id STRING,
  target_surface STRING,
  context_pack_json STRING,
  export_status STRING,
  exported_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_evals.capsule_quality_scores (
  eval_id STRING NOT NULL,
  capsule_id STRING,
  citation_precision DOUBLE,
  unsupported_claim_rate DOUBLE,
  source_coverage DOUBLE,
  human_acceptance STRING,
  notes STRING,
  measured_at TIMESTAMP
) USING DELTA
""")

# COMMAND ----------

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.capsule_showcase AS
SELECT
  m.capsule_kind,
  m.capsule_id,
  m.title,
  m.review_state,
  m.source_count,
  m.claim_count,
  m.graph_node_count,
  m.graph_edge_count,
  e.citation_precision,
  e.source_coverage,
  x.export_status
FROM {catalog}.capsule_gold.capsule_manifests m
LEFT JOIN {catalog}.capsule_evals.capsule_quality_scores e
  ON m.capsule_id = e.capsule_id
LEFT JOIN {catalog}.capsule_exports.context_pack_exports x
  ON m.capsule_id = x.capsule_id
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.guided_builder_showcase AS
SELECT
  s.session_id,
  s.requested_capsule_kind,
  s.status AS session_status,
  p.capsule_id,
  p.proposal_kind,
  p.target_layer,
  p.review_state AS proposal_review_state,
  f.target_agent,
  f.review_required
FROM {catalog}.capsule_bronze.capsule_builder_sessions s
LEFT JOIN {catalog}.capsule_gold.capsule_improvement_proposals p
  ON s.session_id = p.session_id
LEFT JOIN {catalog}.capsule_gold.agent_guidance_frames f
  ON p.session_id = f.session_id
 AND p.capsule_id = f.capsule_id
""")

print(f"Initialized TACITUS Context Capsule schemas in catalog={catalog}")

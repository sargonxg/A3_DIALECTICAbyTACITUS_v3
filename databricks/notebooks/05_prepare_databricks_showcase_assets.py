# Databricks notebook source
# MAGIC %md
# MAGIC # Prepare Databricks Showcase Assets
# MAGIC
# MAGIC Creates curated Databricks-only demo assets on top of the capsule factory:
# MAGIC
# MAGIC - dashboard-ready views for KPIs, graph scale, reviews, and agent frames;
# MAGIC - an AI-search-ready Delta table that combines evidence, claims, and agent
# MAGIC   guidance text;
# MAGIC - compact CLI-visible proof rows for the TACITUS Databricks demo.

# COMMAND ----------

from datetime import datetime
import json

from pyspark.sql import Row

dbutils.widgets.text("catalog", "dialectica")
catalog = dbutils.widgets.get("catalog")
now = datetime.utcnow()

# COMMAND ----------

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.ai_search_corpus (
  corpus_id STRING NOT NULL,
  capsule_id STRING,
  source_layer STRING,
  title STRING,
  body STRING,
  review_state STRING,
  source_refs ARRAY<STRING>,
  metadata_json STRING,
  updated_at TIMESTAMP
) USING DELTA
TBLPROPERTIES (delta.enableChangeDataFeed = true)
""")

spark.sql(f"DELETE FROM {catalog}.capsule_gold.ai_search_corpus WHERE corpus_id LIKE 'demo:%'")

evidence_rows = [
    Row(
        corpus_id=f"demo:corpus:evidence:{row.span_id}",
        capsule_id=None,
        source_layer="evidence_span",
        title=row.locator,
        body=row.text,
        review_state=row.trust_state,
        source_refs=[row.source_id, row.input_id],
        metadata_json=json.dumps({"rights_profile": row.rights_profile, "source_hash": row.source_hash}, sort_keys=True),
        updated_at=now,
    )
    for row in spark.table(f"{catalog}.capsule_silver.evidence_spans")
    .where("span_id LIKE 'demo:%'")
    .collect()
]

claim_rows = [
    Row(
        corpus_id=f"demo:corpus:claim:{row.claim_id}",
        capsule_id=row.capsule_id,
        source_layer="capsule_claim",
        title=row.stance,
        body=row.claim_text,
        review_state=row.review_state,
        source_refs=row.source_span_ids,
        metadata_json=json.dumps({"confidence": row.confidence, "uncertainty": row.uncertainty}, sort_keys=True),
        updated_at=now,
    )
    for row in spark.table(f"{catalog}.capsule_gold.capsule_claims")
    .where("claim_id LIKE 'demo:%'")
    .collect()
]

frame_rows = [
    Row(
        corpus_id=f"demo:corpus:frame:{row.frame_id}",
        capsule_id=row.capsule_id,
        source_layer="agent_guidance_frame",
        title=row.target_agent,
        body=" ".join(
            [
                row.deterministic_controls_json,
                row.retrieval_contract_json,
                row.allowed_generation_space_json,
            ]
        ),
        review_state="review_required" if row.review_required else "approved",
        source_refs=[row.session_id],
        metadata_json=json.dumps({"target_agent": row.target_agent}, sort_keys=True),
        updated_at=now,
    )
    for row in spark.table(f"{catalog}.capsule_gold.agent_guidance_frames")
    .where("frame_id LIKE 'demo:%'")
    .collect()
]

corpus = evidence_rows + claim_rows + frame_rows
if corpus:
    spark.createDataFrame(corpus).write.mode("append").saveAsTable(
        f"{catalog}.capsule_gold.ai_search_corpus"
    )

# COMMAND ----------

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_kpis AS
SELECT 'capsules' AS metric, COUNT(*) AS value
FROM {catalog}.capsule_gold.capsule_manifests
WHERE capsule_id LIKE 'cap_%_demo'
UNION ALL
SELECT 'builder_sessions', COUNT(*)
FROM {catalog}.capsule_bronze.capsule_builder_sessions
WHERE session_id LIKE 'demo:%'
UNION ALL
SELECT 'candidate_graph_nodes', COUNT(*)
FROM {catalog}.capsule_silver.semantic_nodes
WHERE node_id LIKE 'demo:deep:%'
UNION ALL
SELECT 'candidate_graph_edges', COUNT(*)
FROM {catalog}.capsule_silver.semantic_edges
WHERE edge_id LIKE 'demo:deep:%'
UNION ALL
SELECT 'reviewable_proposals', COUNT(*)
FROM {catalog}.capsule_gold.capsule_improvement_proposals
WHERE session_id LIKE 'demo:%'
UNION ALL
SELECT 'agent_guidance_frames', COUNT(*)
FROM {catalog}.capsule_gold.agent_guidance_frames
WHERE session_id LIKE 'demo:%'
UNION ALL
SELECT 'ai_search_corpus_rows', COUNT(*)
FROM {catalog}.capsule_gold.ai_search_corpus
WHERE corpus_id LIKE 'demo:%'
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_capsule_portfolio AS
SELECT
  m.capsule_kind,
  m.capsule_id,
  m.title,
  m.review_state,
  m.source_count,
  m.claim_count,
  m.graph_node_count,
  m.graph_edge_count,
  q.citation_precision,
  q.source_coverage,
  q.unsupported_claim_rate
FROM {catalog}.capsule_gold.capsule_manifests m
LEFT JOIN {catalog}.capsule_evals.capsule_quality_scores q
  ON m.capsule_id = q.capsule_id
WHERE m.capsule_id LIKE 'cap_%_demo'
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_graph_scale AS
SELECT
  p.capsule_id,
  m.capsule_kind,
  p.ontology_id,
  p.node_count,
  p.edge_count,
  p.review_state,
  p.materialization_policy
FROM {catalog}.capsule_silver.graph_partitions p
LEFT JOIN {catalog}.capsule_gold.capsule_manifests m
  ON p.capsule_id = m.capsule_id
WHERE p.partition_id LIKE 'demo:%'
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_review_workbench AS
SELECT
  p.session_id,
  m.capsule_kind,
  p.capsule_id,
  p.target_layer,
  p.proposal_kind,
  p.title,
  p.review_state
FROM {catalog}.capsule_gold.capsule_improvement_proposals p
LEFT JOIN {catalog}.capsule_gold.capsule_manifests m
  ON p.capsule_id = m.capsule_id
WHERE p.session_id LIKE 'demo:%'
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_agent_guidance AS
SELECT
  f.session_id,
  m.capsule_kind,
  f.capsule_id,
  f.target_agent,
  f.review_required,
  f.deterministic_controls_json,
  f.retrieval_contract_json,
  f.allowed_generation_space_json
FROM {catalog}.capsule_gold.agent_guidance_frames f
LEFT JOIN {catalog}.capsule_gold.capsule_manifests m
  ON f.capsule_id = m.capsule_id
WHERE f.session_id LIKE 'demo:%'
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_connector_boundary AS
SELECT
  s.session_id,
  c.connector_kind,
  c.connector_label,
  c.data_scope,
  c.status,
  c.consent_required,
  c.notes
FROM {catalog}.capsule_bronze.capsule_builder_sessions s
JOIN {catalog}.capsule_bronze.source_connector_requests c
  ON s.session_id = c.session_id
WHERE s.session_id LIKE 'demo:%'
""")

# COMMAND ----------

display(spark.table(f"{catalog}.capsule_gold.dashboard_kpis"))
display(spark.table(f"{catalog}.capsule_gold.dashboard_capsule_portfolio"))
display(spark.table(f"{catalog}.capsule_gold.dashboard_graph_scale"))
display(spark.table(f"{catalog}.capsule_gold.dashboard_review_workbench"))
display(spark.table(f"{catalog}.capsule_gold.dashboard_agent_guidance"))
display(spark.table(f"{catalog}.capsule_gold.ai_search_corpus").limit(20))

print(f"Prepared Databricks showcase assets in catalog={catalog}. AI-search-ready corpus rows: {len(corpus)}")

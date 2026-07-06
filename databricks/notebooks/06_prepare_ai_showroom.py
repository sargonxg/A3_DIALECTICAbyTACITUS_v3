# Databricks notebook source
# MAGIC %md
# MAGIC # Prepare TACITUS AI Showroom
# MAGIC
# MAGIC Builds the Databricks-native explanation layer for the TACITUS Context
# MAGIC Capsule Factory. This is the "show this to a Databricks engineer" layer:
# MAGIC click paths, platform capability map, AI extraction runs, ontology design
# MAGIC cards, and a capsule deep-dive view.
# MAGIC
# MAGIC The notebook intentionally uses `ai_query` against a Databricks-hosted
# MAGIC foundation model endpoint so the demo proves that extraction and ontology
# MAGIC drafting can happen inside Databricks SQL/Workflows, not in an external app.

# COMMAND ----------

from datetime import datetime
import json

from pyspark.sql import Row

dbutils.widgets.text("catalog", "dialectica")
dbutils.widgets.text("model_endpoint", "databricks-gpt-5-mini")
catalog = dbutils.widgets.get("catalog")
model_endpoint = dbutils.widgets.get("model_endpoint")
model_endpoint_sql = model_endpoint.replace("'", "''")
now = datetime.utcnow()

# COMMAND ----------

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.showcase_click_path (
  step_id INT NOT NULL,
  surface STRING,
  click_path STRING,
  what_to_show STRING,
  talk_track STRING,
  proof_object STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.databricks_capability_map (
  capability_id STRING NOT NULL,
  databricks_surface STRING,
  tacitus_use STRING,
  why_it_matters STRING,
  proof_object STRING,
  maturity STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.ai_extraction_runs (
  run_id STRING NOT NULL,
  capsule_kind STRING,
  source_surface STRING,
  source_text STRING,
  model_endpoint STRING,
  extraction_task STRING,
  prompt_text STRING,
  ai_result STRING,
  review_state STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.ontology_design_cards (
  ontology_card_id STRING NOT NULL,
  capsule_kind STRING,
  ontology_name STRING,
  primitive_classes ARRAY<STRING>,
  temporal_primitives ARRAY<STRING>,
  causal_primitives ARRAY<STRING>,
  review_rules ARRAY<STRING>,
  agent_control_rules ARRAY<STRING>,
  example_question STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.capsule_showroom_narrative (
  narrative_id STRING NOT NULL,
  section STRING,
  headline STRING,
  body STRING,
  proof_table STRING,
  demo_prompt STRING,
  created_at TIMESTAMP
) USING DELTA
""")

for table, column in [
    ("capsule_gold.showcase_click_path", "step_id"),
    ("capsule_gold.databricks_capability_map", "capability_id"),
    ("capsule_gold.ai_extraction_runs", "run_id"),
    ("capsule_gold.ontology_design_cards", "ontology_card_id"),
    ("capsule_gold.capsule_showroom_narrative", "narrative_id"),
]:
    if column == "step_id":
        spark.sql(f"DELETE FROM {catalog}.{table} WHERE step_id BETWEEN 1 AND 99")
    else:
        spark.sql(f"DELETE FROM {catalog}.{table} WHERE {column} LIKE 'demo:%'")

# COMMAND ----------

click_path_rows = [
    Row(1, "Workspace", "Workspace > Users > giulio@tacitus.me > .bundle > tacitus-context-capsule-factory > dev > files > notebooks", "Open notebooks 01 through 06.", "This proves the whole factory is deployed as versioned Databricks code, not a slide deck.", "Databricks bundle source files", now),
    Row(2, "Workflows", "Workflows > [dev] TACITUS Context Capsule Builder Showcase > Runs", "Open the latest successful run and expand every task.", "The run shows a reproducible pipeline from lakehouse setup to AI extraction showroom.", "Job 1123194333821498", now),
    Row(3, "Catalog", "Catalog > dialectica > capsule_bronze", "Show raw inputs, human knowledge records, guided builder sessions, connector requests.", "This is where documents, user knowledge, and consent-bounded connector requests land.", f"{catalog}.capsule_bronze.*", now),
    Row(4, "Catalog", "Catalog > dialectica > capsule_silver", "Show ontology terms, semantic nodes/edges, temporal relations, causal relations.", "This is the determinizing layer: messy inputs become typed graph, time, causality, and ontology primitives.", f"{catalog}.capsule_silver.*", now),
    Row(5, "Catalog", "Catalog > dialectica > capsule_gold", "Show manifests, claims, runtime contracts, AI extraction runs, showroom views.", "This is where Databricks produces reviewable Context Capsules that PRAXIS can attach.", f"{catalog}.capsule_gold.*", now),
    Row(6, "SQL Editor", "SQL Editor > Saved queries", "Open TACITUS Capsule Factory queries.", "A Databricks engineer can inspect the demo without running the app.", "Saved TACITUS query pack", now),
    Row(7, "Databricks App", "Apps > tacitus-capsule-builder-dev", "Open the control room tabs: Showroom, AI Lab, Ontology, Portfolio, Graph, Review, Exports.", "This is the executive demo surface built on governed Databricks tables.", "tacitus-capsule-builder-dev", now),
]

spark.createDataFrame(
    click_path_rows,
    "step_id int, surface string, click_path string, what_to_show string, talk_track string, proof_object string, created_at timestamp",
).write.mode("append").saveAsTable(f"{catalog}.capsule_gold.showcase_click_path")

capability_rows = [
    Row("demo:capability:bundle", "Declarative Automation Bundles", "Deploy the whole capsule factory from CLI.", "Makes the demo reproducible, inspectable, and promotable across dev/prod.", "databricks.yml plus resources/*.yml", "working", now),
    Row("demo:capability:workflow", "Workflows / Jobs", "Run the six-stage capsule build plus AI showroom build.", "Shows that capsule generation is operational, scheduled, and auditable.", "TACITUS Context Capsule Builder Showcase job", "working", now),
    Row("demo:capability:unity-catalog", "Unity Catalog", "Govern bronze/silver/gold/export/eval capsule assets.", "Every source, claim, ontology node, and export lives under a governed namespace.", f"{catalog}.capsule_* schemas", "working", now),
    Row("demo:capability:ai-functions", "Databricks AI Functions", "Use ai_query for extraction, ontology drafting, and review analysis.", "Demonstrates AI inside Databricks over governed Delta data.", f"{catalog}.capsule_gold.ai_extraction_runs", "working", now),
    Row("demo:capability:model-serving", "Mosaic AI Model Serving", "Use hosted foundation model endpoints for capsule extraction.", "Keeps inference governed by Databricks permissions and endpoint readiness.", model_endpoint, "working", now),
    Row("demo:capability:delta", "Delta Lake", "Persist auditable capsule products, AI outputs, and search corpus.", "Capsules become data products, not ephemeral chat results.", f"{catalog}.capsule_gold.ai_search_corpus", "working", now),
    Row("demo:capability:ai-search", "AI Search / Vector Search", "Use the AI-search-ready corpus as the Delta Sync source.", "Turns capsules into retrieval infrastructure for PRAXIS agents.", f"{catalog}.capsule_gold.ai_search_corpus", "ready_next", now),
    Row("demo:capability:apps", "Databricks Apps", "Serve the control room from inside the workspace.", "Gives stakeholders a governed showcase without leaving Databricks.", "tacitus-capsule-builder-dev", "working", now),
    Row("demo:capability:evals", "Evaluation tables / MLflow-ready metrics", "Track citation precision, unsupported claims, source coverage, review state.", "Makes capsule quality measurable and fundable.", f"{catalog}.capsule_evals.capsule_quality_scores", "working", now),
]

spark.createDataFrame(
    capability_rows,
    "capability_id string, databricks_surface string, tacitus_use string, why_it_matters string, proof_object string, maturity string, created_at timestamp",
).write.mode("append").saveAsTable(f"{catalog}.capsule_gold.databricks_capability_map")

# COMMAND ----------

ontology_rows = [
    Row(
        "demo:ontology:user",
        "USER",
        "Expert/User Context Ontology",
        ["Preference", "ExpertRole", "ReviewStandard", "PrivateMemory", "VoicePattern", "Constraint"],
        ["preference_valid_from", "preference_known_at", "staleness_window", "revocation_time"],
        ["preference_causes_format_choice", "review_standard_blocks_claim", "privacy_scope_limits_retrieval"],
        ["Never publish private user context", "Mark preference as current only when known_at is fresh", "Require human approval for sensitive personalization"],
        ["Use source-led brief style", "Prefer caveats over confidence theater", "Block public leakage"],
        "What should PRAXIS know about this user before drafting or reviewing analysis?",
        now,
    ),
    Row(
        "demo:ontology:situation",
        "SITUATION",
        "Policy Situation Ontology",
        ["Actor", "Institution", "PolicyInstrument", "Obligation", "Constraint", "Event", "OpenQuestion"],
        ["event_time", "deadline", "known_at", "valid_until", "before_after_meets"],
        ["instrument_changes_actor_incentive", "capacity_constrains_enforcement", "uncertainty_delays_decision"],
        ["Separate occurred_at from known_at", "Do not assert current state from stale evidence", "Attach every material claim to source spans"],
        ["Retrieve actor map first", "Use temporal graph before recommendation", "Surface disputed or stale nodes"],
        "What is the actual situation, what changed, and what is still uncertain?",
        now,
    ),
    Row(
        "demo:ontology:tool",
        "TOOL",
        "Reusable Expert Method Ontology",
        ["Method", "Step", "InputPrimitive", "FailureMode", "QualityGate", "ReviewerRole"],
        ["method_version_from", "step_order", "review_cycle", "deprecation_time"],
        ["step_improves_quality", "failure_mode_invalidates_output", "quality_gate_blocks_publication"],
        ["Name assumptions", "Name failure modes", "Do not use method if required primitive is missing"],
        ["Constrain answer structure", "Force mechanism naming", "Stop when evidence is insufficient"],
        "Which expert method should guide the agent and what does it forbid?",
        now,
    ),
    Row(
        "demo:ontology:output",
        "OUTPUT",
        "Reviewed Output Ontology",
        ["Output", "Section", "Claim", "Citation", "Caveat", "ReviewVerdict", "PublicationGate"],
        ["drafted_at", "reviewed_at", "valid_through", "publication_deadline"],
        ["citation_supports_claim", "review_verdict_blocks_publication", "caveat_changes_recommendation_strength"],
        ["No unsupported material claim", "No publication when review_state is needs_review", "Every recommendation needs a source or caveat"],
        ["Reuse only approved outputs", "Carry caveats into follow-up answers", "Show review state in PRAXIS"],
        "What should a finished answer look like, and what must block it from shipping?",
        now,
    ),
]

spark.createDataFrame(
    ontology_rows,
    "ontology_card_id string, capsule_kind string, ontology_name string, primitive_classes array<string>, temporal_primitives array<string>, causal_primitives array<string>, review_rules array<string>, agent_control_rules array<string>, example_question string, created_at timestamp",
).write.mode("append").saveAsTable(f"{catalog}.capsule_gold.ontology_design_cards")

# COMMAND ----------

source_rows = {
    row.capsule_kind: row
    for row in spark.table(f"{catalog}.capsule_bronze.raw_inputs")
    .where("input_id LIKE 'demo:%'")
    .select("capsule_kind", "title", "content")
    .collect()
}

prompts = [
    Row(
        "demo:ai:user-profile",
        "USER",
        "guided_builder_and_private_context",
        source_rows["USER"].content,
        model_endpoint,
        "Extract private user preferences, voice constraints, review standards, privacy limits, and PRAXIS agent controls. Return compact JSON with keys: preferences, voice, review_standards, privacy_boundaries, agent_controls.",
        now,
    ),
    Row(
        "demo:ai:situation-ontology",
        "SITUATION",
        "documents_and_policy_sources",
        source_rows["SITUATION"].content,
        model_endpoint,
        "Extract actors, institutions, policy instruments, temporal facts, causal assumptions, open questions, and ontology primitives. Return compact JSON with keys: actors, policy_instruments, temporal_relations, causal_relations, open_questions, ontology_terms.",
        now,
    ),
    Row(
        "demo:ai:tool-method",
        "TOOL",
        "expert_method_and_tacit_knowledge",
        source_rows["TOOL"].content,
        model_endpoint,
        "Turn this expert method into a reusable Tool Context Capsule. Return compact JSON with keys: method_steps, required_inputs, failure_modes, quality_gates, agent_control_rules.",
        now,
    ),
    Row(
        "demo:ai:output-review",
        "OUTPUT",
        "draft_output_and_review_gate",
        source_rows["OUTPUT"].content,
        model_endpoint,
        "Review this output capsule candidate. Return compact JSON with keys: usable_sections, missing_evidence, publication_gates, recommended_revisions, praxis_attachment_policy.",
        now,
    ),
]

spark.createDataFrame(
    prompts,
    "run_id string, capsule_kind string, source_surface string, source_text string, model_endpoint string, extraction_task string, created_at timestamp",
).createOrReplaceTempView("tacitus_ai_showroom_prompts")

spark.sql(f"""
INSERT INTO {catalog}.capsule_gold.ai_extraction_runs
SELECT
  run_id,
  capsule_kind,
  source_surface,
  source_text,
  model_endpoint,
  extraction_task,
  CONCAT(
    'You are DIALECTICA, the TACITUS context-capsule extraction engine. ',
    'Use source-bounded analysis only. Do not invent facts. ',
    extraction_task,
    '\\n\\nSOURCE:\\n',
    source_text
  ) AS prompt_text,
  ai_query(
    '{model_endpoint_sql}',
    CONCAT(
      'You are DIALECTICA, the TACITUS context-capsule extraction engine. ',
      'Use source-bounded analysis only. Do not invent facts. ',
      extraction_task,
      '\\n\\nSOURCE:\\n',
      source_text
    )
  ) AS ai_result,
  'ai_generated_review_required' AS review_state,
  created_at
FROM tacitus_ai_showroom_prompts
""")

# COMMAND ----------

narrative_rows = [
    Row("demo:narrative:one", "Opening", "TACITUS turns context into governed data products.", "A Context Capsule is a bounded, typed, reviewable unit of context. Databricks stores the evidence, ontology, temporal graph, causal graph, AI extraction output, quality metrics, and PRAXIS export contract.", f"{catalog}.capsule_gold.dashboard_capsule_deep_dive", "Show me the USER, SITUATION, TOOL, and OUTPUT capsules that PRAXIS can attach.", now),
    Row("demo:narrative:two", "AI Extraction", "AI runs inside Databricks over governed Delta data.", "The workflow calls a Databricks-hosted foundation model with ai_query, stores the prompt and output in Delta, and marks it review_required so AI never silently becomes canonical.", f"{catalog}.capsule_gold.dashboard_ai_extraction_lab", "Extract ontology primitives and agent controls from this source.", now),
    Row("demo:narrative:three", "Ontology", "Each capsule kind has its own ontology, but the factory is generic.", "USER, SITUATION, TOOL, and OUTPUT capsules share the same medallion backbone while each carries its own primitive classes, temporal model, causal model, review rules, and agent controls.", f"{catalog}.capsule_gold.dashboard_ontology_showroom", "What ontology should structure this capsule?", now),
    Row("demo:narrative:four", "PRAXIS Boundary", "Databricks produces reviewable intelligence; PRAXIS decides what becomes canonical.", "The export layer creates PRAXIS context packs, but review state and publication gates travel with the capsule. This is the governance boundary that makes the company fundable.", f"{catalog}.capsule_exports.context_pack_exports", "Attach approved context to PRAXIS without publishing unreviewed claims.", now),
]

spark.createDataFrame(
    narrative_rows,
    "narrative_id string, section string, headline string, body string, proof_table string, demo_prompt string, created_at timestamp",
).write.mode("append").saveAsTable(f"{catalog}.capsule_gold.capsule_showroom_narrative")

# COMMAND ----------

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_showcase_click_path AS
SELECT step_id, surface, click_path, what_to_show, talk_track, proof_object
FROM {catalog}.capsule_gold.showcase_click_path
ORDER BY step_id
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_platform_story AS
SELECT databricks_surface, tacitus_use, why_it_matters, proof_object, maturity
FROM {catalog}.capsule_gold.databricks_capability_map
ORDER BY
  CASE maturity WHEN 'working' THEN 1 WHEN 'ready_next' THEN 2 ELSE 3 END,
  databricks_surface
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_ai_extraction_lab AS
SELECT
  run_id,
  capsule_kind,
  source_surface,
  model_endpoint,
  extraction_task,
  ai_result,
  review_state,
  created_at
FROM {catalog}.capsule_gold.ai_extraction_runs
WHERE run_id LIKE 'demo:%'
ORDER BY capsule_kind
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_ontology_showroom AS
SELECT
  capsule_kind,
  ontology_name,
  primitive_classes,
  temporal_primitives,
  causal_primitives,
  review_rules,
  agent_control_rules,
  example_question
FROM {catalog}.capsule_gold.ontology_design_cards
WHERE ontology_card_id LIKE 'demo:%'
ORDER BY capsule_kind
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_capsule_deep_dive AS
SELECT
  m.capsule_kind,
  m.capsule_id,
  m.title,
  m.summary,
  m.review_state,
  m.source_count,
  m.claim_count,
  m.graph_node_count,
  m.graph_edge_count,
  o.ontology_name,
  o.primitive_classes,
  o.temporal_primitives,
  o.causal_primitives,
  e.model_endpoint,
  e.review_state AS ai_review_state,
  e.ai_result
FROM {catalog}.capsule_gold.capsule_manifests m
LEFT JOIN {catalog}.capsule_gold.ontology_design_cards o
  ON m.capsule_kind = o.capsule_kind
LEFT JOIN {catalog}.capsule_gold.ai_extraction_runs e
  ON m.capsule_kind = e.capsule_kind
WHERE m.capsule_id LIKE 'cap_%_demo'
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_showroom_narrative AS
SELECT section, headline, body, proof_table, demo_prompt
FROM {catalog}.capsule_gold.capsule_showroom_narrative
WHERE narrative_id LIKE 'demo:%'
ORDER BY narrative_id
""")

# COMMAND ----------

display(spark.table(f"{catalog}.capsule_gold.dashboard_showcase_click_path"))
display(spark.table(f"{catalog}.capsule_gold.dashboard_platform_story"))
display(spark.table(f"{catalog}.capsule_gold.dashboard_ai_extraction_lab"))
display(spark.table(f"{catalog}.capsule_gold.dashboard_ontology_showroom"))
display(spark.table(f"{catalog}.capsule_gold.dashboard_capsule_deep_dive"))

print(
    "Prepared TACITUS AI showroom with Databricks AI Functions "
    f"using model_endpoint={model_endpoint} in catalog={catalog}."
)

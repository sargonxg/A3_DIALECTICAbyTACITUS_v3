# Databricks notebook source
# MAGIC %md
# MAGIC # Prepare TACITUS Capsule Operations Agent
# MAGIC
# MAGIC Creates the Databricks-native control plane for an agent/MCP layer:
# MAGIC
# MAGIC - a tool registry describing what the agent can safely do;
# MAGIC - request and response logs for interactive guidance;
# MAGIC - MCP-shaped tool contracts that can be exposed by a Databricks App;
# MAGIC - a feedback queue for Codex/DIALECTICA/PRAXIS engineering improvements;
# MAGIC - dashboard views for the Databricks App and SQL Editor.
# MAGIC
# MAGIC This notebook keeps the trust boundary explicit: the agent may explain,
# MAGIC inspect, plan, trigger Databricks jobs, and write feedback rows. It does
# MAGIC not silently publish capsules or write to PRAXIS canonical stores.

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
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.agent_tool_registry (
  tool_name STRING NOT NULL,
  tool_kind STRING,
  databricks_surface STRING,
  description STRING,
  input_schema_json STRING,
  output_contract_json STRING,
  safety_level STRING,
  review_policy STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.agent_requests (
  request_id STRING NOT NULL,
  requested_by STRING,
  user_prompt STRING,
  inferred_intent STRING,
  requested_action STRING,
  status STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.agent_responses (
  response_id STRING NOT NULL,
  request_id STRING,
  model_endpoint STRING,
  response_text STRING,
  recommended_tools_json STRING,
  follow_up_actions_json STRING,
  codex_feedback_json STRING,
  review_state STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.agent_tool_invocations (
  invocation_id STRING NOT NULL,
  request_id STRING,
  tool_name STRING,
  parameters_json STRING,
  status STRING,
  result_json STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.agent_feedback_queue (
  feedback_id STRING NOT NULL,
  source_surface STRING,
  target_agent STRING,
  severity STRING,
  finding STRING,
  recommended_change STRING,
  linked_object STRING,
  status STRING,
  created_at TIMESTAMP
) USING DELTA
""")

spark.sql(f"""
CREATE TABLE IF NOT EXISTS {catalog}.capsule_gold.mcp_tool_contracts (
  mcp_tool_name STRING NOT NULL,
  exposed_by_app STRING,
  description STRING,
  input_schema_json STRING,
  output_schema_json STRING,
  databricks_permissions STRING,
  backing_tables ARRAY<STRING>,
  side_effect_policy STRING,
  created_at TIMESTAMP
) USING DELTA
""")

for table, column in [
    ("capsule_gold.agent_tool_registry", "tool_name"),
    ("capsule_gold.agent_requests", "request_id"),
    ("capsule_gold.agent_responses", "response_id"),
    ("capsule_gold.agent_tool_invocations", "invocation_id"),
    ("capsule_gold.agent_feedback_queue", "feedback_id"),
    ("capsule_gold.mcp_tool_contracts", "mcp_tool_name"),
]:
    spark.sql(f"DELETE FROM {catalog}.{table} WHERE {column} LIKE 'demo:%' OR {column} LIKE 'tacitus_%'")

# COMMAND ----------

tools = [
    Row(
        "tacitus_explain_workspace",
        "read_only",
        "Catalog / Workflows / Apps / SQL",
        "Explain where to click in Databricks and what each surface proves.",
        json.dumps({"question": "string"}, sort_keys=True),
        json.dumps({"walkthrough": "ordered steps", "proof_objects": "tables/jobs/apps"}, sort_keys=True),
        "safe",
        "May answer directly from dashboard views.",
        now,
    ),
    Row(
        "tacitus_summarize_capsule_factory",
        "read_only",
        "Unity Catalog",
        "Summarize current capsule portfolio, graph scale, review state, and AI extraction state.",
        json.dumps({"scope": "portfolio|graph|ai|all"}, sort_keys=True),
        json.dumps({"summary": "string", "counts": "object", "risks": "array"}, sort_keys=True),
        "safe",
        "May query curated dashboard views only.",
        now,
    ),
    Row(
        "tacitus_plan_capsule",
        "ai_guidance",
        "AI Functions",
        "Turn a user request into a proposed USER/SITUATION/TOOL/OUTPUT capsule build plan.",
        json.dumps({"user_prompt": "string", "capsule_kind": "optional string"}, sort_keys=True),
        json.dumps({"plan": "string", "recommended_sources": "array", "review_gates": "array"}, sort_keys=True),
        "review_required",
        "AI-generated plans are non-canonical until accepted by a human.",
        now,
    ),
    Row(
        "tacitus_run_capsule_factory",
        "side_effect",
        "Lakeflow Jobs",
        "Trigger the TACITUS Context Capsule Builder Showcase workflow.",
        json.dumps({"reason": "string"}, sort_keys=True),
        json.dumps({"run_id": "string", "run_page_url": "string", "status": "string"}, sort_keys=True),
        "operator_action",
        "Requires an explicit button press or MCP tool call; all invocations are logged.",
        now,
    ),
    Row(
        "tacitus_get_latest_run",
        "read_only",
        "Lakeflow Jobs",
        "Fetch latest workflow run state and run URL.",
        json.dumps({"limit": "integer"}, sort_keys=True),
        json.dumps({"latest_runs": "array"}, sort_keys=True),
        "safe",
        "May read job state.",
        now,
    ),
    Row(
        "tacitus_write_codex_feedback",
        "feedback",
        "Unity Catalog",
        "Write improvement feedback for Codex, DIALECTICA, PRAXIS, or Databricks bundle work.",
        json.dumps({"finding": "string", "recommended_change": "string", "severity": "low|medium|high"}, sort_keys=True),
        json.dumps({"feedback_id": "string", "status": "queued"}, sort_keys=True),
        "review_required",
        "Feedback is a queue item, not an automatic code change.",
        now,
    ),
]

spark.createDataFrame(
    tools,
    "tool_name string, tool_kind string, databricks_surface string, description string, input_schema_json string, output_contract_json string, safety_level string, review_policy string, created_at timestamp",
).write.mode("append").saveAsTable(f"{catalog}.capsule_gold.agent_tool_registry")

mcp_contracts = [
    Row(
        "tacitus_explain_workspace",
        "mcp-tacitus-capsules-dev",
        "Return the Databricks workspace walkthrough and explain which objects prove each part of the capsule factory.",
        json.dumps({"type": "object", "properties": {"question": {"type": "string"}}}, sort_keys=True),
        json.dumps({"type": "object", "properties": {"answer": {"type": "string"}, "proof_tables": {"type": "array"}}}, sort_keys=True),
        "SELECT on curated showroom views",
        [f"{catalog}.capsule_gold.dashboard_showcase_click_path", f"{catalog}.capsule_gold.dashboard_platform_story"],
        "read_only",
        now,
    ),
    Row(
        "tacitus_run_capsule_factory",
        "mcp-tacitus-capsules-dev",
        "Trigger the Databricks workflow and return a run URL.",
        json.dumps({"type": "object", "properties": {"reason": {"type": "string"}}}, sort_keys=True),
        json.dumps({"type": "object", "properties": {"run_id": {"type": "string"}, "run_page_url": {"type": "string"}}}, sort_keys=True),
        "CAN_MANAGE_RUN on Lakeflow job",
        [f"{catalog}.capsule_gold.agent_tool_invocations"],
        "side_effect_logged",
        now,
    ),
    Row(
        "tacitus_write_codex_feedback",
        "mcp-tacitus-capsules-dev",
        "Queue feedback for Codex/agents to improve bundle structure, docs, app UX, or capsule ontology quality.",
        json.dumps({"type": "object", "properties": {"finding": {"type": "string"}, "recommended_change": {"type": "string"}, "severity": {"type": "string"}}}, sort_keys=True),
        json.dumps({"type": "object", "properties": {"feedback_id": {"type": "string"}, "status": {"type": "string"}}}, sort_keys=True),
        "MODIFY on feedback queue",
        [f"{catalog}.capsule_gold.agent_feedback_queue"],
        "review_queue_only",
        now,
    ),
]

spark.createDataFrame(
    mcp_contracts,
    "mcp_tool_name string, exposed_by_app string, description string, input_schema_json string, output_schema_json string, databricks_permissions string, backing_tables array<string>, side_effect_policy string, created_at timestamp",
).write.mode("append").saveAsTable(f"{catalog}.capsule_gold.mcp_tool_contracts")

# COMMAND ----------

seed_prompt = "Guide me through the TACITUS Databricks workspace and tell me what to improve next."
seed_request = Row(
    "demo:agent-request:showcase-guide",
    "giulio@tacitus.me",
    seed_prompt,
    "showcase_guidance_and_improvement_feedback",
    "explain_workspace_and_propose_next_steps",
    "answered",
    now,
)

spark.createDataFrame(
    [seed_request],
    "request_id string, requested_by string, user_prompt string, inferred_intent string, requested_action string, status string, created_at timestamp",
).write.mode("append").saveAsTable(f"{catalog}.capsule_gold.agent_requests")

prompt = f"""
You are the TACITUS Capsule Operations Agent running inside Databricks.
You guide users through a governed Context Capsule Factory for PRAXIS.

Use these facts:
- Workflow: [dev] TACITUS Context Capsule Builder Showcase.
- App: tacitus-capsule-builder-dev.
- Catalog: {catalog}.
- The factory creates USER, SITUATION, TOOL, and OUTPUT capsules.
- It includes bronze ingestion, silver ontology/semantic/temporal/causal graph,
  gold capsule manifests, AI extraction runs, evals, and PRAXIS exports.
- Nothing writes directly into PRAXIS canonical state; exports are reviewable.

Answer the user prompt with:
1. Where to click in Databricks.
2. What the agent can do.
3. What the MCP tools expose.
4. What Codex/agents should improve next.

User prompt:
{seed_prompt}
""".replace("'", "''")

response_df = spark.sql(f"""
SELECT ai_query('{model_endpoint_sql}', '{prompt}') AS response_text
""")
response_text = response_df.collect()[0]["response_text"]

spark.createDataFrame(
    [
        Row(
            "demo:agent-response:showcase-guide",
            "demo:agent-request:showcase-guide",
            model_endpoint,
            response_text,
            json.dumps(["tacitus_explain_workspace", "tacitus_summarize_capsule_factory", "tacitus_write_codex_feedback"], sort_keys=True),
            json.dumps(["Open App Agent tab", "Review feedback queue", "Promote MCP app when ready"], sort_keys=True),
            json.dumps(
                {
                    "next_code_changes": [
                        "Add production AI Search index when spend is approved",
                        "Add real connector adapters behind explicit consent",
                        "Add MLflow tracing/evals for agent interactions",
                    ]
                },
                sort_keys=True,
            ),
            "ai_generated_review_required",
            now,
        )
    ],
    "response_id string, request_id string, model_endpoint string, response_text string, recommended_tools_json string, follow_up_actions_json string, codex_feedback_json string, review_state string, created_at timestamp",
).write.mode("append").saveAsTable(f"{catalog}.capsule_gold.agent_responses")

feedback_rows = [
    Row("demo:feedback:ai-search-index", "capsule_ops_agent", "codex", "medium", "The AI-search-ready Delta table exists but no live AI Search index is created.", "Create the AI Search Delta Sync index after spend approval and add it as an app resource.", f"{catalog}.capsule_gold.ai_search_corpus", "queued", now),
    Row("demo:feedback:mlflow-tracing", "capsule_ops_agent", "codex", "medium", "Agent responses are stored in Delta but not yet traced through MLflow.", "Add MLflow tracing/eval integration for prompt quality, tool calls, and review outcomes.", f"{catalog}.capsule_gold.agent_responses", "queued", now),
    Row("demo:feedback:real-connectors", "capsule_ops_agent", "praxis", "high", "Connector requests are modeled but still synthetic.", "Implement consent-first Gmail, upload, PRAXIS history, and Ladybug/Postgres adapters behind review gates.", f"{catalog}.capsule_bronze.source_connector_requests", "queued", now),
]

spark.createDataFrame(
    feedback_rows,
    "feedback_id string, source_surface string, target_agent string, severity string, finding string, recommended_change string, linked_object string, status string, created_at timestamp",
).write.mode("append").saveAsTable(f"{catalog}.capsule_gold.agent_feedback_queue")

# COMMAND ----------

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_capsule_ops_agent AS
SELECT
  r.request_id,
  r.requested_by,
  r.user_prompt,
  r.inferred_intent,
  r.status,
  a.model_endpoint,
  a.response_text,
  a.recommended_tools_json,
  a.follow_up_actions_json,
  a.codex_feedback_json,
  a.review_state,
  r.created_at
FROM {catalog}.capsule_gold.agent_requests r
LEFT JOIN {catalog}.capsule_gold.agent_responses a
  ON r.request_id = a.request_id
ORDER BY r.created_at DESC
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_agent_tool_registry AS
SELECT
  tool_name,
  tool_kind,
  databricks_surface,
  description,
  safety_level,
  review_policy
FROM {catalog}.capsule_gold.agent_tool_registry
ORDER BY safety_level, tool_name
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_mcp_contracts AS
SELECT
  mcp_tool_name,
  exposed_by_app,
  description,
  databricks_permissions,
  backing_tables,
  side_effect_policy
FROM {catalog}.capsule_gold.mcp_tool_contracts
ORDER BY mcp_tool_name
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_agent_feedback_queue AS
SELECT
  feedback_id,
  source_surface,
  target_agent,
  severity,
  finding,
  recommended_change,
  linked_object,
  status,
  created_at
FROM {catalog}.capsule_gold.agent_feedback_queue
ORDER BY
  CASE severity WHEN 'high' THEN 1 WHEN 'medium' THEN 2 ELSE 3 END,
  created_at DESC
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_agent_tool_invocations AS
SELECT invocation_id, request_id, tool_name, parameters_json, status, result_json, created_at
FROM {catalog}.capsule_gold.agent_tool_invocations
ORDER BY created_at DESC
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_showroom_console AS
SELECT
  'click_path' AS section,
  CAST(step_id AS STRING) AS item_id,
  CONCAT(surface, ': ', what_to_show) AS summary,
  to_json(named_struct(
    'surface', surface,
    'click_path', click_path,
    'talk_track', talk_track,
    'proof_object', proof_object
  )) AS payload_json
FROM {catalog}.capsule_gold.dashboard_showcase_click_path
UNION ALL
SELECT
  'platform_story',
  databricks_surface,
  tacitus_use,
  to_json(named_struct(
    'why_it_matters', why_it_matters,
    'proof_object', proof_object,
    'maturity', maturity
  ))
FROM {catalog}.capsule_gold.dashboard_platform_story
UNION ALL
SELECT
  'narrative',
  section,
  headline,
  to_json(named_struct(
    'body', body,
    'proof_table', proof_table,
    'demo_prompt', demo_prompt
  ))
FROM {catalog}.capsule_gold.dashboard_showroom_narrative
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_builder_console AS
SELECT
  'session' AS section,
  session_id AS item_id,
  user_prompt AS summary,
  to_json(named_struct(
    'requested_capsule_kind', requested_capsule_kind,
    'requested_by', requested_by,
    'target_surface', target_surface,
    'consent_profile', consent_profile,
    'status', status
  )) AS payload_json
FROM {catalog}.capsule_bronze.capsule_builder_sessions
WHERE session_id LIKE 'demo:%'
UNION ALL
SELECT
  'interview_turn',
  turn_id,
  message,
  to_json(named_struct(
    'session_id', session_id,
    'speaker', speaker,
    'captured_signal_kind', captured_signal_kind,
    'proposed_capsule_field', proposed_capsule_field,
    'confidence', confidence
  ))
FROM {catalog}.capsule_bronze.builder_interview_turns
WHERE session_id LIKE 'demo:%'
UNION ALL
SELECT
  'connector_request',
  connector_request_id,
  connector_label,
  to_json(named_struct(
    'session_id', session_id,
    'connector_kind', connector_kind,
    'data_scope', data_scope,
    'status', status,
    'consent_required', consent_required,
    'notes', notes
  ))
FROM {catalog}.capsule_bronze.source_connector_requests
WHERE session_id LIKE 'demo:%'
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_graph_console AS
SELECT
  'graph_partition' AS section,
  capsule_id AS item_id,
  CONCAT(capsule_kind, ' / ', ontology_id) AS summary,
  to_json(named_struct(
    'node_count', node_count,
    'edge_count', edge_count,
    'review_state', review_state,
    'materialization_policy', materialization_policy
  )) AS payload_json
FROM {catalog}.capsule_gold.dashboard_graph_scale
UNION ALL
SELECT
  'temporal_relation',
  temporal_relation_id,
  relation_type,
  to_json(named_struct(
    'capsule_id', capsule_id,
    'subject_id', subject_id,
    'object_id', object_id,
    'valid_from', valid_from,
    'known_at', known_at,
    'valid_until', valid_until,
    'allen_relation', allen_relation,
    'uncertainty', uncertainty,
    'review_state', review_state
  ))
FROM {catalog}.capsule_silver.temporal_relations
WHERE capsule_id LIKE 'cap_%_demo'
UNION ALL
SELECT
  'causal_relation',
  causal_relation_id,
  mechanism,
  to_json(named_struct(
    'capsule_id', capsule_id,
    'cause_id', cause_id,
    'effect_id', effect_id,
    'confidence', confidence,
    'assumptions', assumptions,
    'review_state', review_state
  ))
FROM {catalog}.capsule_silver.causal_relations
WHERE capsule_id LIKE 'cap_%_demo'
""")

spark.sql(f"""
CREATE OR REPLACE VIEW {catalog}.capsule_gold.dashboard_agent_console AS
SELECT
  'agent_response' AS section,
  request_id AS item_id,
  user_prompt AS summary,
  to_json(named_struct(
    'requested_by', requested_by,
    'inferred_intent', inferred_intent,
    'status', status,
    'model_endpoint', model_endpoint,
    'response_text', response_text,
    'recommended_tools_json', recommended_tools_json,
    'follow_up_actions_json', follow_up_actions_json,
    'codex_feedback_json', codex_feedback_json,
    'review_state', review_state
  )) AS payload_json
FROM {catalog}.capsule_gold.dashboard_capsule_ops_agent
UNION ALL
SELECT
  'tool_registry',
  tool_name,
  description,
  to_json(named_struct(
    'tool_kind', tool_kind,
    'databricks_surface', databricks_surface,
    'safety_level', safety_level,
    'review_policy', review_policy
  ))
FROM {catalog}.capsule_gold.dashboard_agent_tool_registry
UNION ALL
SELECT
  'mcp_contract',
  mcp_tool_name,
  description,
  to_json(named_struct(
    'exposed_by_app', exposed_by_app,
    'databricks_permissions', databricks_permissions,
    'backing_tables', backing_tables,
    'side_effect_policy', side_effect_policy
  ))
FROM {catalog}.capsule_gold.dashboard_mcp_contracts
UNION ALL
SELECT
  'feedback',
  feedback_id,
  finding,
  to_json(named_struct(
    'source_surface', source_surface,
    'target_agent', target_agent,
    'severity', severity,
    'recommended_change', recommended_change,
    'linked_object', linked_object,
    'status', status
  ))
FROM {catalog}.capsule_gold.dashboard_agent_feedback_queue
UNION ALL
SELECT
  'tool_invocation',
  invocation_id,
  tool_name,
  to_json(named_struct(
    'request_id', request_id,
    'parameters_json', parameters_json,
    'status', status,
    'result_json', result_json,
    'created_at', created_at
  ))
FROM {catalog}.capsule_gold.dashboard_agent_tool_invocations
""")

# COMMAND ----------

display(spark.table(f"{catalog}.capsule_gold.dashboard_capsule_ops_agent"))
display(spark.table(f"{catalog}.capsule_gold.dashboard_agent_tool_registry"))
display(spark.table(f"{catalog}.capsule_gold.dashboard_mcp_contracts"))
display(spark.table(f"{catalog}.capsule_gold.dashboard_agent_feedback_queue"))

print(f"Prepared TACITUS Capsule Operations Agent control plane in catalog={catalog}.")

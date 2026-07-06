import os
import json
import uuid
from datetime import datetime, timezone
from typing import Any

import pandas as pd
import streamlit as st
from databricks import sql
from databricks.sdk import WorkspaceClient
from databricks.sdk.core import Config


CATALOG = os.getenv("TACITUS_CATALOG", "dialectica")
WAREHOUSE_ID = os.getenv("DATABRICKS_WAREHOUSE_ID", "")
CAPSULE_JOB_ID = os.getenv("DATABRICKS_CAPSULE_JOB_ID", "1123194333821498")
MODEL_ENDPOINT = os.getenv("TACITUS_MODEL_ENDPOINT", "databricks-gpt-5-mini")


@st.cache_resource
def get_connection() -> Any:
    cfg = Config()
    host = cfg.host.replace("https://", "").replace("http://", "")
    http_path = f"/sql/1.0/warehouses/{WAREHOUSE_ID}"
    return sql.connect(
        server_hostname=host,
        http_path=http_path,
        credentials_provider=lambda: cfg.authenticate,
        _use_arrow_native_complex_types=False,
    )


@st.cache_resource
def get_workspace_client() -> WorkspaceClient:
    return WorkspaceClient()


@st.cache_data(ttl=60)
def query(sql_text: str) -> pd.DataFrame:
    with get_connection().cursor() as cursor:
        cursor.execute(sql_text)
        return cursor.fetchall_arrow().to_pandas()


def execute_sql(sql_text: str) -> None:
    with get_connection().cursor() as cursor:
        cursor.execute(sql_text)


def sql_literal(value: Any) -> str:
    if value is None:
        return "NULL"
    return "'" + str(value).replace("'", "''") + "'"


def show_table(label: str, sql_text: str) -> None:
    st.subheader(label)
    try:
        st.dataframe(query(sql_text), use_container_width=True, hide_index=True)
    except Exception as exc:
        st.error(f"Could not load {label}: {exc}")


def write_agent_request(prompt: str, intent: str, action: str, status: str) -> str:
    request_id = f"app:agent-request:{uuid.uuid4()}"
    execute_sql(
        f"""
        INSERT INTO {CATALOG}.capsule_gold.agent_requests
        VALUES (
          {sql_literal(request_id)},
          {sql_literal('databricks_app_user')},
          {sql_literal(prompt)},
          {sql_literal(intent)},
          {sql_literal(action)},
          {sql_literal(status)},
          current_timestamp()
        )
        """
    )
    return request_id


def write_agent_response(request_id: str, prompt: str) -> str:
    response_id = f"app:agent-response:{uuid.uuid4()}"
    context = query(
        f"""
        SELECT
          (SELECT COUNT(*) FROM {CATALOG}.capsule_gold.dashboard_capsule_portfolio) AS capsule_count,
          (SELECT COUNT(*) FROM {CATALOG}.capsule_gold.dashboard_ai_extraction_lab) AS ai_extraction_runs,
          (SELECT COUNT(*) FROM {CATALOG}.capsule_gold.dashboard_agent_console WHERE section = 'feedback') AS queued_feedback,
          (SELECT COUNT(*) FROM {CATALOG}.capsule_gold.dashboard_agent_console WHERE section = 'mcp_contract') AS mcp_tools
        """
    ).iloc[0].to_dict()
    agent_prompt = f"""
You are the TACITUS Capsule Operations Agent running inside Databricks.
Answer as an operator guide for the current Databricks workspace.

Current state:
{json.dumps(context, sort_keys=True)}

Rules:
- Explain what to click in Databricks when useful.
- Prefer the existing governed objects in catalog {CATALOG}.
- If the user asks to run the factory, tell them to use the run button.
- If the user asks for code improvements, emit concrete feedback for Codex.
- Never claim that PRAXIS canonical state was changed.
- Keep unreviewed AI output marked review-required.

User request:
{prompt}
"""
    result = query(
        f"""
        SELECT ai_query({sql_literal(MODEL_ENDPOINT)}, {sql_literal(agent_prompt)}) AS response_text
        """
    )
    response_text = result.iloc[0]["response_text"]
    execute_sql(
        f"""
        INSERT INTO {CATALOG}.capsule_gold.agent_responses
        VALUES (
          {sql_literal(response_id)},
          {sql_literal(request_id)},
          {sql_literal(MODEL_ENDPOINT)},
          {sql_literal(response_text)},
          {sql_literal(json.dumps(['tacitus_explain_workspace', 'tacitus_summarize_capsule_factory', 'tacitus_write_codex_feedback'], sort_keys=True))},
          {sql_literal(json.dumps(['Review suggested feedback', 'Run workflow only with explicit operator action'], sort_keys=True))},
          {sql_literal(json.dumps({'source': 'Databricks App Agent', 'prompt': prompt}, sort_keys=True))},
          {sql_literal('ai_generated_review_required')},
          current_timestamp()
        )
        """
    )
    return str(response_text)


def log_tool_invocation(tool_name: str, parameters: dict[str, Any], status: str, result: dict[str, Any]) -> None:
    execute_sql(
        f"""
        INSERT INTO {CATALOG}.capsule_gold.agent_tool_invocations
        VALUES (
          {sql_literal('app:tool-invocation:' + str(uuid.uuid4()))},
          {sql_literal(parameters.get('request_id'))},
          {sql_literal(tool_name)},
          {sql_literal(json.dumps(parameters, sort_keys=True))},
          {sql_literal(status)},
          {sql_literal(json.dumps(result, sort_keys=True))},
          current_timestamp()
        )
        """
    )


st.set_page_config(page_title="TACITUS Capsule Factory", layout="wide")

st.title("TACITUS Context Capsule Factory")
st.caption("Databricks control room for PRAXIS-ready Context Capsules")

if not WAREHOUSE_ID:
    st.error("DATABRICKS_WAREHOUSE_ID is not configured for this app.")
    st.stop()

st.markdown(
    """
This Databricks App shows the complete capsule-factory proof:
CLI-deployed workflow, Unity Catalog layers, dashboard-ready marts, graph scale,
reviewable proposals, Databricks AI extraction, ontology design, AI-search-ready
corpus, and PRAXIS agent guidance frames.
"""
)

kpis = query(f"SELECT metric, value FROM {CATALOG}.capsule_gold.dashboard_kpis ORDER BY metric")
kpi_cols = st.columns(min(len(kpis), 7))
for idx, row in kpis.iterrows():
    with kpi_cols[idx % len(kpi_cols)]:
        st.metric(row["metric"].replace("_", " ").title(), int(row["value"]))

tabs = st.tabs(
    [
        "Agent / MCP",
        "Showroom",
        "AI Extraction Lab",
        "Ontology",
        "Portfolio",
        "Builder",
        "Graph",
        "Review",
        "Agent Guidance",
        "AI Search Corpus",
        "Exports",
    ]
)

with tabs[0]:
    st.subheader("TACITUS Capsule Operations Agent")
    st.caption(
        "Ask the Databricks-native agent to explain the workspace, inspect the "
        "capsule factory, plan extraction work, or queue feedback for Codex."
    )

    with st.form("agent_prompt_form"):
        agent_prompt = st.text_area(
            "Agent request",
            value="Explain how to demo this workspace and what we should improve next.",
            height=120,
        )
        submitted = st.form_submit_button("Ask Agent")

    if submitted and agent_prompt.strip():
        try:
            request_id = write_agent_request(
                agent_prompt.strip(),
                "interactive_databricks_agent_request",
                "answer_with_ai_query",
                "answered",
            )
            answer = write_agent_response(request_id, agent_prompt.strip())
            st.success("Agent response stored in Unity Catalog.")
            st.write(answer)
            st.cache_data.clear()
        except Exception as exc:
            st.error(f"Agent request failed: {exc}")

    st.subheader("Workflow Control")
    st.caption(
        "This button triggers the Databricks workflow through the app service "
        "principal's Lakeflow Jobs resource. The invocation is logged."
    )
    if st.button("Run Capsule Factory Workflow"):
        try:
            request_id = write_agent_request(
                "Run the TACITUS Context Capsule Builder Showcase workflow.",
                "operator_workflow_trigger",
                "run_capsule_factory",
                "submitted",
            )
            run = get_workspace_client().jobs.run_now(job_id=int(CAPSULE_JOB_ID))
            cfg = Config()
            run_url = f"{cfg.host}/?o=7474658425841042#job/{CAPSULE_JOB_ID}/run/{run.run_id}"
            result = {"run_id": str(run.run_id), "run_page_url": run_url}
            log_tool_invocation(
                "tacitus_run_capsule_factory",
                {"request_id": request_id, "job_id": CAPSULE_JOB_ID},
                "submitted",
                result,
            )
            st.success("Workflow submitted.")
            st.markdown(f"[Open run]({run_url})")
            st.cache_data.clear()
        except Exception as exc:
            st.error(f"Could not run workflow: {exc}")

    with st.form("feedback_form"):
        st.subheader("Queue Feedback For Codex / Agents")
        finding = st.text_area("Finding", height=80)
        recommended_change = st.text_area("Recommended change", height=80)
        severity = st.selectbox("Severity", ["medium", "low", "high"])
        feedback_submitted = st.form_submit_button("Queue Feedback")

    if feedback_submitted and finding.strip() and recommended_change.strip():
        try:
            feedback_id = f"app:feedback:{uuid.uuid4()}"
            execute_sql(
                f"""
                INSERT INTO {CATALOG}.capsule_gold.agent_feedback_queue
                VALUES (
                  {sql_literal(feedback_id)},
                  {sql_literal('databricks_app_agent')},
                  {sql_literal('codex')},
                  {sql_literal(severity)},
                  {sql_literal(finding.strip())},
                  {sql_literal(recommended_change.strip())},
                  {sql_literal('tacitus-capsule-builder-dev')},
                  {sql_literal('queued')},
                  current_timestamp()
                )
                """
            )
            st.success(f"Queued feedback: {feedback_id}")
            st.cache_data.clear()
        except Exception as exc:
            st.error(f"Could not queue feedback: {exc}")

    show_table(
        "Agent Console",
        f"""
        SELECT section, item_id, summary, payload_json
        FROM {CATALOG}.capsule_gold.dashboard_agent_console
        LIMIT 20
        """,
    )

with tabs[1]:
    show_table(
        "Showroom Console",
        f"""
        SELECT section, item_id, summary, payload_json
        FROM {CATALOG}.capsule_gold.dashboard_showroom_console
        ORDER BY section
        """,
    )

with tabs[2]:
    st.info(
        "These rows are produced by Databricks AI Functions using ai_query over "
        "governed Delta inputs. The prompt and result are both stored for review."
    )
    show_table(
        "Databricks AI Extraction Runs",
        f"""
        SELECT run_id, capsule_kind, source_surface, model_endpoint,
               extraction_task, ai_result, review_state
        FROM {CATALOG}.capsule_gold.dashboard_ai_extraction_lab
        ORDER BY capsule_kind
        """,
    )

with tabs[3]:
    show_table(
        "Capsule Ontology Design Cards",
        f"""
        SELECT capsule_kind, ontology_name, primitive_classes,
               temporal_primitives, causal_primitives, review_rules,
               agent_control_rules, example_question
        FROM {CATALOG}.capsule_gold.dashboard_ontology_showroom
        ORDER BY capsule_kind
        """,
    )
    show_table(
        "Capsule Deep Dive: Ontology + AI Extraction + Manifest",
        f"""
        SELECT capsule_kind, capsule_id, title, review_state, source_count,
               claim_count, graph_node_count, graph_edge_count,
               ontology_name, primitive_classes, temporal_primitives,
               causal_primitives, model_endpoint, ai_review_state, ai_result
        FROM {CATALOG}.capsule_gold.dashboard_capsule_deep_dive
        ORDER BY capsule_kind
        """,
    )

with tabs[4]:
    show_table(
        "Capsule Portfolio",
        f"""
        SELECT capsule_kind, capsule_id, title, review_state, source_count,
               claim_count, graph_node_count, graph_edge_count,
               citation_precision, source_coverage, unsupported_claim_rate
        FROM {CATALOG}.capsule_gold.dashboard_capsule_portfolio
        ORDER BY capsule_kind
        """,
    )

with tabs[5]:
    show_table(
        "Guided Builder Console",
        f"""
        SELECT section, item_id, summary, payload_json
        FROM {CATALOG}.capsule_gold.dashboard_builder_console
        ORDER BY section, item_id
        """,
    )

with tabs[6]:
    show_table(
        "Graph / Temporal / Causal Console",
        f"""
        SELECT section, item_id, summary, payload_json
        FROM {CATALOG}.capsule_gold.dashboard_graph_console
        ORDER BY section, item_id
        """,
    )

with tabs[7]:
    show_table(
        "Reviewable Improvement Proposals",
        f"""
        SELECT capsule_kind, capsule_id, target_layer, proposal_kind, title, review_state
        FROM {CATALOG}.capsule_gold.dashboard_review_workbench
        ORDER BY capsule_kind, target_layer
        """,
    )

with tabs[8]:
    show_table(
        "PRAXIS Agent Guidance Frames",
        f"""
        SELECT capsule_kind, capsule_id, target_agent, review_required,
               deterministic_controls_json, retrieval_contract_json,
               allowed_generation_space_json
        FROM {CATALOG}.capsule_gold.dashboard_agent_guidance
        ORDER BY capsule_kind
        """,
    )

with tabs[9]:
    st.info(
        "This Delta table is ready to back an AI Search Delta Sync index. "
        "Use corpus_id as the primary key and body as the text column."
    )
    show_table(
        "AI-Search-Ready Corpus",
        f"""
        SELECT corpus_id, capsule_id, source_layer, title, review_state,
               source_refs, body
        FROM {CATALOG}.capsule_gold.ai_search_corpus
        ORDER BY source_layer, corpus_id
        LIMIT 100
        """,
    )

with tabs[10]:
    show_table(
        "PRAXIS Context-Pack Exports",
        f"""
        SELECT capsule_id, target_surface, export_status, context_pack_json
        FROM {CATALOG}.capsule_exports.context_pack_exports
        WHERE capsule_id LIKE 'cap_%_demo'
        ORDER BY target_surface, capsule_id
        """,
    )

st.divider()
st.caption(
    "Boundary: Databricks produces governed, reviewable capsule intelligence. "
    "PRAXIS decides what becomes canonical."
)

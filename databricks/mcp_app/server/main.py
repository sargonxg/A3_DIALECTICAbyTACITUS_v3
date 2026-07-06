import json
import os
import uuid
from typing import Any

from databricks import sql
from databricks.sdk import WorkspaceClient
from databricks.sdk.core import Config
from mcp.server.fastmcp import FastMCP


CATALOG = os.getenv("TACITUS_CATALOG", "dialectica")
WAREHOUSE_ID = os.getenv("DATABRICKS_WAREHOUSE_ID", "")
CAPSULE_JOB_ID = os.getenv("DATABRICKS_CAPSULE_JOB_ID", "1123194333821498")

mcp = FastMCP(
    "TACITUS Capsule Factory MCP",
    host="0.0.0.0",
    port=int(os.getenv("PORT", "8000")),
)


def _connection() -> Any:
    cfg = Config()
    host = cfg.host.replace("https://", "").replace("http://", "")
    return sql.connect(
        server_hostname=host,
        http_path=f"/sql/1.0/warehouses/{WAREHOUSE_ID}",
        credentials_provider=lambda: cfg.authenticate,
        _use_arrow_native_complex_types=False,
    )


def _query(sql_text: str) -> list[dict[str, Any]]:
    with _connection().cursor() as cursor:
        cursor.execute(sql_text)
        frame = cursor.fetchall_arrow().to_pandas()
    return json.loads(frame.to_json(orient="records", date_format="iso"))


def _execute(sql_text: str) -> None:
    with _connection().cursor() as cursor:
        cursor.execute(sql_text)


def _sql_literal(value: Any) -> str:
    if value is None:
        return "NULL"
    return "'" + str(value).replace("'", "''") + "'"


@mcp.tool()
def health() -> dict[str, Any]:
    """Confirm that the TACITUS Databricks MCP server is running."""
    return {
        "status": "ok",
        "catalog": CATALOG,
        "warehouse_configured": bool(WAREHOUSE_ID),
        "capsule_job_id": CAPSULE_JOB_ID,
    }


@mcp.tool()
def explain_workspace(question: str = "") -> dict[str, Any]:
    """Explain where to click in Databricks and which TACITUS proof objects to show."""
    click_path = _query(
        f"""
        SELECT step_id, surface, click_path, what_to_show, talk_track, proof_object
        FROM {CATALOG}.capsule_gold.dashboard_showcase_click_path
        ORDER BY step_id
        """
    )
    platform_story = _query(
        f"""
        SELECT databricks_surface, tacitus_use, why_it_matters, proof_object, maturity
        FROM {CATALOG}.capsule_gold.dashboard_platform_story
        ORDER BY maturity, databricks_surface
        """
    )
    return {
        "question": question,
        "answer": "Start with Workflows, then Catalog, SQL Editor, Apps, and the MCP contracts.",
        "click_path": click_path,
        "platform_story": platform_story,
    }


@mcp.tool()
def summarize_capsule_factory() -> dict[str, Any]:
    """Summarize the current capsule factory state, KPIs, AI runs, and feedback queue."""
    return {
        "kpis": _query(f"SELECT metric, value FROM {CATALOG}.capsule_gold.dashboard_kpis ORDER BY metric"),
        "capsules": _query(
            f"""
            SELECT capsule_kind, capsule_id, title, review_state, source_count,
                   claim_count, graph_node_count, graph_edge_count
            FROM {CATALOG}.capsule_gold.dashboard_capsule_portfolio
            ORDER BY capsule_kind
            """
        ),
        "ai_extractions": _query(
            f"""
            SELECT capsule_kind, model_endpoint, review_state
            FROM {CATALOG}.capsule_gold.dashboard_ai_extraction_lab
            ORDER BY capsule_kind
            """
        ),
        "queued_feedback": _query(
            f"""
            SELECT severity, finding, recommended_change, status
            FROM {CATALOG}.capsule_gold.dashboard_agent_feedback_queue
            WHERE status = 'queued'
            ORDER BY created_at DESC
            LIMIT 20
            """
        ),
    }


@mcp.tool()
def run_capsule_factory(reason: str) -> dict[str, Any]:
    """Trigger the TACITUS Context Capsule Builder Showcase workflow and log the tool invocation."""
    client = WorkspaceClient()
    run = client.jobs.run_now(job_id=int(CAPSULE_JOB_ID))
    cfg = Config()
    run_url = f"{cfg.host}/?o=7474658425841042#job/{CAPSULE_JOB_ID}/run/{run.run_id}"
    result = {
        "run_id": str(run.run_id),
        "run_page_url": run_url,
        "status": "submitted",
    }
    _execute(
        f"""
        INSERT INTO {CATALOG}.capsule_gold.agent_tool_invocations
        VALUES (
          {_sql_literal('mcp:tool-invocation:' + str(uuid.uuid4()))},
          {_sql_literal(None)},
          {_sql_literal('tacitus_run_capsule_factory')},
          {_sql_literal(json.dumps({'reason': reason}, sort_keys=True))},
          {_sql_literal('submitted')},
          {_sql_literal(json.dumps(result, sort_keys=True))},
          current_timestamp()
        )
        """
    )
    return result


@mcp.tool()
def write_codex_feedback(finding: str, recommended_change: str, severity: str = "medium") -> dict[str, Any]:
    """Queue feedback for Codex or TACITUS agents to improve the Databricks capsule factory."""
    feedback_id = f"mcp:feedback:{uuid.uuid4()}"
    _execute(
        f"""
        INSERT INTO {CATALOG}.capsule_gold.agent_feedback_queue
        VALUES (
          {_sql_literal(feedback_id)},
          {_sql_literal('databricks_mcp_server')},
          {_sql_literal('codex')},
          {_sql_literal(severity)},
          {_sql_literal(finding)},
          {_sql_literal(recommended_change)},
          {_sql_literal('mcp-tacitus-capsule-factory-dev')},
          {_sql_literal('queued')},
          current_timestamp()
        )
        """
    )
    return {"feedback_id": feedback_id, "status": "queued"}


@mcp.tool()
def get_mcp_contracts() -> dict[str, Any]:
    """Return the MCP tool contracts stored in Unity Catalog."""
    return {
        "contracts": _query(
            f"""
            SELECT mcp_tool_name, exposed_by_app, description, databricks_permissions,
                   backing_tables, side_effect_policy
            FROM {CATALOG}.capsule_gold.dashboard_mcp_contracts
            ORDER BY mcp_tool_name
            """
        )
    }


def main() -> None:
    mcp.run(transport="streamable-http")


if __name__ == "__main__":
    main()

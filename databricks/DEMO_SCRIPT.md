# TACITUS Databricks-Only Showcase Script

Use this script to show TACITUS and PRAXIS from the Databricks CLI and the
Databricks workspace page.

## Demo Goal

Show that Databricks is the governed Context Capsule Factory behind PRAXIS:

```text
CLI deploys the factory
-> Databricks Workflow runs the build
-> Unity Catalog stores every layer
-> Databricks AI Functions extract capsule structure inside the lakehouse
-> SQL views show portfolio, graph scale, review state, and agent controls
-> AI-search-ready corpus prepares retrieval
-> PRAXIS receives reviewable context-pack exports
```

PRAXIS remains the cockpit and review surface. Databricks produces governed,
reviewable capsule intelligence.

## CLI Setup

```powershell
cd C:\Users\giuli\A3_DIALECTICAbyTACITUS_v3\databricks

databricks auth profiles
databricks bundle validate -p tacitus
databricks bundle deploy -t dev -p tacitus
databricks bundle run tacitus_context_capsule_builder_showcase -t dev -p tacitus
```

## Workspace Pages To Open

Job:

```text
https://dbc-69e04818-40fb.cloud.databricks.com/jobs/1123194333821498?o=7474658425841042
```

Latest seven-task run:

```text
https://dbc-69e04818-40fb.cloud.databricks.com/?o=7474658425841042#job/1123194333821498/run/697665372540171
```

Databricks App:

```text
https://tacitus-capsule-builder-dev-7474658425841042.aws.databricksapps.com
```

MCP server app:

```text
https://mcp-tacitus-capsules-dev-7474658425841042.aws.databricksapps.com/mcp
```

Catalog:

```text
dialectica
```

Primary tables and views:

```text
dialectica.capsule_gold.dashboard_kpis
dialectica.capsule_gold.dashboard_capsule_portfolio
dialectica.capsule_gold.dashboard_graph_scale
dialectica.capsule_gold.dashboard_review_workbench
dialectica.capsule_gold.dashboard_agent_guidance
dialectica.capsule_gold.dashboard_connector_boundary
dialectica.capsule_gold.dashboard_showcase_click_path
dialectica.capsule_gold.dashboard_platform_story
dialectica.capsule_gold.dashboard_ai_extraction_lab
dialectica.capsule_gold.dashboard_ontology_showroom
dialectica.capsule_gold.dashboard_capsule_deep_dive
dialectica.capsule_gold.dashboard_showroom_narrative
dialectica.capsule_gold.dashboard_capsule_ops_agent
dialectica.capsule_gold.dashboard_agent_console
dialectica.capsule_gold.dashboard_agent_tool_registry
dialectica.capsule_gold.dashboard_mcp_contracts
dialectica.capsule_gold.dashboard_agent_feedback_queue
dialectica.capsule_gold.ai_search_corpus
dialectica.capsule_exports.context_pack_exports
```

Saved SQL queries:

```text
TACITUS Capsule Factory - KPI Overview
TACITUS Capsule Factory - Portfolio
TACITUS Capsule Factory - Graph Scale
TACITUS Capsule Factory - Review Workbench
TACITUS Capsule Factory - Agent Guidance
TACITUS Capsule Factory - AI Search Corpus
TACITUS Capsule Factory - Showroom Click Path
TACITUS Capsule Factory - Databricks Platform Story
TACITUS Capsule Factory - AI Extraction Lab
TACITUS Capsule Factory - Ontology Showroom
TACITUS Capsule Factory - Capsule Deep Dive
```

## Databricks Workspace Click Path

Start from:

```text
https://dbc-69e04818-40fb.cloud.databricks.com/browse?o=7474658425841042
```

Then show:

1. `Workflows` -> `[dev] TACITUS Context Capsule Builder Showcase` -> latest
   run `697665372540171`. Expand the DAG and point at the seventh task,
   `prepare_ai_showroom`.
2. `Catalog` -> `dialectica` -> `capsule_bronze`. Show builder sessions,
   uploaded/source inputs, human knowledge, and connector consent requests.
3. `Catalog` -> `dialectica` -> `capsule_silver`. Show ontology terms,
   semantic nodes/edges, temporal relations, causal relations, graph
   partitions, and similarity matches.
4. `Catalog` -> `dialectica` -> `capsule_gold`. Show capsule manifests,
   claims, runtime contracts, AI extraction runs, ontology cards, showroom
   narrative, and dashboard views.
5. `SQL Editor` -> saved queries -> open `TACITUS Capsule Factory - Showroom
   Click Path`, then `AI Extraction Lab`, then `Capsule Deep Dive`.
6. `Apps` -> `tacitus-capsule-builder-dev`. Open the `Showroom`, `AI
   Extraction Lab`, and `Ontology` tabs first.
7. In that same app, open `Agent / MCP`. Ask the agent to explain the workspace,
   trigger the workflow only if you want a new run, and queue Codex feedback.
8. `Apps` -> `mcp-tacitus-capsules-dev`. The MCP endpoint is `/mcp`; use this
   from Databricks AI Gateway MCP clients or MCP Inspector.

## Spoken Walkthrough

1. "PRAXIS stays simple: ask, attach context, review work."
2. "Databricks is where TACITUS builds the heavy context backbone."
3. "This Workflow creates four capsule families: USER, SITUATION, TOOL, and
   OUTPUT."
4. "The builder session shows how a user can tell the system what capsule they
   need."
5. "Connector rows show consent boundaries for uploads, PRAXIS history, Gmail,
   and Ladybug/Postgres."
6. "Silver tables show the structured substrate: ontology, graph, temporality,
   causality, and similarity."
7. "Gold tables show reviewable capsule intelligence: proposals, runtime
   contracts, agent guidance frames, and eval scores."
8. "The AI extraction lab is running inside Databricks through `ai_query`,
   and every prompt/result is stored as reviewable Delta data."
9. "The ontology showroom proves the capsules are generic: each kind has its
   own primitive classes, temporal model, causal model, review rules, and
   agent-control rules."
10. "The AI-search-ready corpus is what we would index for retrieval across
   evidence, claims, and agent guidance."
11. "The context-pack exports are the bridge back to PRAXIS, but they import as
   review cards. No hidden canonical write."

## Wow Moments

- Open the Workflow DAG and show the seven-step Databricks build.
- Open the Databricks App and show the tabbed control-room view.
- Open `dashboard_showcase_click_path` and let Databricks itself explain what
  to click.
- Open `dashboard_platform_story` and show the capability map: bundles,
  Workflows, Unity Catalog, AI Functions, Mosaic AI Model Serving, Delta, AI
  Search readiness, Apps, and evals.
- Open `dashboard_ai_extraction_lab` and show real AI extraction results
  generated by Databricks `ai_query` from governed Delta inputs.
- Open `dashboard_ontology_showroom` and show USER/SITUATION/TOOL/OUTPUT
  ontologies with temporal, causal, review, and agent-control primitives.
- Open `dashboard_agent_console` and show the agent's request log, tool
  registry, MCP contracts, feedback queue, and tool invocation log.
- Open the `Agent / MCP` app tab and ask: "What should a Databricks engineer
  look at first?"
- Open the MCP app and explain that the tools expose the same control plane at
  `/mcp`.
- Open `dashboard_kpis` and show counts in one table.
- Open `dashboard_graph_scale` and show 1,000 candidate graph nodes plus 1,500
  candidate graph edges.
- Open `dashboard_review_workbench` and show reviewable ontology, graph,
  temporal, and agent-control proposals.
- Open `dashboard_agent_guidance` and show the deterministic controls that
  narrow what PRAXIS agents can cite, infer, generate, or block.
- Open `ai_search_corpus` and explain this is the retrieval substrate for
  similar situations, user preferences, causal claims, and output contracts.
- Open `context_pack_exports` and show the PRAXIS-ready JSON payload.

## SQL Query Pack

Use `databricks/sql/showcase_queries.sql` for SQL Editor demos. The fastest
single-query proof is:

```sql
SELECT * FROM dialectica.capsule_gold.dashboard_kpis;
```

Then run:

```sql
SELECT * FROM dialectica.capsule_gold.dashboard_review_workbench;
SELECT * FROM dialectica.capsule_gold.dashboard_agent_guidance;
SELECT * FROM dialectica.capsule_gold.dashboard_ai_extraction_lab;
SELECT * FROM dialectica.capsule_gold.dashboard_ontology_showroom;
SELECT * FROM dialectica.capsule_gold.ai_search_corpus LIMIT 20;
```

## AI Search Next Step

The Delta source table is ready:

```text
dialectica.capsule_gold.ai_search_corpus
```

Use `corpus_id` as the primary key and `body` as the text column. Keep metadata
fields `capsule_id`, `source_layer`, `review_state`, and `metadata_json`.

Create the actual AI Search index only when you are ready to spend endpoint
resources. The source table is already structured for Delta Sync.

## Demo Boundary

Everything in this Databricks demo is reviewable and non-canonical until PRAXIS
accepts it. This is the core trust story:

```text
Databricks can generate structure at scale.
PRAXIS decides what becomes official context.
```

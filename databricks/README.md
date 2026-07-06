# TACITUS Context Capsule Factory on Databricks

This Databricks Bundle builds a demo Context Capsule factory for TACITUS.

It is intentionally broader than conflict analysis. The demo shows how
DIALECTICA can compile four capsule kinds from domain sources and human
knowledge:

- `USER`: analyst/team context, preferences, authority boundaries, and review
  standards.
- `SITUATION`: a concrete policy analysis object with actors, claims, events,
  risks, and uncertainty.
- `TOOL`: an expert method, ontology, rubric, or reasoning device.
- `OUTPUT`: a reviewed work product that can be attached back to PRAXIS.

## Boundary

Databricks is the governed evidence, lineage, evaluation, and analytics plane.
It is not the PRAXIS app database and not the canonical capsule compiler.
DIALECTICA still validates and compiles capsule bundles; PRAXIS still owns the
user-facing cockpit.

## Bundle Commands

Read-only validation:

```powershell
cd C:\Users\giuli\A3_DIALECTICAbyTACITUS_v3\databricks
databricks bundle validate -p tacitus
```

Deploy to the Databricks workspace after explicit approval:

```powershell
databricks bundle deploy -t dev -p tacitus
```

Run the showcase job after deploy:

```powershell
databricks bundle run tacitus_context_capsule_builder_showcase -t dev -p tacitus
```

## What The Showcase Creates

Target catalog: `dialectica`

Deployment status on 2026-06-24: deployed and run successfully.

Job URL:

```text
https://dbc-69e04818-40fb.cloud.databricks.com/jobs/1123194333821498?o=7474658425841042
```

Successful run:

```text
https://dbc-69e04818-40fb.cloud.databricks.com/?o=7474658425841042#job/1123194333821498/run/697665372540171
```

Schemas:

- `capsule_registry`
- `capsule_bronze`
- `capsule_silver`
- `capsule_gold`
- `capsule_exports`
- `capsule_evals`

The job writes synthetic but product-shaped demo rows for a generic policy
analysis domain:

- `cap_user_policy_director_demo`
- `cap_situation_ai_governance_demo`
- `cap_tool_causal_policy_map_demo`
- `cap_output_ai_governance_brief_demo`

The final notebook displays ready-to-demo queries for capsule manifests,
evidence, ontology terms, temporal relations, causal relations, review status,
and PRAXIS context-pack exports.

The expanded guided-builder flow also creates:

- one concrete builder session that starts from a PRAXIS-style user request;
- AI interview turns that capture user preferences, similarity requests,
  ontology plans, and agent operating rules;
- connector request rows for uploads, PRAXIS history, Gmail OAuth, and
  Ladybug/Postgres graph snapshots;
- similarity matches across USER, SITUATION, TOOL, and OUTPUT capsules;
- graph partitions with 1,000 synthetic candidate graph nodes and 1,500
  candidate graph edges;
- reviewable ontology, graph, temporal, and agent-control proposals;
- deterministic PRAXIS agent guidance frames that bound what agents may cite,
  retrieve, generate, block, or send back for review.
- dashboard-ready Databricks views under `dialectica.capsule_gold`;
- a Databricks AI Functions showroom that uses `ai_query` against
  `databricks-gpt-5-mini` for USER, SITUATION, TOOL, and OUTPUT extraction;
- ontology design cards for each capsule kind, including primitive classes,
  temporal primitives, causal primitives, review rules, and agent-control
  rules;
- click-path and platform-story views that explain the demo directly in Unity
  Catalog;
- an AI-search-ready Delta table at
  `dialectica.capsule_gold.ai_search_corpus`;
- a Databricks App control room:
  `https://tacitus-capsule-builder-dev-7474658425841042.aws.databricksapps.com`.
- a Databricks-native Capsule Operations Agent and MCP control plane:
  `https://mcp-tacitus-capsules-dev-7474658425841042.aws.databricksapps.com/mcp`.

Latest verification on 2026-06-24:

- `capsule_builder_sessions`: 1 demo session
- `builder_interview_turns`: 4 turns
- `source_connector_requests`: 4 connector rows
- `capsule_similarity_matches`: 4 matches
- `graph_partitions`: 4 partitions
- `semantic_nodes`: 1,000 `demo:deep:%` candidate graph nodes
- `semantic_edges`: 1,500 `demo:deep:%` candidate graph edges
- `capsule_improvement_proposals`: 16 reviewable proposals
- `agent_guidance_frames`: 4 PRAXIS agent frames
- `context_pack_exports`: 4 guided-builder PRAXIS exports
- `dashboard_kpis`: 7 KPI rows
- `dashboard_showcase_click_path`: 7 Databricks walkthrough rows
- `dashboard_platform_story`: 9 Databricks capability rows
- `dashboard_ai_extraction_lab`: 4 Databricks AI extraction runs
- `dashboard_ontology_showroom`: 4 ontology design cards
- `dashboard_capsule_deep_dive`: 4 capsule deep-dive rows
- `dashboard_capsule_ops_agent`: seeded Databricks agent request/response
- `dashboard_agent_console`: consolidated agent, tool, MCP, feedback, and
  invocation console
- `dashboard_mcp_contracts`: MCP tool contracts
- `ai_search_corpus`: 16 retrieval corpus rows

## First Commercial Proof

This bundle is designed to support a Databricks-page demo:

1. Open the deployed job.
2. Run the showcase workflow.
3. Open the final `prepare_ai_showroom` notebook task.
4. Show the guided builder session, interview turns, connector requests,
   similarity matches, graph partitions, proposals, and agent guidance frames.
5. Open the Databricks App:
   `https://tacitus-capsule-builder-dev-7474658425841042.aws.databricksapps.com`.
6. Open the saved SQL queries named `TACITUS Capsule Factory - ...`.
7. Open the `dialectica.capsule_exports.context_pack_exports` table.
8. Show each capsule's PRAXIS context pack JSON and guided-builder export.
9. Show lineage and source/evidence tables behind those context packs.
10. Explain that this is the generic capsule factory TACITUS can specialize for
   policy, regulation, conflict, climate, public health, legal, and strategy
   analysis.

## Databricks App

The bundle deploys a Streamlit Databricks App:

```text
tacitus-capsule-builder-dev
https://tacitus-capsule-builder-dev-7474658425841042.aws.databricksapps.com
```

The app reads curated Unity Catalog views and tables through the Serverless
Starter Warehouse. It shows portfolio KPIs, the builder session, connector
boundaries, Databricks click paths, AI extraction runs, ontology design cards,
graph scale, temporal and causal relations, reviewable proposals, agent
guidance frames, the AI-search-ready corpus, PRAXIS exports, and an
interactive `Agent / MCP` tab.

The bundle also deploys a custom MCP server app:

```text
mcp-tacitus-capsules-dev
https://mcp-tacitus-capsules-dev-7474658425841042.aws.databricksapps.com/mcp
```

Tools exposed by the MCP app:

- `health`
- `explain_workspace`
- `summarize_capsule_factory`
- `run_capsule_factory`
- `write_codex_feedback`
- `get_mcp_contracts`

Saved SQL queries created in the workspace:

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

## Demo Script

Use this short script when showing the Databricks page:

1. "PRAXIS stays simple: ask first, save reusable context."
2. "Databricks runs the heavy capsule factory behind that simple surface."
3. "Here is the user request and the builder interview."
4. "Here are the allowed source connectors and consent boundaries."
5. "Here are the four capsules PRAXIS needs: USER, SITUATION, TOOL, OUTPUT."
6. "Here is Databricks AI extracting structure from governed Delta inputs."
7. "Here are the capsule-specific ontologies: primitive classes, temporal
   model, causal model, review rules, and agent controls."
8. "Here is the graph scale: nodes, edges, ontology partitions, temporal and
   causal layers."
9. "Here are the reviewable improvement proposals. Nothing silently becomes
   canonical."
10. "Here is the agent guidance frame. This is how the capsule narrows the
   probabilistic generation space for PRAXIS agents."

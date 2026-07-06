# Databricks Connection

Status: optional TACITUS analytics and ML plane. Databricks is not the
canonical DIALECTICA or PRAXIS store.

## Current Local Profile

Profile: `tacitus`

Host:

```text
https://dbc-69e04818-40fb.cloud.databricks.com
```

Account id:

```text
7caac1c1-eb1d-4290-a26b-77dd326ae5bb
```

Local check on 2026-06-24:

```powershell
databricks --version
databricks auth profiles
databricks auth env --profile tacitus
```

Observed result:

- Databricks CLI is installed: `v0.298.0`.
- The `tacitus` profile exists and is valid.
- `databricks current-user me --profile tacitus` returns
  `giulio@tacitus.me`.
- The workspace has an existing Unity Catalog catalog named `dialectica`.

## Refresh The Connection

Do not paste tokens into this repository or into `.env` files. Refresh the
profile with browser OAuth:

```powershell
databricks auth login --host "https://dbc-69e04818-40fb.cloud.databricks.com?account_id=7caac1c1-eb1d-4290-a26b-77dd326ae5bb" --profile tacitus
```

Then verify:

```powershell
databricks auth profiles
databricks current-user me --profile tacitus
python -m dialectica_tools.databricks_connection --profile tacitus
```

If Python cannot import `dialectica_tools`, run the checker from the tool root:

```powershell
cd C:\Users\giuli\A3_DIALECTICAbyTACITUS_v3\tools\python
python -m dialectica_tools.databricks_connection --profile tacitus
```

## TACITUS Use Cases

Use Databricks for derived, governed analytics and heavier experimental work:

- DIALECTICA capsule build metrics, eval reports, review latency, and quality
  deltas.
- DIALECTICA graph snapshots for offline graph analytics, KGE experiments, and
  model comparison.
- PRAXIS agent-run and answer-quality marts after sensitive text is redacted,
  hashed, aggregated, or explicitly approved for analysis.
- Cross-product TACITUS dashboards that compare capsule freshness, source
  coverage, review bottlenecks, and user-visible improvement.
- A Databricks-page showcase of the TACITUS Context Capsule Factory through
  the bundle under `databricks/`.

Do not use Databricks as:

- the canonical DIALECTICA capsule store;
- the canonical PRAXIS user-facing store;
- a direct write path back into PRAXIS Firestore;
- a hidden promotion path for graph deltas or capsule claims;
- a default sink for raw user documents, private notes, or unpublished source
  text.

## First Table Shape

The first DIALECTICA snapshot should stay append-only and reconstructable:

```text
graph_nodes(snapshot_id, workspace_id, capsule_id, node_id, node_type, label, review_state, source_ids, created_at)
graph_edges(snapshot_id, workspace_id, capsule_id, edge_id, source_id, target_id, edge_type, review_state, source_ids, created_at)
```

Recommended companion marts:

```text
capsule_builds(capsule_id, tenant_id, bundle_digest, status, source_count, warning_count, review_state, compiled_at)
read_receipts(capsule_id, bundle_digest, praxis_run_id, workflow, source_count, claim_count, warning_count, used_at)
answer_quality(praxis_run_id, capsule_id, workflow, capsule_vs_baseline_delta, source_coverage, unresolved_uncertainty_count, measured_at)
```

These tables are derived outputs. DIALECTICA PostgreSQL and capsule bundles
remain canonical; PRAXIS Firestore remains canonical for PRAXIS cockpit state
and capsule visibility.

## Integration Rule

Any production exporter must:

1. run after Rust validation and review gates;
2. include `snapshot_id`, `workspace_id` or tenant scope, `capsule_id`, and
   `bundle_digest`;
3. avoid raw sensitive text by default;
4. be idempotent by snapshot;
5. return a receipt that PRAXIS can display as analytics-only, not as proof of
   capsule truth.

## Context Capsule Factory Bundle

The local Databricks Bundle lives at `databricks/` and validates against the
`tacitus` profile.

Deployment status on 2026-06-24: deployed and run successfully on serverless
Databricks Workflows.

Job:

```text
https://dbc-69e04818-40fb.cloud.databricks.com/jobs/1123194333821498?o=7474658425841042
```

Successful run:

```text
https://dbc-69e04818-40fb.cloud.databricks.com/?o=7474658425841042#job/1123194333821498/run/697665372540171
```

Created schemas:

```text
dialectica.capsule_registry
dialectica.capsule_bronze
dialectica.capsule_silver
dialectica.capsule_gold
dialectica.capsule_exports
dialectica.capsule_evals
```

Key demo tables:

```text
dialectica.capsule_bronze.capsule_builder_sessions
dialectica.capsule_bronze.builder_interview_turns
dialectica.capsule_bronze.source_connector_requests
dialectica.capsule_silver.capsule_similarity_matches
dialectica.capsule_silver.graph_partitions
dialectica.capsule_gold.capsule_manifests
dialectica.capsule_gold.capsule_claims
dialectica.capsule_gold.reasoning_devices
dialectica.capsule_gold.memory_build_history
dialectica.capsule_gold.runtime_contracts
dialectica.capsule_gold.capsule_improvement_proposals
dialectica.capsule_gold.agent_guidance_frames
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
dialectica.capsule_gold.ai_search_corpus
dialectica.capsule_exports.context_pack_exports
```

The first successful run created four demo PRAXIS-oriented context-pack
exports, one each for `USER`, `SITUATION`, `TOOL`, and `OUTPUT` capsules.

Read-only validation:

```powershell
cd C:\Users\giuli\A3_DIALECTICAbyTACITUS_v3\databricks
databricks bundle validate -p tacitus
```

Deploying or running this bundle mutates the Databricks workspace and may start
compute, so it requires explicit operator approval:

```powershell
databricks bundle deploy -t dev -p tacitus
databricks bundle run tacitus_context_capsule_builder_showcase -t dev -p tacitus
```

The bundle creates a Databricks job that builds demo `USER`, `SITUATION`,
`TOOL`, and `OUTPUT` capsules in the `dialectica` catalog and exports PRAXIS
context-pack JSON rows.

Expanded guided-builder flow:

```text
PRAXIS-style user request
-> capsule_builder_sessions
-> builder_interview_turns
-> source_connector_requests
-> similarity matches
-> ontology / graph / temporal / causal candidate layers
-> capsule_improvement_proposals
-> agent_guidance_frames
-> PRAXIS guided-builder context-pack exports
```

The guided-builder rows are intentionally reviewable and non-canonical. They
show how Databricks can run broad, expensive, ontology-rich context refinement
while PRAXIS still owns human review and final capsule promotion.

Latest Databricks verification on 2026-06-24:

```text
builder_sessions: 1
interview_turns: 4
connector_requests: 4
similarity_matches: 4
graph_partitions: 4
deep_graph_nodes: 1000
deep_graph_edges: 1500
improvement_proposals: 16
agent_guidance_frames: 4
guided_exports: 4
dashboard_kpis: 7
showcase_click_path: 7
platform_capabilities: 9
ai_extraction_runs: 4
ontology_cards: 4
capsule_deep_dive_rows: 4
ai_search_corpus_rows: 16
```

Databricks App:

```text
tacitus-capsule-builder-dev
https://tacitus-capsule-builder-dev-7474658425841042.aws.databricksapps.com
```

Databricks MCP server app:

```text
mcp-tacitus-capsules-dev
https://mcp-tacitus-capsules-dev-7474658425841042.aws.databricksapps.com/mcp
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

Agent and MCP control-plane views:

```text
dialectica.capsule_gold.dashboard_capsule_ops_agent
dialectica.capsule_gold.dashboard_agent_console
dialectica.capsule_gold.dashboard_agent_tool_registry
dialectica.capsule_gold.dashboard_mcp_contracts
dialectica.capsule_gold.dashboard_agent_feedback_queue
dialectica.capsule_gold.dashboard_agent_tool_invocations
```

# Databricks notebook source
# MAGIC %md
# MAGIC # Build Demo Context Capsules
# MAGIC
# MAGIC Compiles product-shaped demo rows for USER, SITUATION, TOOL, and OUTPUT
# MAGIC capsules. This notebook demonstrates the Databricks-side factory output
# MAGIC that DIALECTICA can validate and PRAXIS can attach as context.

# COMMAND ----------

from datetime import datetime, timedelta
import hashlib
import json

from pyspark.sql import Row

dbutils.widgets.text("catalog", "dialectica")
catalog = dbutils.widgets.get("catalog")
now = datetime.utcnow()

capsule_ids = [
    "cap_user_policy_director_demo",
    "cap_situation_ai_governance_demo",
    "cap_tool_causal_policy_map_demo",
    "cap_output_ai_governance_brief_demo",
]

for table, column in [
    ("capsule_silver.evidence_spans", "span_id"),
    ("capsule_silver.ontology_terms", "term_id"),
    ("capsule_silver.semantic_nodes", "capsule_id"),
    ("capsule_silver.semantic_edges", "capsule_id"),
    ("capsule_silver.temporal_relations", "capsule_id"),
    ("capsule_silver.causal_relations", "capsule_id"),
    ("capsule_gold.capsule_manifests", "capsule_id"),
    ("capsule_gold.capsule_claims", "capsule_id"),
    ("capsule_gold.review_verdicts", "capsule_id"),
    ("capsule_gold.reasoning_devices", "capsule_id"),
    ("capsule_gold.memory_build_history", "capsule_id"),
    ("capsule_gold.runtime_contracts", "capsule_id"),
    ("capsule_exports.context_pack_exports", "capsule_id"),
    ("capsule_evals.capsule_quality_scores", "capsule_id"),
]:
    if column == "capsule_id":
        quoted = ", ".join([f"'{item}'" for item in capsule_ids])
        spark.sql(f"DELETE FROM {catalog}.{table} WHERE capsule_id IN ({quoted})")
    else:
        spark.sql(f"DELETE FROM {catalog}.{table} WHERE {column} LIKE 'demo:%'")

# COMMAND ----------

raw_inputs = {
    row.input_id: row
    for row in spark.table(f"{catalog}.capsule_bronze.raw_inputs")
    .where("input_id LIKE 'demo:%'")
    .collect()
}

spans = [
    Row(
        span_id="demo:span:user-style:1",
        source_id="demo:human:user-interview",
        input_id="demo:input:user-style",
        text=raw_inputs["demo:input:user-style"].content,
        locator="interview:style",
        source_hash=hashlib.sha256(raw_inputs["demo:input:user-style"].content.encode()).hexdigest(),
        rights_profile="private_user_context",
        trust_state="human_reviewed_demo",
        created_at=now,
    ),
    Row(
        span_id="demo:span:situation:1",
        source_id="demo:source:ai-act-summary",
        input_id="demo:input:situation-ai-governance",
        text=raw_inputs["demo:input:situation-ai-governance"].content,
        locator="policy-source:summary",
        source_hash=hashlib.sha256(raw_inputs["demo:input:situation-ai-governance"].content.encode()).hexdigest(),
        rights_profile="demo_public_summary",
        trust_state="source_backed_demo",
        created_at=now,
    ),
    Row(
        span_id="demo:span:tool:1",
        source_id="demo:human:expert-rubric",
        input_id="demo:input:causal-tool",
        text=raw_inputs["demo:input:causal-tool"].content,
        locator="rubric:method",
        source_hash=hashlib.sha256(raw_inputs["demo:input:causal-tool"].content.encode()).hexdigest(),
        rights_profile="tacitus_method_demo",
        trust_state="expert_reviewed_demo",
        created_at=now,
    ),
    Row(
        span_id="demo:span:output:1",
        source_id="demo:source:agency-capacity-note",
        input_id="demo:input:output-brief",
        text=raw_inputs["demo:input:output-brief"].content,
        locator="brief:draft",
        source_hash=hashlib.sha256(raw_inputs["demo:input:output-brief"].content.encode()).hexdigest(),
        rights_profile="team_internal_reviewed",
        trust_state="review_required_demo",
        created_at=now,
    ),
]
spark.createDataFrame(spans).write.mode("append").saveAsTable(
    f"{catalog}.capsule_silver.evidence_spans"
)

# COMMAND ----------

terms = [
    Row("demo:term:source-led-brief", "expert_context_v1", "Source-led brief", "Output that foregrounds source receipts and caveats.", None, "USER", "approved", ["demo:span:user-style:1"], now),
    Row("demo:term:implementation-guidance", "policy_analysis_v1", "Implementation guidance", "A policy instrument clarifying compliance expectations.", None, "SITUATION", "approved", ["demo:span:situation:1"], now),
    Row("demo:term:agency-capacity", "policy_analysis_v1", "Agency capacity", "Operational ability to implement, monitor, and enforce policy.", None, "SITUATION", "approved_with_caveats", ["demo:span:output:1"], now),
    Row("demo:term:causal-mechanism", "policy_analysis_v1", "Causal mechanism", "The pathway connecting an intervention to an outcome.", None, "TOOL", "approved", ["demo:span:tool:1"], now),
]
spark.createDataFrame(
    terms,
    "term_id string, domain_pack_id string, label string, definition string, parent_term_id string, capsule_kind string, review_state string, source_span_ids array<string>, created_at timestamp",
).write.mode("append").saveAsTable(f"{catalog}.capsule_silver.ontology_terms")

manifest_specs = [
    ("cap_user_policy_director_demo", "USER", "expert_context_v1", "Policy Director User Context", "Private operating context for source-led policy analysis.", "approved_with_caveats"),
    ("cap_situation_ai_governance_demo", "SITUATION", "policy_analysis_v1", "AI Governance Implementation Situation", "A policy situation capsule with actors, obligations, temporal pressure, and causal assumptions.", "approved_with_caveats"),
    ("cap_tool_causal_policy_map_demo", "TOOL", "policy_analysis_v1", "Causal Policy Map Tool", "A reusable expert method capsule for intervention-mechanism-outcome analysis.", "approved"),
    ("cap_output_ai_governance_brief_demo", "OUTPUT", "policy_analysis_v1", "AI Governance Readiness Brief", "A reviewed output capsule suitable for attachment to PRAXIS.", "needs_review"),
]

manifests = []
for capsule_id, kind, domain_pack_id, title, summary, review_state in manifest_specs:
    digest = hashlib.sha256(f"{capsule_id}|{title}|{summary}|{now.isoformat()}".encode()).hexdigest()
    manifests.append(
        Row(
            capsule_id=capsule_id,
            capsule_kind=kind,
            domain_pack_id=domain_pack_id,
            title=title,
            summary=summary,
            review_state=review_state,
            freshness="current",
            bundle_digest=f"sha256:{digest}",
            source_count=2 if kind in ["USER", "TOOL"] else 3,
            claim_count=2,
            graph_node_count=3,
            graph_edge_count=2,
            compiled_at=now,
        )
    )
spark.createDataFrame(manifests).write.mode("append").saveAsTable(
    f"{catalog}.capsule_gold.capsule_manifests"
)

# COMMAND ----------

claims = [
    Row("demo:claim:user-source-led", "cap_user_policy_director_demo", "The analyst prefers source-led briefs with explicit caveats.", "preference", 0.91, ["demo:span:user-style:1"], "approved_with_caveats", "Private user preference; do not expose in public output.", now),
    Row("demo:claim:user-causal-threshold", "cap_user_policy_director_demo", "Causal claims should name the mechanism and evidence strength.", "review_standard", 0.88, ["demo:span:user-style:1"], "approved_with_caveats", "Applies to policy analysis outputs.", now),
    Row("demo:claim:guidance-needed", "cap_situation_ai_governance_demo", "Implementation guidance is a near-term policy bottleneck.", "analysis", 0.83, ["demo:span:situation:1"], "approved_with_caveats", "Synthetic demo; verify with real sources before production use.", now),
    Row("demo:claim:capacity-risk", "cap_situation_ai_governance_demo", "Agency capacity constraints may weaken implementation quality.", "risk", 0.76, ["demo:span:output:1"], "needs_review", "Requires operational evidence.", now),
    Row("demo:claim:tool-mechanism-required", "cap_tool_causal_policy_map_demo", "A causal policy map requires an explicit mechanism between intervention and outcome.", "method_rule", 0.94, ["demo:span:tool:1"], "approved", "Core method rule.", now),
    Row("demo:claim:tool-correlation-warning", "cap_tool_causal_policy_map_demo", "Correlation should not be treated as mechanism without source-backed explanation.", "failure_mode", 0.92, ["demo:span:tool:1"], "approved", "Core method caveat.", now),
    Row("demo:claim:output-phased-calendar", "cap_output_ai_governance_brief_demo", "The output recommends a phased guidance calendar.", "recommendation", 0.78, ["demo:span:output:1", "demo:span:situation:1"], "needs_review", "Needs human approval before publication.", now),
    Row("demo:claim:output-consultation-checkpoint", "cap_output_ai_governance_brief_demo", "The output recommends a stakeholder consultation checkpoint.", "recommendation", 0.74, ["demo:span:output:1"], "needs_review", "Stakeholder list incomplete.", now),
]
spark.createDataFrame(claims, "claim_id string, capsule_id string, claim_text string, stance string, confidence double, source_span_ids array<string>, review_state string, uncertainty string, created_at timestamp").write.mode("append").saveAsTable(
    f"{catalog}.capsule_gold.capsule_claims"
)

# COMMAND ----------

nodes = [
    Row("demo:node:user-analyst", "cap_user_policy_director_demo", "expert_context_v1", "ExpertRole", "Policy director", ["demo:span:user-style:1"], json.dumps({"scope": "private"}), "approved_with_caveats", now),
    Row("demo:node:user-brief-style", "cap_user_policy_director_demo", "expert_context_v1", "Preference", "Source-led brief style", ["demo:span:user-style:1"], json.dumps({"output_effect": "format"}), "approved_with_caveats", now),
    Row("demo:node:user-causal-threshold", "cap_user_policy_director_demo", "expert_context_v1", "ReviewStandard", "Causal mechanism threshold", ["demo:span:user-style:1"], json.dumps({"required": True}), "approved_with_caveats", now),
    Row("demo:node:regulator", "cap_situation_ai_governance_demo", "policy_analysis_v1", "Actor", "Regulator", ["demo:span:situation:1"], json.dumps({"actor_type": "public_authority"}), "approved", now),
    Row("demo:node:implementation-guidance", "cap_situation_ai_governance_demo", "policy_analysis_v1", "PolicyInstrument", "Implementation guidance", ["demo:span:situation:1"], json.dumps({"instrument_type": "guidance"}), "approved", now),
    Row("demo:node:agency-capacity", "cap_situation_ai_governance_demo", "policy_analysis_v1", "Constraint", "Agency capacity", ["demo:span:output:1"], json.dumps({"risk": "capacity"}), "needs_review", now),
    Row("demo:node:causal-map", "cap_tool_causal_policy_map_demo", "policy_analysis_v1", "Method", "Causal policy map", ["demo:span:tool:1"], json.dumps({"method_version": "0.1.0"}), "approved", now),
    Row("demo:node:mechanism", "cap_tool_causal_policy_map_demo", "policy_analysis_v1", "Step", "Name mechanism", ["demo:span:tool:1"], json.dumps({"required": True}), "approved", now),
    Row("demo:node:correlation-risk", "cap_tool_causal_policy_map_demo", "policy_analysis_v1", "FailureMode", "Correlation treated as mechanism", ["demo:span:tool:1"], json.dumps({"severity": "high"}), "approved", now),
    Row("demo:node:brief", "cap_output_ai_governance_brief_demo", "policy_analysis_v1", "Output", "AI governance readiness brief", ["demo:span:output:1"], json.dumps({"surface": "praxis_memo"}), "needs_review", now),
    Row("demo:node:phased-calendar", "cap_output_ai_governance_brief_demo", "policy_analysis_v1", "Recommendation", "Phased guidance calendar", ["demo:span:output:1"], json.dumps({"section": "recommendation"}), "needs_review", now),
    Row("demo:node:consultation", "cap_output_ai_governance_brief_demo", "policy_analysis_v1", "Recommendation", "Stakeholder consultation checkpoint", ["demo:span:output:1"], json.dumps({"section": "recommendation"}), "needs_review", now),
]
spark.createDataFrame(nodes, "node_id string, capsule_id string, domain_pack_id string, node_type string, label string, source_span_ids array<string>, properties_json string, review_state string, created_at timestamp").write.mode("append").saveAsTable(
    f"{catalog}.capsule_silver.semantic_nodes"
)

edges = [
    Row("demo:edge:user-prefers-brief", "cap_user_policy_director_demo", "prefers_output", "demo:node:user-analyst", "demo:node:user-brief-style", ["demo:span:user-style:1"], 0.91, json.dumps({"valid_from": now.isoformat()}), json.dumps({}), "approved_with_caveats", now),
    Row("demo:edge:user-requires-causal-threshold", "cap_user_policy_director_demo", "requires_review_standard", "demo:node:user-analyst", "demo:node:user-causal-threshold", ["demo:span:user-style:1"], 0.88, json.dumps({"valid_from": now.isoformat()}), json.dumps({}), "approved_with_caveats", now),
    Row("demo:edge:regulator-issues-guidance", "cap_situation_ai_governance_demo", "issues", "demo:node:regulator", "demo:node:implementation-guidance", ["demo:span:situation:1"], 0.83, json.dumps({"deadline": (now + timedelta(days=180)).date().isoformat()}), json.dumps({"mechanism": "clarifies compliance expectations"}), "approved_with_caveats", now),
    Row("demo:edge:capacity-constrains-guidance", "cap_situation_ai_governance_demo", "constrains", "demo:node:agency-capacity", "demo:node:implementation-guidance", ["demo:span:output:1"], 0.76, json.dumps({"valid_from": now.isoformat()}), json.dumps({"mechanism": "staffing limits review and enforcement bandwidth"}), "needs_review", now),
    Row("demo:edge:method-has-step", "cap_tool_causal_policy_map_demo", "has_step", "demo:node:causal-map", "demo:node:mechanism", ["demo:span:tool:1"], 0.94, json.dumps({"valid_from": now.isoformat()}), json.dumps({"effect": "raises causal clarity"}), "approved", now),
    Row("demo:edge:method-warns-risk", "cap_tool_causal_policy_map_demo", "warns_about", "demo:node:causal-map", "demo:node:correlation-risk", ["demo:span:tool:1"], 0.92, json.dumps({"valid_from": now.isoformat()}), json.dumps({"effect": "prevents false causal claims"}), "approved", now),
    Row("demo:edge:brief-recommends-calendar", "cap_output_ai_governance_brief_demo", "recommends", "demo:node:brief", "demo:node:phased-calendar", ["demo:span:output:1"], 0.78, json.dumps({"valid_from": now.isoformat()}), json.dumps({"mechanism": "reduces implementation uncertainty"}), "needs_review", now),
    Row("demo:edge:brief-recommends-consultation", "cap_output_ai_governance_brief_demo", "recommends", "demo:node:brief", "demo:node:consultation", ["demo:span:output:1"], 0.74, json.dumps({"valid_from": now.isoformat()}), json.dumps({"mechanism": "surfaces affected actor constraints"}), "needs_review", now),
]
spark.createDataFrame(edges, "edge_id string, capsule_id string, edge_type string, from_node_id string, to_node_id string, source_span_ids array<string>, confidence double, temporal_scope_json string, causal_scope_json string, review_state string, created_at timestamp").write.mode("append").saveAsTable(
    f"{catalog}.capsule_silver.semantic_edges"
)

# COMMAND ----------

temporal_relations = [
    Row("demo:time:guidance-window", "cap_situation_ai_governance_demo", "deadline_window", "demo:node:implementation-guidance", "demo:node:regulator", now, now + timedelta(days=180), now, "BEFORE", "medium", ["demo:span:situation:1"], "approved_with_caveats"),
    Row("demo:time:user-preference-current", "cap_user_policy_director_demo", "current_preference", "demo:node:user-brief-style", "demo:node:user-analyst", now, None, now, "STARTS", "low", ["demo:span:user-style:1"], "approved_with_caveats"),
    Row("demo:time:method-version-current", "cap_tool_causal_policy_map_demo", "method_version_current", "demo:node:causal-map", "demo:node:mechanism", now, None, now, "EQUALS", "low", ["demo:span:tool:1"], "approved"),
    Row("demo:time:output-review-pending", "cap_output_ai_governance_brief_demo", "review_gate_pending", "demo:node:brief", "demo:node:phased-calendar", now, now + timedelta(days=14), now, "MEETS", "medium", ["demo:span:output:1"], "needs_review"),
]
spark.createDataFrame(temporal_relations, "temporal_relation_id string, capsule_id string, relation_type string, subject_id string, object_id string, valid_from timestamp, valid_until timestamp, known_at timestamp, allen_relation string, uncertainty string, source_span_ids array<string>, review_state string").write.mode("append").saveAsTable(
    f"{catalog}.capsule_silver.temporal_relations"
)

causal_relations = [
    Row("demo:cause:guidance-clarity", "cap_situation_ai_governance_demo", "demo:node:implementation-guidance", "demo:node:phased-calendar", "Guidance calendar reduces compliance timing uncertainty.", 0.72, ["requires agency capacity", "requires stakeholder notice"], ["demo:span:situation:1", "demo:span:output:1"], "needs_review"),
    Row("demo:cause:method-quality", "cap_tool_causal_policy_map_demo", "demo:node:mechanism", "demo:node:correlation-risk", "Naming mechanisms reduces false causal inference.", 0.86, ["reviewer applies caveat"], ["demo:span:tool:1"], "approved"),
]
spark.createDataFrame(causal_relations, "causal_relation_id string, capsule_id string, cause_id string, effect_id string, mechanism string, confidence double, assumptions array<string>, source_span_ids array<string>, review_state string").write.mode("append").saveAsTable(
    f"{catalog}.capsule_silver.causal_relations"
)

# COMMAND ----------

reviews = [
    Row("demo:review:user", "cap_user_policy_director_demo", "privacy_reviewer", "approved_with_caveats", ["Do not reveal private user style in public outputs."], now),
    Row("demo:review:situation", "cap_situation_ai_governance_demo", "policy_editor", "approved_with_caveats", ["Capacity claim needs stronger operational evidence."], now),
    Row("demo:review:tool", "cap_tool_causal_policy_map_demo", "method_reviewer", "approved", ["Use only with evidence-backed mechanisms."], now),
    Row("demo:review:output", "cap_output_ai_governance_brief_demo", "human_reviewer", "needs_review", ["Recommendations need signoff before PRAXIS publication."], now),
]
spark.createDataFrame(reviews, "review_id string, capsule_id string, reviewer_role string, decision string, caveats array<string>, reviewed_at timestamp").write.mode("append").saveAsTable(
    f"{catalog}.capsule_gold.review_verdicts"
)

devices = [
    Row("demo:device:sourceability-check", "cap_user_policy_director_demo", "Sourceability check", "heuristic", ["Identify material claim", "Find source span", "Require trust tier", "Flag unsupported personalization"], ["private preference leak", "overfitting to stale preference"], ["Claim", "EvidenceSpan", "TrustTier"], "privacy_reviewer", "approved_with_caveats", now),
    Row("demo:device:decision-clock", "cap_situation_ai_governance_demo", "Decision clock", "temporal_method", ["Name deadline", "Separate occurred_at from known_at", "List decisions possible now", "List blocking unknowns"], ["treating stale knowledge as current", "hiding deadline uncertainty"], ["Event", "TemporalRelation", "Claim"], "policy_editor", "approved_with_caveats", now),
    Row("demo:device:causal-policy-map", "cap_tool_causal_policy_map_demo", "Causal policy map", "expert_method", ["Name intervention", "Name mechanism", "Name affected actor", "Name intended and unintended outcomes", "Attach evidence strength"], ["correlation as mechanism", "single-actor causal story"], ["Actor", "PolicyInstrument", "Outcome", "CausalRelation"], "method_reviewer", "approved", now),
    Row("demo:device:brief-contract-check", "cap_output_ai_governance_brief_demo", "Brief contract check", "output_contract", ["Verify required sections", "Check citations", "Surface caveats", "Block publication if review missing"], ["uncited recommendation", "missing review gate"], ["Output", "Claim", "ReviewVerdict"], "human_reviewer", "needs_review", now),
]
spark.createDataFrame(devices, "device_id string, capsule_id string, label string, device_kind string, procedure_steps array<string>, traps array<string>, required_primitives array<string>, reviewer_role string, review_state string, created_at timestamp").write.mode("append").saveAsTable(
    f"{catalog}.capsule_gold.reasoning_devices"
)

build_history = [
    Row("demo:decision:user-private-scope", "cap_user_policy_director_demo", "governance", "privacy_reviewer", "Approved user context only for private PRAXIS workflows.", "The site promise says every judgment has an author and private user context should not leak.", ["demo:span:user-style:1"], now),
    Row("demo:decision:situation-causal-caveat", "cap_situation_ai_governance_demo", "review", "policy_editor", "Marked capacity causal relation as needs_review.", "Agency capacity claim is plausible but needs stronger operational evidence.", ["demo:span:output:1"], now),
    Row("demo:decision:tool-method-approved", "cap_tool_causal_policy_map_demo", "method_review", "method_reviewer", "Approved causal policy map as reusable Tool capsule.", "The method states traps and required primitives.", ["demo:span:tool:1"], now),
    Row("demo:decision:output-human-gate", "cap_output_ai_governance_brief_demo", "publication_gate", "human_reviewer", "Blocked PRAXIS publication until human approval.", "Output capsules must represent what good looks like, not silently publish drafts.", ["demo:span:output:1"], now),
]
spark.createDataFrame(build_history, "decision_id string, capsule_id string, build_phase string, actor string, decision string, rationale string, source_refs array<string>, decided_at timestamp").write.mode("append").saveAsTable(
    f"{catalog}.capsule_gold.memory_build_history"
)

runtime_contracts = [
    Row("demo:runtime:user", "cap_user_policy_director_demo", "PRAXIS", json.dumps({"T1": "assert if source-backed", "T2": "attribute", "T3": "hedge"}), "Cite user preference spans only when they materially shape private output.", json.dumps({"may_combine_with": ["SITUATION", "TOOL", "OUTPUT"], "privacy": "private"}), ["public_user_profile_disclosure", "unsupported_personalization"], "approved_with_caveats", now),
    Row("demo:runtime:situation", "cap_situation_ai_governance_demo", "PRAXIS", json.dumps({"approved": "assert with citation", "approved_with_caveats": "attribute and caveat", "needs_review": "surface as warning"}), "Cite all factual, temporal, and causal claims.", json.dumps({"may_combine_with": ["USER", "TOOL", "OUTPUT"], "show_disputes": True}), ["uncited_material_claim", "legal_conclusion_without_review"], "approved_with_caveats", now),
    Row("demo:runtime:tool", "cap_tool_causal_policy_map_demo", "PRAXIS", json.dumps({"method_step": "name device in margin", "trap": "warn explicitly"}), "Cite method source when the device shapes an answer.", json.dumps({"may_combine_with": ["USER", "SITUATION", "OUTPUT"], "device_attribution": True}), ["actor_motive_without_source", "mechanism_missing"], "approved", now),
    Row("demo:runtime:output", "cap_output_ai_governance_brief_demo", "PRAXIS", json.dumps({"approved_output": "reuse as exemplar", "needs_review": "do not publish"}), "Cite source spans and review verdicts before reuse.", json.dumps({"may_combine_with": ["USER", "SITUATION", "TOOL"], "publication_gate": "human"}), ["review_missing", "citation_appendix_missing"], "needs_review", now),
]
spark.createDataFrame(runtime_contracts, "contract_id string, capsule_id string, target_surface string, trust_rules_json string, citation_policy string, composition_rules_json string, stop_conditions array<string>, review_state string, created_at timestamp").write.mode("append").saveAsTable(
    f"{catalog}.capsule_gold.runtime_contracts"
)

quality = [
    Row("demo:eval:user", "cap_user_policy_director_demo", 0.93, 0.02, 0.91, "accepted_with_privacy_caveat", "Private user context captured as reviewable evidence.", now),
    Row("demo:eval:situation", "cap_situation_ai_governance_demo", 0.88, 0.08, 0.84, "accepted_with_caveats", "Situation has strong structure but needs real-source expansion.", now),
    Row("demo:eval:tool", "cap_tool_causal_policy_map_demo", 0.95, 0.01, 0.9, "accepted", "Method capsule is reusable across policy domains.", now),
    Row("demo:eval:output", "cap_output_ai_governance_brief_demo", 0.82, 0.12, 0.78, "needs_review", "Output should not be published until human approval.", now),
]
spark.createDataFrame(quality, "eval_id string, capsule_id string, citation_precision double, unsupported_claim_rate double, source_coverage double, human_acceptance string, notes string, measured_at timestamp").write.mode("append").saveAsTable(
    f"{catalog}.capsule_evals.capsule_quality_scores"
)

# COMMAND ----------

manifest_rows = spark.table(f"{catalog}.capsule_gold.capsule_manifests").where(
    "capsule_id IN ('" + "','".join(capsule_ids) + "')"
).collect()
claim_rows = spark.table(f"{catalog}.capsule_gold.capsule_claims").where(
    "capsule_id IN ('" + "','".join(capsule_ids) + "')"
).collect()
device_rows = spark.table(f"{catalog}.capsule_gold.reasoning_devices").where(
    "capsule_id IN ('" + "','".join(capsule_ids) + "')"
).collect()
runtime_rows = spark.table(f"{catalog}.capsule_gold.runtime_contracts").where(
    "capsule_id IN ('" + "','".join(capsule_ids) + "')"
).collect()

claims_by_capsule = {}
for claim in claim_rows:
    claims_by_capsule.setdefault(claim.capsule_id, []).append(
        {
            "claim_id": claim.claim_id,
            "text": claim.claim_text,
            "confidence": claim.confidence,
            "source_span_ids": claim.source_span_ids,
            "review_state": claim.review_state,
            "uncertainty": claim.uncertainty,
        }
    )

devices_by_capsule = {}
for device in device_rows:
    devices_by_capsule.setdefault(device.capsule_id, []).append(
        {
            "device_id": device.device_id,
            "label": device.label,
            "procedure_steps": device.procedure_steps,
            "traps": device.traps,
            "review_state": device.review_state,
        }
    )

runtime_by_capsule = {row.capsule_id: row for row in runtime_rows}

exports = []
for manifest in manifest_rows:
    pack = {
        "contract_version": "2026-06-24.tacitus-context-pack.v0",
        "capsule_id": manifest.capsule_id,
        "capsule_kind": manifest.capsule_kind,
        "title": manifest.title,
        "summary": manifest.summary,
        "bundle_digest": manifest.bundle_digest,
        "review_state": manifest.review_state,
        "freshness": manifest.freshness,
        "claims": claims_by_capsule.get(manifest.capsule_id, []),
        "reasoning_devices": devices_by_capsule.get(manifest.capsule_id, []),
        "agent_guidance": {
            "use_policy": "Use as PRAXIS context only; preserve caveats and cite source_span_ids.",
            "trust_rules": json.loads(runtime_by_capsule[manifest.capsule_id].trust_rules_json),
            "citation_policy": runtime_by_capsule[manifest.capsule_id].citation_policy,
            "composition_rules": json.loads(runtime_by_capsule[manifest.capsule_id].composition_rules_json),
            "stop_conditions": runtime_by_capsule[manifest.capsule_id].stop_conditions,
        },
        "databricks_tables": {
            "manifest": f"{catalog}.capsule_gold.capsule_manifests",
            "claims": f"{catalog}.capsule_gold.capsule_claims",
            "graph": f"{catalog}.capsule_silver.semantic_nodes / semantic_edges",
        },
    }
    exports.append(
        Row(
            export_id=f"demo:export:{manifest.capsule_id}",
            capsule_id=manifest.capsule_id,
            target_surface="PRAXIS",
            context_pack_json=json.dumps(pack, sort_keys=True),
            export_status="ready_for_praxis_import" if manifest.review_state != "needs_review" else "requires_human_review",
            exported_at=now,
        )
    )

spark.createDataFrame(exports).write.mode("append").saveAsTable(
    f"{catalog}.capsule_exports.context_pack_exports"
)

print("Built four TACITUS demo Context Capsules and PRAXIS context-pack exports.")

# Databricks notebook source
# MAGIC %md
# MAGIC # Load TACITUS Demo Inputs
# MAGIC
# MAGIC Loads synthetic, non-sensitive sources and human knowledge records for a
# MAGIC generic policy-analysis capsule factory. These are demo rows, but the
# MAGIC shape is designed for real source and expert knowledge ingestion.

# COMMAND ----------

from datetime import datetime
import json

from pyspark.sql import Row

dbutils.widgets.text("catalog", "dialectica")
catalog = dbutils.widgets.get("catalog")
now = datetime.utcnow()

demo_domain_ids = ["policy_analysis_v1", "expert_context_v1"]

spark.sql(
    f"DELETE FROM {catalog}.capsule_registry.domain_packs "
    "WHERE domain_pack_id IN ('policy_analysis_v1', 'expert_context_v1')"
)
spark.sql(
    f"DELETE FROM {catalog}.capsule_registry.ontology_blueprints "
    "WHERE domain_pack_id IN ('policy_analysis_v1', 'expert_context_v1')"
)
spark.sql(
    f"DELETE FROM {catalog}.capsule_registry.source_registry "
    "WHERE source_id LIKE 'demo:%'"
)
spark.sql(
    f"DELETE FROM {catalog}.capsule_bronze.raw_inputs "
    "WHERE input_id LIKE 'demo:%'"
)
spark.sql(
    f"DELETE FROM {catalog}.capsule_bronze.human_knowledge_records "
    "WHERE knowledge_id LIKE 'demo:%'"
)

# COMMAND ----------

domain_packs = [
    Row(
        domain_pack_id="policy_analysis_v1",
        name="Policy Analysis Domain Pack",
        market="policy, regulation, governance, strategy, and conflict analysis",
        ontology_focus=["actors", "claims", "events", "obligations", "causes", "constraints"],
        capsule_kinds=["USER", "SITUATION", "TOOL", "OUTPUT"],
        created_at=now,
        notes="Generic enough for broad TACITUS customers; specialized by ontology blueprint.",
    ),
    Row(
        domain_pack_id="expert_context_v1",
        name="Expert Context Domain Pack",
        market="expert teams encoding institutional methods and review standards",
        ontology_focus=["expertise", "rubrics", "decision thresholds", "style", "authority boundaries"],
        capsule_kinds=["USER", "TOOL"],
        created_at=now,
        notes="Captures human knowledge as evidence-backed, reviewable context.",
    ),
]
spark.createDataFrame(domain_packs).write.mode("append").saveAsTable(
    f"{catalog}.capsule_registry.domain_packs"
)

blueprints = [
    Row(
        ontology_id="ontology:policy-causal-analysis-v1",
        domain_pack_id="policy_analysis_v1",
        name="Policy causal analysis ontology",
        version="0.1.0",
        target_capsule_kind="SITUATION",
        core_classes=["Actor", "PolicyInstrument", "Claim", "Obligation", "Event", "Outcome", "Constraint"],
        temporal_model="valid_time + observed_time + deadline + supersession",
        causal_model="intervention -> mechanism -> affected actor -> outcome -> evidence",
        review_rubric="Every causal edge needs a mechanism, confidence, caveat, and evidence span.",
        created_at=now,
    ),
    Row(
        ontology_id="ontology:expert-operating-context-v1",
        domain_pack_id="expert_context_v1",
        name="Expert operating context ontology",
        version="0.1.0",
        target_capsule_kind="USER",
        core_classes=["ExpertRole", "Preference", "AuthorityBoundary", "ReviewStandard", "OutputContract"],
        temporal_model="preference valid_from + review expiry",
        causal_model="not causal by default; records influence on output behavior",
        review_rubric="Private human knowledge must carry scope and disclosure caveats.",
        created_at=now,
    ),
    Row(
        ontology_id="ontology:analytical-tool-v1",
        domain_pack_id="policy_analysis_v1",
        name="Analytical tool ontology",
        version="0.1.0",
        target_capsule_kind="TOOL",
        core_classes=["Method", "Step", "FailureMode", "RequiredEvidence", "OutputContract"],
        temporal_model="method version + deprecation + review expiry",
        causal_model="method step -> expected reasoning effect -> failure mode",
        review_rubric="Methods must state required inputs, forbidden uses, and failure modes.",
        created_at=now,
    ),
]
spark.createDataFrame(blueprints).write.mode("append").saveAsTable(
    f"{catalog}.capsule_registry.ontology_blueprints"
)

# COMMAND ----------

sources = [
    Row(
        source_id="demo:human:user-interview",
        source_kind="human_interview_note",
        title="Demo analyst onboarding interview",
        source_uri="demo://human/user-interview",
        rights_profile="private_user_context",
        license_status="internal_demo",
        allowed_uses=["private_praxis_context", "demo_context_pack"],
        created_at=now,
    ),
    Row(
        source_id="demo:human:expert-rubric",
        source_kind="expert_method_note",
        title="Demo causal policy mapping rubric",
        source_uri="demo://human/causal-policy-map-rubric",
        rights_profile="tacitus_method_demo",
        license_status="internal_demo",
        allowed_uses=["method_capsule", "demo_context_pack"],
        created_at=now,
    ),
    Row(
        source_id="demo:source:ai-act-summary",
        source_kind="public_policy_document",
        title="Demo AI governance implementation source",
        source_uri="demo://sources/ai-governance-implementation",
        rights_profile="demo_public_summary",
        license_status="synthetic_demo",
        allowed_uses=["situation_capsule", "output_capsule", "demo_context_pack"],
        created_at=now,
    ),
    Row(
        source_id="demo:source:agency-capacity-note",
        source_kind="internal_analysis_note",
        title="Demo agency capacity note",
        source_uri="demo://sources/agency-capacity",
        rights_profile="team_internal_reviewed",
        license_status="synthetic_demo",
        allowed_uses=["situation_capsule", "output_capsule"],
        created_at=now,
    ),
]
spark.createDataFrame(sources).write.mode("append").saveAsTable(
    f"{catalog}.capsule_registry.source_registry"
)

raw_inputs = [
    Row(
        input_id="demo:input:user-style",
        capsule_kind="USER",
        domain_pack_id="expert_context_v1",
        source_id="demo:human:user-interview",
        input_kind="human_knowledge",
        title="Analyst operating style",
        content="The analyst prefers source-led briefs, explicit caveats, causal assumptions, and a clear action line.",
        collected_at=now,
        metadata_json=json.dumps({"privacy": "private", "review_required": True}),
    ),
    Row(
        input_id="demo:input:situation-ai-governance",
        capsule_kind="SITUATION",
        domain_pack_id="policy_analysis_v1",
        source_id="demo:source:ai-act-summary",
        input_kind="policy_source",
        title="AI governance implementation situation",
        content="A regulator is preparing implementation guidance while agencies face staffing constraints and firms seek compliance certainty.",
        collected_at=now,
        metadata_json=json.dumps({"jurisdiction": "EU", "time_horizon": "next_180_days"}),
    ),
    Row(
        input_id="demo:input:causal-tool",
        capsule_kind="TOOL",
        domain_pack_id="policy_analysis_v1",
        source_id="demo:human:expert-rubric",
        input_kind="expert_method",
        title="Causal policy map method",
        content="Map intervention, mechanism, affected actor, intended outcome, unintended consequence, evidence strength, and review caveat.",
        collected_at=now,
        metadata_json=json.dumps({"method_version": "0.1.0"}),
    ),
    Row(
        input_id="demo:input:output-brief",
        capsule_kind="OUTPUT",
        domain_pack_id="policy_analysis_v1",
        source_id="demo:source:agency-capacity-note",
        input_kind="draft_output",
        title="AI governance readiness brief draft",
        content="The brief recommends a phased guidance calendar, agency capacity review, and stakeholder consultation checkpoint.",
        collected_at=now,
        metadata_json=json.dumps({"target_surface": "praxis_memo"}),
    ),
]
spark.createDataFrame(raw_inputs).write.mode("append").saveAsTable(
    f"{catalog}.capsule_bronze.raw_inputs"
)

human_knowledge = [
    Row(
        knowledge_id="demo:knowledge:user-review-standard",
        domain_pack_id="expert_context_v1",
        source_id="demo:human:user-interview",
        expert_role="policy_director",
        statement="Do not present causal conclusions unless the mechanism and evidence strength are explicit.",
        review_state="approved_with_caveats",
        private_scope="private_to_user_or_team",
        created_at=now,
    ),
    Row(
        knowledge_id="demo:knowledge:tool-failure-mode",
        domain_pack_id="policy_analysis_v1",
        source_id="demo:human:expert-rubric",
        expert_role="policy_method_reviewer",
        statement="A causal policy map fails if it hides uncertainty or treats correlation as mechanism.",
        review_state="approved",
        private_scope="shareable_method_demo",
        created_at=now,
    ),
]
spark.createDataFrame(human_knowledge).write.mode("append").saveAsTable(
    f"{catalog}.capsule_bronze.human_knowledge_records"
)

print("Loaded TACITUS demo domain packs, source registry, raw inputs, and human knowledge records.")

# Databricks notebook source
# MAGIC %md
# MAGIC # Build Guided Capsule Builder Flow
# MAGIC
# MAGIC This notebook turns the base capsule tables into a concrete Databricks
# MAGIC showcase flow:
# MAGIC
# MAGIC 1. A PRAXIS user asks for a Context Capsule.
# MAGIC 2. The builder records permitted sources and interview turns.
# MAGIC 3. Databricks expands evidence into ontology, semantic, temporal, causal,
# MAGIC    and similarity layers.
# MAGIC 4. The workflow emits reviewable proposals and agent guidance frames that
# MAGIC    constrain PRAXIS generation.

# COMMAND ----------

from datetime import datetime, timedelta
import hashlib
import json

from pyspark.sql import Row

dbutils.widgets.text("catalog", "dialectica")
catalog = dbutils.widgets.get("catalog")
now = datetime.utcnow()

session_id = "demo:builder-session:ai-procurement-brief"
capsule_ids = [
    "cap_user_policy_director_demo",
    "cap_situation_ai_governance_demo",
    "cap_tool_causal_policy_map_demo",
    "cap_output_ai_governance_brief_demo",
]

for statement in [
    f"DELETE FROM {catalog}.capsule_bronze.capsule_builder_sessions WHERE session_id = '{session_id}'",
    f"DELETE FROM {catalog}.capsule_bronze.builder_interview_turns WHERE session_id = '{session_id}'",
    f"DELETE FROM {catalog}.capsule_bronze.source_connector_requests WHERE session_id = '{session_id}'",
    f"DELETE FROM {catalog}.capsule_silver.capsule_similarity_matches WHERE session_id = '{session_id}'",
    f"DELETE FROM {catalog}.capsule_gold.capsule_improvement_proposals WHERE session_id = '{session_id}'",
    f"DELETE FROM {catalog}.capsule_gold.agent_guidance_frames WHERE session_id = '{session_id}'",
    f"DELETE FROM {catalog}.capsule_silver.graph_partitions WHERE partition_id LIKE 'demo:partition:%'",
    f"DELETE FROM {catalog}.capsule_silver.ontology_terms WHERE term_id LIKE 'demo:deep:%'",
    f"DELETE FROM {catalog}.capsule_silver.semantic_nodes WHERE node_id LIKE 'demo:deep:%'",
    f"DELETE FROM {catalog}.capsule_silver.semantic_edges WHERE edge_id LIKE 'demo:deep:%'",
    f"DELETE FROM {catalog}.capsule_silver.temporal_relations WHERE temporal_relation_id LIKE 'demo:deep:%'",
    f"DELETE FROM {catalog}.capsule_silver.causal_relations WHERE causal_relation_id LIKE 'demo:deep:%'",
    f"DELETE FROM {catalog}.capsule_exports.context_pack_exports WHERE export_id LIKE 'demo:export:guided:%'",
]:
    spark.sql(statement)

# COMMAND ----------

session = Row(
    session_id=session_id,
    requested_by="demo_user_policy_director",
    target_surface="PRAXIS Ask + Capsule Library",
    requested_capsule_kind="SITUATION",
    user_prompt=(
        "Build a situation capsule for an AI procurement policy brief. Use my style, "
        "similar prior reasoning, the causal policy map tool, and produce something "
        "PRAXIS agents can use without inventing unsupported claims."
    ),
    consent_profile="demo_explicit_sources_only",
    source_policy_json=json.dumps(
        {
            "allowed": ["uploaded_documents", "demo_prior_outputs", "human_interview_notes", "approved_source_registry"],
            "blocked": ["private_email_body_without_oauth_consent", "unreviewed_personal_profile_export"],
            "raw_text_export": "blocked_by_default",
            "praxis_write_policy": "review_card_only",
        },
        sort_keys=True,
    ),
    status="compiled_reviewable_context",
    created_at=now,
)
spark.createDataFrame([session]).write.mode("append").saveAsTable(
    f"{catalog}.capsule_bronze.capsule_builder_sessions"
)

turns = [
    Row(
        turn_id="demo:turn:001",
        session_id=session_id,
        speaker="user",
        message="I need a brief that is cautious, source-led, and clear about uncertainty.",
        captured_signal_kind="user_preference",
        proposed_capsule_field="USER.memoryPosture.canonicalCapsuleFacts",
        confidence=0.9,
        created_at=now,
    ),
    Row(
        turn_id="demo:turn:002",
        session_id=session_id,
        speaker="builder_ai",
        message="I will treat causal claims as review-required unless mechanism and evidence strength are explicit.",
        captured_signal_kind="agent_operating_rule",
        proposed_capsule_field="TOOL.agentOperatingContract.stopConditions",
        confidence=0.88,
        created_at=now + timedelta(seconds=15),
    ),
    Row(
        turn_id="demo:turn:003",
        session_id=session_id,
        speaker="user",
        message="Compare this with similar governance rollout situations, but do not overstate similarity.",
        captured_signal_kind="similarity_request",
        proposed_capsule_field="SITUATION.memoryPosture.advisoryRecallPolicy",
        confidence=0.84,
        created_at=now + timedelta(seconds=30),
    ),
    Row(
        turn_id="demo:turn:004",
        session_id=session_id,
        speaker="builder_ai",
        message="I found reusable structure: actors, implementation guidance, agency capacity, deadlines, and stakeholder consultation.",
        captured_signal_kind="ontology_plan",
        proposed_capsule_field="SITUATION.ontologyBinding",
        confidence=0.86,
        created_at=now + timedelta(seconds=45),
    ),
]
spark.createDataFrame(turns).write.mode("append").saveAsTable(
    f"{catalog}.capsule_bronze.builder_interview_turns"
)

connectors = [
    Row("demo:connector:upload", session_id, "document_upload", "Uploaded procurement notes", "selected_files_only", "ready", True, "User-approved files land in bronze raw inputs.", now),
    Row("demo:connector:praxis-history", session_id, "praxis_history", "Prior PRAXIS outputs", "capsule_and_output_receipts", "ready", True, "Only approved prior output capsules and receipts.", now),
    Row("demo:connector:gmail", session_id, "gmail_oauth", "Gmail policy correspondence", "metadata_and_selected_threads", "consent_required", True, "Not connected in demo; shown as governed future connector.", now),
    Row("demo:connector:ladybug", session_id, "postgres_ladybug", "Ladybug graph/Postgres", "capsule_graph_projection", "planned", True, "Databricks consumes reviewed graph snapshots, not app canonical state.", now),
]
spark.createDataFrame(
    connectors,
    "connector_request_id string, session_id string, connector_kind string, connector_label string, data_scope string, status string, consent_required boolean, notes string, created_at timestamp",
).write.mode("append").saveAsTable(f"{catalog}.capsule_bronze.source_connector_requests")

# COMMAND ----------

matches = [
    Row("demo:match:user-style", session_id, "cap_user_policy_director_demo", "user_preference", 0.93, "Same analyst preference for source-led caveated policy briefs.", "private_context_only", now),
    Row("demo:match:similar-situation", session_id, "cap_situation_ai_governance_demo", "situation_similarity", 0.86, "Same policy implementation pattern: guidance bottleneck plus agency capacity constraint.", "reuse_structure_not_facts", now),
    Row("demo:match:method", session_id, "cap_tool_causal_policy_map_demo", "reasoning_method", 0.91, "Causal policy map is appropriate because the user asks for mechanism-aware analysis.", "approved_tool_capsule", now),
    Row("demo:match:output-shape", session_id, "cap_output_ai_governance_brief_demo", "output_exemplar", 0.79, "Prior readiness brief offers a useful structure but remains review gated.", "reuse_as_format_only", now),
]
spark.createDataFrame(
    matches,
    "match_id string, session_id string, candidate_capsule_id string, match_kind string, similarity_score double, match_rationale string, reuse_policy string, created_at timestamp",
).write.mode("append").saveAsTable(f"{catalog}.capsule_silver.capsule_similarity_matches")

# COMMAND ----------

deep_terms = []
for idx, label in enumerate(
    [
        "Procurement authority",
        "Vendor assurance",
        "Model risk",
        "Auditability",
        "Budget constraint",
        "Implementation deadline",
        "Stakeholder consultation",
        "Public accountability",
        "Operational capacity",
        "Review threshold",
        "Evidence strength",
        "Causal mechanism",
    ],
    start=1,
):
    deep_terms.append(
        Row(
            term_id=f"demo:deep:term:{idx:03d}",
            domain_pack_id="policy_analysis_v1",
            label=label,
            definition=f"Demo ontology term for {label.lower()} in capsule-guided policy analysis.",
            parent_term_id=None,
            capsule_kind="SITUATION",
            review_state="candidate_reviewable",
            source_span_ids=["demo:span:situation:1", "demo:span:tool:1"],
            created_at=now,
        )
    )
spark.createDataFrame(
    deep_terms,
    "term_id string, domain_pack_id string, label string, definition string, parent_term_id string, capsule_kind string, review_state string, source_span_ids array<string>, created_at timestamp",
).write.mode("append").saveAsTable(f"{catalog}.capsule_silver.ontology_terms")

node_types = ["Actor", "Claim", "Source", "Event", "Constraint", "Preference", "ToolStep", "OutputContract"]
edge_types = ["supports", "constrains", "requires_review", "precedes", "uses_method", "inherits_preference"]

deep_nodes = []
deep_edges = []
deep_temporal = []
deep_causal = []
partitions = []

for capsule_index, capsule_id in enumerate(capsule_ids):
    ontology_id = {
        "cap_user_policy_director_demo": "ontology:expert-operating-context-v1",
        "cap_situation_ai_governance_demo": "ontology:policy-causal-analysis-v1",
        "cap_tool_causal_policy_map_demo": "ontology:analytical-tool-v1",
        "cap_output_ai_governance_brief_demo": "ontology:policy-causal-analysis-v1",
    }[capsule_id]
    node_total = 250
    edge_total = 375
    partitions.append(
        Row(
            partition_id=f"demo:partition:{capsule_id}:scale",
            capsule_id=capsule_id,
            partition_kind="demo_scale_projection",
            ontology_id=ontology_id,
            node_count=node_total,
            edge_count=edge_total,
            review_state="candidate_reviewable",
            materialization_policy="advisory_until_praxis_review",
            created_at=now,
        )
    )

    for idx in range(node_total):
        node_id = f"demo:deep:node:{capsule_id}:{idx:04d}"
        node_type = node_types[(idx + capsule_index) % len(node_types)]
        deep_nodes.append(
            Row(
                node_id=node_id,
                capsule_id=capsule_id,
                domain_pack_id="policy_analysis_v1" if "user" not in capsule_id else "expert_context_v1",
                node_type=node_type,
                label=f"{node_type} {idx:04d} for {capsule_id}",
                source_span_ids=["demo:span:situation:1"] if "situation" in capsule_id else ["demo:span:tool:1"],
                properties_json=json.dumps(
                    {
                        "demo_scale": True,
                        "ontology_id": ontology_id,
                        "rank": idx,
                        "review_policy": "proposal_only",
                    },
                    sort_keys=True,
                ),
                review_state="candidate_reviewable",
                created_at=now,
            )
        )

    for idx in range(edge_total):
        from_idx = idx % node_total
        to_idx = (idx * 7 + 13) % node_total
        edge_type = edge_types[(idx + capsule_index) % len(edge_types)]
        deep_edges.append(
            Row(
                edge_id=f"demo:deep:edge:{capsule_id}:{idx:04d}",
                capsule_id=capsule_id,
                edge_type=edge_type,
                from_node_id=f"demo:deep:node:{capsule_id}:{from_idx:04d}",
                to_node_id=f"demo:deep:node:{capsule_id}:{to_idx:04d}",
                source_span_ids=["demo:span:situation:1", "demo:span:tool:1"],
                confidence=round(0.52 + ((idx % 43) / 100), 2),
                temporal_scope_json=json.dumps({"known_at": now.isoformat(), "validity": "demo_candidate"}, sort_keys=True),
                causal_scope_json=json.dumps({"mechanism_required": edge_type in ["supports", "constrains"]}, sort_keys=True),
                review_state="candidate_reviewable",
                created_at=now,
            )
        )

    for idx in range(12):
        deep_temporal.append(
            Row(
                temporal_relation_id=f"demo:deep:time:{capsule_id}:{idx:03d}",
                capsule_id=capsule_id,
                relation_type="validity_window",
                subject_id=f"demo:deep:node:{capsule_id}:{idx:04d}",
                object_id=f"demo:deep:node:{capsule_id}:{(idx + 1):04d}",
                valid_from=now + timedelta(days=idx),
                valid_until=now + timedelta(days=idx + 90),
                known_at=now,
                allen_relation="DURING",
                uncertainty="medium" if idx % 3 == 0 else "low",
                source_span_ids=["demo:span:situation:1"],
                review_state="candidate_reviewable",
            )
        )
        deep_causal.append(
            Row(
                causal_relation_id=f"demo:deep:cause:{capsule_id}:{idx:03d}",
                capsule_id=capsule_id,
                cause_id=f"demo:deep:node:{capsule_id}:{idx:04d}",
                effect_id=f"demo:deep:node:{capsule_id}:{(idx * 3 + 5) % node_total:04d}",
                mechanism=f"Demo mechanism {idx} requires source-backed review before PRAXIS use.",
                confidence=round(0.55 + (idx / 40), 2),
                assumptions=["synthetic scale row", "requires human review", "not canonical"],
                source_span_ids=["demo:span:tool:1"],
                review_state="candidate_reviewable",
            )
        )

spark.createDataFrame(partitions).write.mode("append").saveAsTable(
    f"{catalog}.capsule_silver.graph_partitions"
)
spark.createDataFrame(
    deep_nodes,
    "node_id string, capsule_id string, domain_pack_id string, node_type string, label string, source_span_ids array<string>, properties_json string, review_state string, created_at timestamp",
).write.mode("append").saveAsTable(f"{catalog}.capsule_silver.semantic_nodes")
spark.createDataFrame(
    deep_edges,
    "edge_id string, capsule_id string, edge_type string, from_node_id string, to_node_id string, source_span_ids array<string>, confidence double, temporal_scope_json string, causal_scope_json string, review_state string, created_at timestamp",
).write.mode("append").saveAsTable(f"{catalog}.capsule_silver.semantic_edges")
spark.createDataFrame(
    deep_temporal,
    "temporal_relation_id string, capsule_id string, relation_type string, subject_id string, object_id string, valid_from timestamp, valid_until timestamp, known_at timestamp, allen_relation string, uncertainty string, source_span_ids array<string>, review_state string",
).write.mode("append").saveAsTable(f"{catalog}.capsule_silver.temporal_relations")
spark.createDataFrame(
    deep_causal,
    "causal_relation_id string, capsule_id string, cause_id string, effect_id string, mechanism string, confidence double, assumptions array<string>, source_span_ids array<string>, review_state string",
).write.mode("append").saveAsTable(f"{catalog}.capsule_silver.causal_relations")

# COMMAND ----------

proposals = []
frames = []
exports = []

for capsule_id in capsule_ids:
    proposal_layers = [
        ("ontology_binding", "ontology", "Bind domain-specific ontology stack"),
        ("graph_projection", "semantic_graph", "Attach graph partition as reviewable Capsule+ proposal"),
        ("temporal_scope", "temporality", "Add valid/known time windows to capsule reasoning"),
        ("agent_controls", "agent_guidance", "Constrain PRAXIS generation with capsule-specific controls"),
    ]
    for kind, layer, title in proposal_layers:
        proposal_payload = {
            "session_id": session_id,
            "capsule_id": capsule_id,
            "proposal_kind": kind,
            "target_layer": layer,
            "review_policy": "human_review_required_before_canonical_promotion",
            "source_tables": [
                f"{catalog}.capsule_silver.semantic_nodes",
                f"{catalog}.capsule_silver.semantic_edges",
                f"{catalog}.capsule_silver.temporal_relations",
                f"{catalog}.capsule_silver.causal_relations",
            ],
        }
        proposals.append(
            Row(
                proposal_id=f"demo:proposal:{capsule_id}:{kind}",
                session_id=session_id,
                capsule_id=capsule_id,
                proposal_kind=kind,
                target_layer=layer,
                title=title,
                proposal_json=json.dumps(proposal_payload, sort_keys=True),
                review_state="proposed_for_praxis_review",
                created_at=now,
            )
        )

    controls = {
        "must_cite_source_spans": True,
        "must_preserve_review_state": True,
        "must_surface_temporal_uncertainty": capsule_id != "cap_tool_causal_policy_map_demo",
        "must_name_causal_mechanism": capsule_id in ["cap_situation_ai_governance_demo", "cap_tool_causal_policy_map_demo"],
        "must_not_publish_if_needs_review": True,
        "private_user_context_policy": "do_not_reveal_user_profile_in_public_output",
    }
    retrieval_contract = {
        "primary_tables": [
            f"{catalog}.capsule_gold.capsule_claims",
            f"{catalog}.capsule_silver.evidence_spans",
            f"{catalog}.capsule_silver.semantic_nodes",
        ],
        "search_policy": "hybrid semantic and structured filters when AI Search is enabled",
        "ranking_hints": ["review_state", "source_coverage", "temporal_freshness", "similarity_score"],
    }
    generation_space = {
        "allowed": ["summarize", "compare", "draft_with_citations", "flag_gaps", "propose_review_cards"],
        "blocked": ["invent_sources", "promote_canonical_capsule_facts", "publish_unreviewed_output", "expose_private_preferences"],
        "fallback": "ask_user_or_create_review_card",
    }
    frame_payload = {
        "capsule_id": capsule_id,
        "deterministic_controls": controls,
        "retrieval_contract": retrieval_contract,
        "allowed_generation_space": generation_space,
    }
    frame_digest = hashlib.sha256(json.dumps(frame_payload, sort_keys=True).encode()).hexdigest()
    frames.append(
        Row(
            frame_id=f"demo:frame:{capsule_id}",
            session_id=session_id,
            capsule_id=capsule_id,
            target_agent="PRAXIS Ask / Research Memo Agent",
            deterministic_controls_json=json.dumps(controls, sort_keys=True),
            retrieval_contract_json=json.dumps(retrieval_contract, sort_keys=True),
            allowed_generation_space_json=json.dumps(generation_space, sort_keys=True),
            review_required=True,
            created_at=now,
        )
    )
    exports.append(
        Row(
            export_id=f"demo:export:guided:{capsule_id}",
            capsule_id=capsule_id,
            target_surface="PRAXIS_GUIDED_BUILDER",
            context_pack_json=json.dumps(
                {
                    "contract_version": "2026-06-24.tacitus-guided-builder.v0",
                    "session_id": session_id,
                    "capsule_id": capsule_id,
                    "frame_digest": f"sha256:{frame_digest}",
                    "review_policy": "import_as_review_cards_only",
                    "agent_guidance_frame": frame_payload,
                    "databricks_lineage_tables": {
                        "builder_session": f"{catalog}.capsule_bronze.capsule_builder_sessions",
                        "interview_turns": f"{catalog}.capsule_bronze.builder_interview_turns",
                        "graph_partitions": f"{catalog}.capsule_silver.graph_partitions",
                        "proposals": f"{catalog}.capsule_gold.capsule_improvement_proposals",
                    },
                },
                sort_keys=True,
            ),
            export_status="ready_for_praxis_review_import",
            exported_at=now,
        )
    )

spark.createDataFrame(
    proposals,
    "proposal_id string, session_id string, capsule_id string, proposal_kind string, target_layer string, title string, proposal_json string, review_state string, created_at timestamp",
).write.mode("append").saveAsTable(f"{catalog}.capsule_gold.capsule_improvement_proposals")
spark.createDataFrame(
    frames,
    "frame_id string, session_id string, capsule_id string, target_agent string, deterministic_controls_json string, retrieval_contract_json string, allowed_generation_space_json string, review_required boolean, created_at timestamp",
).write.mode("append").saveAsTable(f"{catalog}.capsule_gold.agent_guidance_frames")
spark.createDataFrame(
    exports,
    "export_id string, capsule_id string, target_surface string, context_pack_json string, export_status string, exported_at timestamp",
).write.mode("append").saveAsTable(f"{catalog}.capsule_exports.context_pack_exports")

# COMMAND ----------

for capsule_id in capsule_ids:
    counts = spark.sql(
        f"""
        SELECT
          (SELECT COUNT(*) FROM {catalog}.capsule_silver.semantic_nodes WHERE capsule_id = '{capsule_id}') AS node_count,
          (SELECT COUNT(*) FROM {catalog}.capsule_silver.semantic_edges WHERE capsule_id = '{capsule_id}') AS edge_count
        """
    ).collect()[0]
    spark.sql(
        f"""
        UPDATE {catalog}.capsule_gold.capsule_manifests
        SET graph_node_count = {int(counts.node_count)},
            graph_edge_count = {int(counts.edge_count)}
        WHERE capsule_id = '{capsule_id}'
        """
    )

print(
    "Built guided Capsule Builder session with "
    f"{len(deep_nodes)} graph nodes, {len(deep_edges)} graph edges, "
    f"{len(proposals)} reviewable proposals, and {len(frames)} PRAXIS agent frames."
)

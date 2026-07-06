# Databricks notebook source
# MAGIC %md
# MAGIC # TACITUS Context Capsule Factory Showcase
# MAGIC
# MAGIC Use this notebook as the Databricks-page demo. It shows a generic capsule
# MAGIC factory that can support policy, regulation, conflict, climate, public
# MAGIC health, legal, and strategy analysis.

# COMMAND ----------

dbutils.widgets.text("catalog", "dialectica")
catalog = dbutils.widgets.get("catalog")

print("TACITUS Context Capsule Factory")
print(f"Catalog: {catalog}")
print("Showcase tables:")
print(f"- {catalog}.capsule_gold.capsule_showcase")
print(f"- {catalog}.capsule_exports.context_pack_exports")
print(f"- {catalog}.capsule_silver.semantic_nodes")
print(f"- {catalog}.capsule_silver.semantic_edges")
print(f"- {catalog}.capsule_silver.temporal_relations")
print(f"- {catalog}.capsule_silver.causal_relations")
print(f"- {catalog}.capsule_gold.reasoning_devices")
print(f"- {catalog}.capsule_gold.memory_build_history")
print(f"- {catalog}.capsule_gold.runtime_contracts")
print(f"- {catalog}.capsule_bronze.capsule_builder_sessions")
print(f"- {catalog}.capsule_gold.agent_guidance_frames")
print(f"- {catalog}.capsule_gold.guided_builder_showcase")

# COMMAND ----------

# MAGIC %md
# MAGIC ## 1. Capsule Portfolio

# COMMAND ----------

display(spark.sql(f"""
SELECT *
FROM {catalog}.capsule_gold.capsule_showcase
ORDER BY
  CASE capsule_kind
    WHEN 'USER' THEN 1
    WHEN 'SITUATION' THEN 2
    WHEN 'TOOL' THEN 3
    WHEN 'OUTPUT' THEN 4
    ELSE 5
  END
"""))

# COMMAND ----------

# MAGIC %md
# MAGIC ## 2. Ontology Terms

# COMMAND ----------

display(spark.sql(f"""
SELECT capsule_kind, label, definition, review_state, source_span_ids
FROM {catalog}.capsule_silver.ontology_terms
WHERE term_id LIKE 'demo:%'
ORDER BY capsule_kind, label
"""))

# COMMAND ----------

# MAGIC %md
# MAGIC ## 3. Semantic Graph

# COMMAND ----------

display(spark.sql(f"""
SELECT capsule_id, node_type, label, review_state, source_span_ids
FROM {catalog}.capsule_silver.semantic_nodes
WHERE capsule_id LIKE 'cap_%_demo'
ORDER BY capsule_id, node_type, label
"""))

display(spark.sql(f"""
SELECT capsule_id, edge_type, from_node_id, to_node_id, confidence, review_state, causal_scope_json
FROM {catalog}.capsule_silver.semantic_edges
WHERE capsule_id LIKE 'cap_%_demo'
ORDER BY capsule_id, edge_type
"""))

# COMMAND ----------

# MAGIC %md
# MAGIC ## 4. Temporal And Causal Layers

# COMMAND ----------

display(spark.sql(f"""
SELECT capsule_id, relation_type, subject_id, object_id, valid_from, known_at, valid_until, allen_relation, uncertainty, review_state
FROM {catalog}.capsule_silver.temporal_relations
WHERE capsule_id LIKE 'cap_%_demo'
ORDER BY capsule_id, relation_type
"""))

display(spark.sql(f"""
SELECT capsule_id, cause_id, effect_id, mechanism, confidence, assumptions, review_state
FROM {catalog}.capsule_silver.causal_relations
WHERE capsule_id LIKE 'cap_%_demo'
ORDER BY capsule_id, confidence DESC
"""))

# COMMAND ----------

# MAGIC %md
# MAGIC ## 5. Reasoning Devices, Build Memory, Governance, Runtime Rules

# COMMAND ----------

display(spark.sql(f"""
SELECT capsule_id, label, device_kind, procedure_steps, traps, required_primitives, review_state
FROM {catalog}.capsule_gold.reasoning_devices
WHERE capsule_id LIKE 'cap_%_demo'
ORDER BY capsule_id, label
"""))

display(spark.sql(f"""
SELECT capsule_id, build_phase, actor, decision, rationale, source_refs
FROM {catalog}.capsule_gold.memory_build_history
WHERE capsule_id LIKE 'cap_%_demo'
ORDER BY capsule_id, decided_at
"""))

display(spark.sql(f"""
SELECT capsule_id, target_surface, trust_rules_json, citation_policy, composition_rules_json, stop_conditions, review_state
FROM {catalog}.capsule_gold.runtime_contracts
WHERE capsule_id LIKE 'cap_%_demo'
ORDER BY capsule_id
"""))

# COMMAND ----------

# MAGIC %md
# MAGIC ## 6. PRAXIS Context-Pack Exports

# COMMAND ----------

display(spark.sql(f"""
SELECT capsule_id, target_surface, export_status, context_pack_json
FROM {catalog}.capsule_exports.context_pack_exports
WHERE capsule_id LIKE 'cap_%_demo'
ORDER BY capsule_id
"""))

# COMMAND ----------

# MAGIC %md
# MAGIC ## 7. Guided Builder Session

# COMMAND ----------

display(spark.sql(f"""
SELECT session_id, requested_capsule_kind, requested_by, target_surface, user_prompt, consent_profile, status
FROM {catalog}.capsule_bronze.capsule_builder_sessions
WHERE session_id LIKE 'demo:%'
ORDER BY created_at DESC
"""))

display(spark.sql(f"""
SELECT turn_id, speaker, message, captured_signal_kind, proposed_capsule_field, confidence
FROM {catalog}.capsule_bronze.builder_interview_turns
WHERE session_id LIKE 'demo:%'
ORDER BY created_at
"""))

display(spark.sql(f"""
SELECT connector_kind, connector_label, data_scope, status, consent_required, notes
FROM {catalog}.capsule_bronze.source_connector_requests
WHERE session_id LIKE 'demo:%'
ORDER BY connector_kind
"""))

# COMMAND ----------

# MAGIC %md
# MAGIC ## 8. Similarity, Scale, And Reviewable Improvements

# COMMAND ----------

display(spark.sql(f"""
SELECT candidate_capsule_id, match_kind, similarity_score, match_rationale, reuse_policy
FROM {catalog}.capsule_silver.capsule_similarity_matches
WHERE session_id LIKE 'demo:%'
ORDER BY similarity_score DESC
"""))

display(spark.sql(f"""
SELECT capsule_id, partition_kind, ontology_id, node_count, edge_count, review_state, materialization_policy
FROM {catalog}.capsule_silver.graph_partitions
WHERE partition_id LIKE 'demo:%'
ORDER BY capsule_id
"""))

display(spark.sql(f"""
SELECT capsule_id, proposal_kind, target_layer, title, review_state
FROM {catalog}.capsule_gold.capsule_improvement_proposals
WHERE session_id LIKE 'demo:%'
ORDER BY capsule_id, target_layer
"""))

# COMMAND ----------

# MAGIC %md
# MAGIC ## 9. Agent Guidance Frames

# COMMAND ----------

display(spark.sql(f"""
SELECT capsule_id, target_agent, deterministic_controls_json, retrieval_contract_json, allowed_generation_space_json, review_required
FROM {catalog}.capsule_gold.agent_guidance_frames
WHERE session_id LIKE 'demo:%'
ORDER BY capsule_id
"""))

display(spark.sql(f"""
SELECT *
FROM {catalog}.capsule_gold.guided_builder_showcase
WHERE session_id LIKE 'demo:%'
ORDER BY requested_capsule_kind, capsule_id, target_layer
"""))

# COMMAND ----------

# MAGIC %md
# MAGIC ## 10. Demo Narrative
# MAGIC
# MAGIC - `USER` capsule: captures human expertise, working style, privacy caveats,
# MAGIC   and review thresholds.
# MAGIC - `SITUATION` capsule: structures a policy situation with ontology terms,
# MAGIC   events, constraints, temporal windows, and causal assumptions.
# MAGIC - `TOOL` capsule: packages an expert method as a reusable, reviewable
# MAGIC   reasoning device.
# MAGIC - `OUTPUT` capsule: turns a reviewed artifact into reusable PRAXIS context,
# MAGIC   while preserving review status and source caveats.
# MAGIC - The two clocks are explicit: `valid_from` / `valid_until` track what
# MAGIC   happened or applies; `known_at` tracks when the desk learned it.
# MAGIC - Allen-style interval labels make timing queryable rather than prose-only.
# MAGIC - Reasoning devices and traps carry expert methods into PRAXIS with
# MAGIC   attribution.
# MAGIC - The guided builder session shows how a user can talk to an AI builder
# MAGIC   while Databricks records the source policy, consent boundary, interview
# MAGIC   signals, similarity matches, graph partitions, reviewable proposals, and
# MAGIC   final agent guidance frames.
# MAGIC - Graph scale is visible through `graph_partitions` plus the expanded
# MAGIC   semantic node/edge tables. The rows are reviewable candidate projections,
# MAGIC   not PRAXIS canonical state.
# MAGIC
# MAGIC This is the broad TACITUS wedge: verified context infrastructure for expert
# MAGIC work, with PRAXIS as the cockpit and DIALECTICA as the compiler.

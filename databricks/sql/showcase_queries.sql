-- TACITUS Context Capsule Factory showcase queries.
-- Set the catalog in the SQL editor if needed:
-- USE CATALOG dialectica;

SELECT *
FROM dialectica.capsule_gold.dashboard_kpis;

SELECT *
FROM dialectica.capsule_gold.dashboard_showcase_click_path
ORDER BY step_id;

SELECT *
FROM dialectica.capsule_gold.dashboard_platform_story
ORDER BY maturity, databricks_surface;

SELECT *
FROM dialectica.capsule_gold.dashboard_ai_extraction_lab
ORDER BY capsule_kind;

SELECT *
FROM dialectica.capsule_gold.dashboard_ontology_showroom
ORDER BY capsule_kind;

SELECT *
FROM dialectica.capsule_gold.dashboard_capsule_deep_dive
ORDER BY capsule_kind;

SELECT *
FROM dialectica.capsule_gold.dashboard_showroom_narrative
ORDER BY section;

SELECT *
FROM dialectica.capsule_gold.dashboard_agent_console
ORDER BY section, item_id;

SELECT *
FROM dialectica.capsule_gold.dashboard_mcp_contracts
ORDER BY mcp_tool_name;

SELECT *
FROM dialectica.capsule_gold.dashboard_agent_feedback_queue
ORDER BY created_at DESC;

SELECT *
FROM dialectica.capsule_gold.dashboard_capsule_portfolio
ORDER BY capsule_kind;

SELECT *
FROM dialectica.capsule_gold.dashboard_graph_scale
ORDER BY capsule_kind;

SELECT *
FROM dialectica.capsule_gold.dashboard_review_workbench
ORDER BY capsule_kind, target_layer;

SELECT *
FROM dialectica.capsule_gold.dashboard_agent_guidance
ORDER BY capsule_kind;

SELECT *
FROM dialectica.capsule_gold.dashboard_connector_boundary
ORDER BY connector_kind;

SELECT corpus_id, capsule_id, source_layer, title, review_state, source_refs
FROM dialectica.capsule_gold.ai_search_corpus
ORDER BY source_layer, corpus_id
LIMIT 50;

SELECT *
FROM dialectica.capsule_gold.capsule_showcase
ORDER BY capsule_kind;

SELECT capsule_id, claim_text, confidence, review_state, source_span_ids
FROM dialectica.capsule_gold.capsule_claims
WHERE capsule_id LIKE 'cap_%_demo'
ORDER BY capsule_id, confidence DESC;

SELECT capsule_id, edge_type, from_node_id, to_node_id, confidence, review_state
FROM dialectica.capsule_silver.semantic_edges
WHERE capsule_id LIKE 'cap_%_demo'
ORDER BY capsule_id, edge_type;

SELECT capsule_id, cause_id, effect_id, mechanism, confidence, review_state
FROM dialectica.capsule_silver.causal_relations
WHERE capsule_id LIKE 'cap_%_demo'
ORDER BY confidence DESC;

SELECT capsule_id, relation_type, valid_from, known_at, valid_until, allen_relation, review_state
FROM dialectica.capsule_silver.temporal_relations
WHERE capsule_id LIKE 'cap_%_demo'
ORDER BY capsule_id, relation_type;

SELECT capsule_id, label, procedure_steps, traps, review_state
FROM dialectica.capsule_gold.reasoning_devices
WHERE capsule_id LIKE 'cap_%_demo'
ORDER BY capsule_id, label;

SELECT capsule_id, target_surface, trust_rules_json, citation_policy, stop_conditions
FROM dialectica.capsule_gold.runtime_contracts
WHERE capsule_id LIKE 'cap_%_demo'
ORDER BY capsule_id;

SELECT capsule_id, export_status, context_pack_json
FROM dialectica.capsule_exports.context_pack_exports
WHERE capsule_id LIKE 'cap_%_demo';

SELECT session_id, requested_capsule_kind, requested_by, target_surface, user_prompt, consent_profile, status
FROM dialectica.capsule_bronze.capsule_builder_sessions
WHERE session_id LIKE 'demo:%'
ORDER BY created_at DESC;

SELECT speaker, message, captured_signal_kind, proposed_capsule_field, confidence
FROM dialectica.capsule_bronze.builder_interview_turns
WHERE session_id LIKE 'demo:%'
ORDER BY created_at;

SELECT connector_kind, connector_label, data_scope, status, consent_required, notes
FROM dialectica.capsule_bronze.source_connector_requests
WHERE session_id LIKE 'demo:%'
ORDER BY connector_kind;

SELECT candidate_capsule_id, match_kind, similarity_score, match_rationale, reuse_policy
FROM dialectica.capsule_silver.capsule_similarity_matches
WHERE session_id LIKE 'demo:%'
ORDER BY similarity_score DESC;

SELECT capsule_id, partition_kind, ontology_id, node_count, edge_count, review_state, materialization_policy
FROM dialectica.capsule_silver.graph_partitions
WHERE partition_id LIKE 'demo:%'
ORDER BY capsule_id;

SELECT capsule_id, proposal_kind, target_layer, title, review_state
FROM dialectica.capsule_gold.capsule_improvement_proposals
WHERE session_id LIKE 'demo:%'
ORDER BY capsule_id, target_layer;

SELECT capsule_id, target_agent, deterministic_controls_json, retrieval_contract_json, allowed_generation_space_json
FROM dialectica.capsule_gold.agent_guidance_frames
WHERE session_id LIKE 'demo:%'
ORDER BY capsule_id;

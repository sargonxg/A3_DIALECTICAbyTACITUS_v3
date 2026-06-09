# Graph Report - A3_DIALECTICAbyTACITUS_v3  (2026-06-09)

## Corpus Check
- 23 files · ~101,526 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 561 nodes · 1235 edges · 25 communities (20 shown, 5 thin omitted)
- Extraction: 92% EXTRACTED · 8% INFERRED · 0% AMBIGUOUS · INFERRED: 103 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `9eb9347f`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_Community 0|Community 0]]
- [[_COMMUNITY_Community 1|Community 1]]
- [[_COMMUNITY_Community 2|Community 2]]
- [[_COMMUNITY_Community 3|Community 3]]
- [[_COMMUNITY_Community 4|Community 4]]
- [[_COMMUNITY_Community 5|Community 5]]
- [[_COMMUNITY_Community 6|Community 6]]
- [[_COMMUNITY_Community 7|Community 7]]
- [[_COMMUNITY_Community 8|Community 8]]
- [[_COMMUNITY_Community 9|Community 9]]
- [[_COMMUNITY_Community 10|Community 10]]
- [[_COMMUNITY_Community 11|Community 11]]
- [[_COMMUNITY_Community 12|Community 12]]
- [[_COMMUNITY_Community 13|Community 13]]
- [[_COMMUNITY_Community 14|Community 14]]
- [[_COMMUNITY_Community 15|Community 15]]
- [[_COMMUNITY_Community 16|Community 16]]
- [[_COMMUNITY_Community 17|Community 17]]
- [[_COMMUNITY_Community 18|Community 18]]
- [[_COMMUNITY_Community 20|Community 20]]
- [[_COMMUNITY_Community 21|Community 21]]
- [[_COMMUNITY_Community 24|Community 24]]

## God Nodes (most connected - your core abstractions)
1. `main()` - 30 edges
2. `build_documents_capsule()` - 29 edges
3. `read_to_string()` - 20 edges
4. `validate_proposal_set()` - 19 edges
5. `write_package()` - 18 edges
6. `export_praxis_context_pack()` - 16 edges
7. `read_json()` - 15 edges
8. `call_tool()` - 15 edges
9. `load_source_pack()` - 14 edges
10. `promote_records()` - 14 edges

## Surprising Connections (you probably didn't know these)
- `build_capsule_tool()` --calls--> `build_documents_capsule()`  [INFERRED]
  services/dialectica-mcp/src/tools.rs → crates/dialectica-builder/src/lib.rs
- `build_capsule_tool()` --calls--> `build_documents_capsule()`  [INFERRED]
  services/dialectica-mcp/src/lib.rs → crates/dialectica-builder/src/lib.rs
- `build_capsule_tool()` --calls--> `parse_capsule_type()`  [INFERRED]
  services/dialectica-mcp/src/tools.rs → crates/dialectica-builder/src/lib.rs
- `build_capsule_tool()` --calls--> `parse_build_mode()`  [INFERRED]
  services/dialectica-mcp/src/tools.rs → crates/dialectica-builder/src/lib.rs
- `count_jsonl()` --calls--> `read_to_string()`  [INFERRED]
  services/dialectica-api/src/lib.rs → crates/dialectica-capsule/src/lib.rs

## Communities (25 total, 5 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.06
Nodes (61): BuildMode, BuildValidationFinding, BuildValidationReport, BuildValidationSeverity, CapsuleBuildPlan, CapsuleBuildRequest, CapsuleType, export_schema_dir() (+53 more)

### Community 1 - "Community 1"
Cohesion: 0.07
Nodes (48): add_graph_edges(), agent_context_markdown(), archive_rejects_output_inside_package_directory(), ArchiveReceipt, capsule_type_value(), claim_values(), collect_files(), compile_fixture() (+40 more)

### Community 2 - "Community 2"
Cohesion: 0.07
Nodes (44): build_ladybug_projection(), edge_insert_cypher(), edge_type(), GraphProjectionError, json_string(), JsonLdGraph, label(), LadybugProjectionPlan (+36 more)

### Community 3 - "Community 3"
Cohesion: 0.07
Nodes (46): auto_decision_set(), build_documents_capsule(), build_proposals(), BuildDocumentsOptions, BuildDocumentsReceipt, BuilderError, capture_source(), collect_document_files() (+38 more)

### Community 4 - "Community 4"
Cohesion: 0.08
Nodes (41): archive_capsule_tool(), build_capsule_tool(), call_tool(), default_workflow(), export_praxis_pack_tool(), handle_jsonrpc_line(), handle_jsonrpc_value(), initialize_advertises_dialectica_tools() (+33 more)

### Community 5 - "Community 5"
Cohesion: 0.09
Nodes (32): archive_capsule_tool(), build_capsule_tool(), call_tool(), capsule_status_tool(), capture_discussion_tool(), configured_roots(), default_workflow(), ensure_output_within_configured_roots() (+24 more)

### Community 6 - "Community 6"
Cohesion: 0.05
Nodes (40): AgentGuidance, capsule_type_blueprint(), CapsuleBundleIndex, CapsuleHealthReport, CapsuleInspection, CapsuleLayerIndex, CitationPolicy, default_graph_profile() (+32 more)

### Community 7 - "Community 7"
Cohesion: 0.12
Nodes (20): ApiError, ApiState, app(), build_graph_preview(), context_pack(), ContextPackQuery, count_jsonl(), default_fixture_dir() (+12 more)

### Community 8 - "Community 8"
Cohesion: 0.17
Nodes (23): load_build_request(), load_source_pack(), missing_required_review_decision_blocks_compilation(), rejected_review_decision_stays_out_of_context_pack(), canonical_situation_dir(), canonical_v3_capsule_rejects_extra_macro_types(), canonical_v3_situation_capsule_loads_and_validates(), golden_build_request_path() (+15 more)

### Community 9 - "Community 9"
Cohesion: 0.13
Nodes (17): approved_manifest_with_digest_is_export_ready(), assert_digest(), check_ladybug_projection(), draft_manifest_is_not_export_ready(), ProposalSet, read_json(), read_jsonl(), ReviewerDecisionSet (+9 more)

### Community 10 - "Community 10"
Cohesion: 0.21
Nodes (13): is_approved_capsule_type(), read_optional_string(), ReviewState, validate_graph_slice(), validate_json_file(), validate_language_profile(), validate_manifest(), validate_mimetype() (+5 more)

### Community 11 - "Community 11"
Cohesion: 0.24
Nodes (16): archive_output_inside_package_returns_tool_error(), build_capsule_smoke_works_through_mcp(), canonical_capsule_fixture(), initialize_accepts_supported_version(), initialize_rejects_unsupported_version(), invalid_tool_arguments_return_mcp_tool_error(), ladybug_query_rejects_mutating_cypher_before_execution(), malformed_json_returns_parse_error() (+8 more)

### Community 12 - "Community 12"
Cohesion: 0.22
Nodes (10): canonical_capsule_fixture(), canonical_capsule_passes_praxis_mvp_eval(), EvalCheck, EvalError, EvalReport, evaluate_praxis_mvp(), pass_helper_marks_check_successful(), retrieval_records_have_source_receipts() (+2 more)

### Community 13 - "Community 13"
Cohesion: 0.18
Nodes (7): CapsuleReport, Small report helpers for local capsule fixture inspection., Human-readable summary extracted from a capsule manifest., Build a compact report from a manifest-like mapping., summarize_manifest(), Local analysis helpers for DIALECTICA fixtures., CapsuleReportTests

### Community 14 - "Community 14"
Cohesion: 0.27
Nodes (4): CapsuleBundle, validate_rights_profile(), validate_temporal_ledger(), ValidationFinding

### Community 15 - "Community 15"
Cohesion: 0.25
Nodes (4): CapsuleManifest, CapsuleOntologyBlueprint, merge_unique(), v3_manifest_builds_macro_type_ontology_blueprint()

### Community 16 - "Community 16"
Cohesion: 0.4
Nodes (3): canonical_to_legacy_capsule_type(), is_canonical_capsule_type(), PraxisCapsuleManifest

## Knowledge Gaps
- **81 isolated node(s):** `BuildDocumentsOptions`, `BuildDocumentsReceipt`, `SourceSummary`, `SourceCapture`, `CollectedFiles` (+76 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **5 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `read_to_string()` connect `Community 3` to `Community 1`, `Community 2`, `Community 5`, `Community 6`, `Community 7`, `Community 8`, `Community 9`, `Community 10`, `Community 17`?**
  _High betweenness centrality (0.222) - this node is a cross-community bridge._
- **Why does `build_documents_capsule()` connect `Community 3` to `Community 1`, `Community 4`, `Community 5`?**
  _High betweenness centrality (0.095) - this node is a cross-community bridge._
- **Why does `read_json()` connect `Community 9` to `Community 0`, `Community 3`, `Community 6`, `Community 7`, `Community 8`, `Community 17`?**
  _High betweenness centrality (0.090) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `main()` (e.g. with `default_fixture_dir()` and `app()`) actually correct?**
  _`main()` has 3 INFERRED edges - model-reasoned connections that need verification._
- **Are the 7 inferred relationships involving `build_documents_capsule()` (e.g. with `read_to_string()` and `compile_from_parts()`) actually correct?**
  _`build_documents_capsule()` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Are the 14 inferred relationships involving `read_to_string()` (e.g. with `build_documents_capsule()` and `ladybug_projection_import_status()`) actually correct?**
  _`read_to_string()` has 14 INFERRED edges - model-reasoned connections that need verification._
- **Are the 4 inferred relationships involving `validate_proposal_set()` (e.g. with `check_proposals()` and `print_build_plan()`) actually correct?**
  _`validate_proposal_set()` has 4 INFERRED edges - model-reasoned connections that need verification._
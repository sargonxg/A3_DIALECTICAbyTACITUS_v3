# Graph Report - A3_DIALECTICAbyTACITUS_v3  (2026-06-08)

## Corpus Check
- 20 files · ~95,487 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 460 nodes · 1003 edges · 22 communities (18 shown, 4 thin omitted)
- Extraction: 92% EXTRACTED · 8% INFERRED · 0% AMBIGUOUS · INFERRED: 77 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `70e53870`
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
- [[_COMMUNITY_Community 18|Community 18]]
- [[_COMMUNITY_Community 21|Community 21]]

## God Nodes (most connected - your core abstractions)
1. `main()` - 29 edges
2. `build_documents_capsule()` - 26 edges
3. `validate_proposal_set()` - 19 edges
4. `write_package()` - 18 edges
5. `read_to_string()` - 16 edges
6. `read_json()` - 15 edges
7. `export_praxis_context_pack()` - 14 edges
8. `load_source_pack()` - 14 edges
9. `validate_reviewer_decision_set()` - 13 edges
10. `promote_records()` - 13 edges

## Surprising Connections (you probably didn't know these)
- `build_capsule_tool()` --calls--> `build_documents_capsule()`  [INFERRED]
  services/dialectica-mcp/src/lib.rs → crates/dialectica-builder/src/lib.rs
- `count_jsonl()` --calls--> `read_to_string()`  [INFERRED]
  services/dialectica-api/src/lib.rs → crates/dialectica-capsule/src/lib.rs
- `archive_capsule_tool()` --calls--> `write_capsule_archive()`  [INFERRED]
  services/dialectica-mcp/src/lib.rs → crates/dialectica-compiler/src/lib.rs
- `context_pack()` --calls--> `export_praxis_context_pack()`  [INFERRED]
  services/dialectica-api/src/lib.rs → crates/dialectica-compiler/src/lib.rs
- `export_praxis_pack_tool()` --calls--> `export_praxis_context_pack()`  [INFERRED]
  services/dialectica-mcp/src/lib.rs → crates/dialectica-compiler/src/lib.rs

## Communities (22 total, 4 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.07
Nodes (49): BuildMode, BuildValidationFinding, BuildValidationReport, BuildValidationSeverity, CapsuleBuildPlan, CapsuleBuildRequest, CapsuleType, ExtractionProposal (+41 more)

### Community 1 - "Community 1"
Cohesion: 0.08
Nodes (40): add_graph_edges(), agent_context_markdown(), ArchiveReceipt, capsule_type_value(), claim_values(), collect_files(), compile_from_parts(), CompileReceipt (+32 more)

### Community 2 - "Community 2"
Cohesion: 0.09
Nodes (36): auto_decision_set(), build_documents_capsule(), build_proposals(), BuildDocumentsOptions, BuildDocumentsReceipt, BuilderError, collect_document_files(), collect_document_files_inner() (+28 more)

### Community 3 - "Community 3"
Cohesion: 0.12
Nodes (36): check_ladybug_projection(), archive_package(), build_documents_from_args(), build_fixture(), check_promotion(), check_proposals(), check_review_decisions(), check_source_pack() (+28 more)

### Community 4 - "Community 4"
Cohesion: 0.05
Nodes (34): AgentGuidance, CapsuleBundleIndex, CapsuleHealthReport, CapsuleInspection, CapsuleLayerIndex, CitationPolicy, FrameMembership, GraphCommunity (+26 more)

### Community 5 - "Community 5"
Cohesion: 0.1
Nodes (23): ApiError, ApiState, app(), build_graph_preview(), context_pack(), ContextPackQuery, count_jsonl(), default_fixture_dir() (+15 more)

### Community 6 - "Community 6"
Cohesion: 0.14
Nodes (30): archive_rejects_output_inside_package_directory(), compile_fixture(), context_pack_contains_praxis_runtime_fields(), fixture_archive_writes_mimetype_first(), fixture_compiler_writes_valid_v3_package(), fresh_temp_dir(), golden_fixture_dir(), load_build_request() (+22 more)

### Community 7 - "Community 7"
Cohesion: 0.13
Nodes (25): archive_capsule_tool(), build_capsule_tool(), call_tool(), default_workflow(), export_praxis_pack_tool(), handle_jsonrpc_line(), handle_jsonrpc_value(), initialize_advertises_dialectica_tools() (+17 more)

### Community 8 - "Community 8"
Cohesion: 0.13
Nodes (23): build_ladybug_projection(), edge_insert_cypher(), edge_type(), GraphProjectionError, json_string(), JsonLdGraph, label(), LadybugProjectionPlan (+15 more)

### Community 9 - "Community 9"
Cohesion: 0.15
Nodes (15): count_jsonl_records(), is_canonical_capsule_type(), PraxisCapsulePackage, read_optional_string(), validate_json_file(), validate_ladybug_build_receipt(), validate_mimetype(), validate_rights_profile() (+7 more)

### Community 10 - "Community 10"
Cohesion: 0.2
Nodes (8): CapsuleBundle, read_jsonl(), temp_package_dir(), v3_package_requires_ladybug_projection_files(), validate_graph_slice(), validate_source_spans(), validate_temporal_ledger(), write_minimal_v3_package()

### Community 11 - "Community 11"
Cohesion: 0.19
Nodes (10): canonical_to_legacy_capsule_type(), capsule_type_blueprint(), CapsuleOntologyBlueprint, default_graph_profile(), merge_unique(), merge_universal_layers(), PraxisCapsuleManifest, semantic_layer() (+2 more)

### Community 12 - "Community 12"
Cohesion: 0.18
Nodes (7): CapsuleReport, Small report helpers for local capsule fixture inspection., Human-readable summary extracted from a capsule manifest., Build a compact report from a manifest-like mapping., summarize_manifest(), Local analysis helpers for DIALECTICA fixtures., CapsuleReportTests

### Community 13 - "Community 13"
Cohesion: 0.24
Nodes (5): CapsuleManifest, is_approved_capsule_type(), ReviewState, validate_language_profile(), validate_manifest()

### Community 14 - "Community 14"
Cohesion: 0.24
Nodes (9): approved_manifest_with_digest_is_export_ready(), assert_digest(), draft_manifest_is_not_export_ready(), export_schema_dir(), unsupported_capsule_type_is_not_export_ready(), v3_manifest_builds_macro_type_ontology_blueprint(), validate_ladybug_projection_files(), validate_ladybug_projection_manifest() (+1 more)

## Knowledge Gaps
- **78 isolated node(s):** `BuildDocumentsOptions`, `BuildDocumentsReceipt`, `SourceSummary`, `CollectedFiles`, `ValidationSeverity` (+73 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **4 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `read_to_string()` connect `Community 1` to `Community 2`, `Community 3`, `Community 4`, `Community 5`, `Community 6`, `Community 8`, `Community 9`, `Community 10`, `Community 14`?**
  _High betweenness centrality (0.173) - this node is a cross-community bridge._
- **Why does `read_json()` connect `Community 5` to `Community 0`, `Community 1`, `Community 4`, `Community 6`, `Community 9`, `Community 10`, `Community 14`?**
  _High betweenness centrality (0.117) - this node is a cross-community bridge._
- **Why does `build_documents_capsule()` connect `Community 2` to `Community 1`, `Community 3`, `Community 7`?**
  _High betweenness centrality (0.102) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `main()` (e.g. with `default_fixture_dir()` and `app()`) actually correct?**
  _`main()` has 3 INFERRED edges - model-reasoned connections that need verification._
- **Are the 6 inferred relationships involving `build_documents_capsule()` (e.g. with `read_to_string()` and `compile_from_parts()`) actually correct?**
  _`build_documents_capsule()` has 6 INFERRED edges - model-reasoned connections that need verification._
- **Are the 4 inferred relationships involving `validate_proposal_set()` (e.g. with `check_proposals()` and `print_build_plan()`) actually correct?**
  _`validate_proposal_set()` has 4 INFERRED edges - model-reasoned connections that need verification._
- **Are the 10 inferred relationships involving `read_to_string()` (e.g. with `build_documents_capsule()` and `local_documents_compile_to_portable_capsule_and_praxis_pack()`) actually correct?**
  _`read_to_string()` has 10 INFERRED edges - model-reasoned connections that need verification._
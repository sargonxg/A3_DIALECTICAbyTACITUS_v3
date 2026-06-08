# Graph Report - A3_DIALECTICAbyTACITUS_v3  (2026-06-08)

## Corpus Check
- 17 files · ~86,665 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 382 nodes · 829 edges · 22 communities (17 shown, 5 thin omitted)
- Extraction: 93% EXTRACTED · 7% INFERRED · 0% AMBIGUOUS · INFERRED: 60 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `7fa7ae81`
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
- [[_COMMUNITY_Community 17|Community 17]]
- [[_COMMUNITY_Community 18|Community 18]]
- [[_COMMUNITY_Community 21|Community 21]]

## God Nodes (most connected - your core abstractions)
1. `main()` - 23 edges
2. `validate_proposal_set()` - 19 edges
3. `write_package()` - 18 edges
4. `read_json()` - 15 edges
5. `read_to_string()` - 14 edges
6. `load_source_pack()` - 14 edges
7. `validate_reviewer_decision_set()` - 13 edges
8. `promote_records()` - 13 edges
9. `load_build_request()` - 12 edges
10. `build_ladybug_projection()` - 12 edges

## Surprising Connections (you probably didn't know these)
- `count_jsonl()` --calls--> `read_to_string()`  [INFERRED]
  services/dialectica-api/src/lib.rs → crates/dialectica-capsule/src/lib.rs
- `context_pack()` --calls--> `export_praxis_context_pack()`  [INFERRED]
  services/dialectica-api/src/lib.rs → crates/dialectica-compiler/src/lib.rs
- `golden_source_pack_loads_and_validates()` --calls--> `validate_source_pack()`  [INFERRED]
  tests/dialectica-contract-tests/tests/lane_a_bundle.rs → crates/dialectica-extractor/src/lib.rs
- `golden_proposal_set_routes_plus_review_gates()` --calls--> `validate_proposal_set()`  [INFERRED]
  tests/dialectica-contract-tests/tests/lane_a_bundle.rs → crates/dialectica-extractor/src/lib.rs
- `proposal_without_source_span_fails_validation()` --calls--> `validate_proposal_set()`  [INFERRED]
  tests/dialectica-contract-tests/tests/lane_a_bundle.rs → crates/dialectica-extractor/src/lib.rs

## Communities (22 total, 5 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.06
Nodes (59): BuildMode, BuildValidationFinding, BuildValidationReport, BuildValidationSeverity, CapsuleBuildPlan, CapsuleBuildRequest, CapsuleType, ExtractionProposal (+51 more)

### Community 1 - "Community 1"
Cohesion: 0.07
Nodes (41): add_graph_edges(), agent_context_markdown(), ArchiveReceipt, capsule_type_value(), claim_values(), collect_files(), compile_from_parts(), CompileReceipt (+33 more)

### Community 2 - "Community 2"
Cohesion: 0.05
Nodes (40): AgentGuidance, capsule_type_blueprint(), CapsuleBundleIndex, CapsuleHealthReport, CapsuleInspection, CapsuleLayerIndex, CitationPolicy, default_graph_profile() (+32 more)

### Community 3 - "Community 3"
Cohesion: 0.14
Nodes (30): archive_rejects_output_inside_package_directory(), compile_fixture(), context_pack_contains_praxis_runtime_fields(), fixture_archive_writes_mimetype_first(), fixture_compiler_writes_valid_v3_package(), fresh_temp_dir(), golden_fixture_dir(), load_build_request() (+22 more)

### Community 4 - "Community 4"
Cohesion: 0.12
Nodes (21): ApiError, ApiState, app(), build_graph_preview(), context_pack(), ContextPackQuery, count_jsonl(), default_fixture_dir() (+13 more)

### Community 5 - "Community 5"
Cohesion: 0.12
Nodes (23): build_ladybug_projection(), edge_insert_cypher(), edge_type(), GraphProjectionError, json_string(), JsonLdGraph, label(), LadybugProjectionPlan (+15 more)

### Community 6 - "Community 6"
Cohesion: 0.18
Nodes (15): is_approved_capsule_type(), read_optional_string(), ReviewState, validate_json_file(), validate_language_profile(), validate_manifest(), validate_mimetype(), validate_rights_profile() (+7 more)

### Community 7 - "Community 7"
Cohesion: 0.19
Nodes (19): check_ladybug_projection(), query_ladybug_projection(), build_fixture(), inspect_bundle(), is_v3_package(), ladybug_check(), ladybug_query(), load_legacy_or_exit() (+11 more)

### Community 8 - "Community 8"
Cohesion: 0.15
Nodes (9): count_jsonl_records(), PraxisCapsulePackage, ProposalSet, read_json(), read_jsonl(), ReviewerDecisionSet, temp_package_dir(), v3_package_requires_ladybug_projection_files() (+1 more)

### Community 9 - "Community 9"
Cohesion: 0.18
Nodes (7): CapsuleReport, Small report helpers for local capsule fixture inspection., Human-readable summary extracted from a capsule manifest., Build a compact report from a manifest-like mapping., summarize_manifest(), Local analysis helpers for DIALECTICA fixtures., CapsuleReportTests

### Community 10 - "Community 10"
Cohesion: 0.25
Nodes (10): approved_manifest_with_digest_is_export_ready(), assert_digest(), digest_file(), draft_manifest_is_not_export_ready(), export_schema_dir(), unsupported_capsule_type_is_not_export_ready(), validate_ladybug_build_receipt(), validate_ladybug_projection_files() (+2 more)

### Community 11 - "Community 11"
Cohesion: 0.25
Nodes (4): CapsuleManifest, CapsuleOntologyBlueprint, merge_unique(), v3_manifest_builds_macro_type_ontology_blueprint()

### Community 12 - "Community 12"
Cohesion: 0.39
Nodes (3): CapsuleBundle, validate_graph_slice(), validate_temporal_ledger()

### Community 13 - "Community 13"
Cohesion: 0.4
Nodes (3): canonical_to_legacy_capsule_type(), is_canonical_capsule_type(), PraxisCapsuleManifest

## Knowledge Gaps
- **74 isolated node(s):** `ValidationSeverity`, `CapsuleBundleIndex`, `CapsuleLayerIndex`, `SourceLedgerRecord`, `TemporalLedgerRecord` (+69 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **5 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `read_json()` connect `Community 8` to `Community 0`, `Community 1`, `Community 2`, `Community 3`, `Community 4`, `Community 6`, `Community 10`?**
  _High betweenness centrality (0.156) - this node is a cross-community bridge._
- **Why does `read_to_string()` connect `Community 1` to `Community 2`, `Community 3`, `Community 4`, `Community 5`, `Community 6`, `Community 7`, `Community 8`, `Community 10`?**
  _High betweenness centrality (0.104) - this node is a cross-community bridge._
- **Why does `load_source_pack()` connect `Community 3` to `Community 0`, `Community 8`, `Community 1`?**
  _High betweenness centrality (0.087) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `main()` (e.g. with `default_fixture_dir()` and `app()`) actually correct?**
  _`main()` has 2 INFERRED edges - model-reasoned connections that need verification._
- **Are the 4 inferred relationships involving `validate_proposal_set()` (e.g. with `check_proposals()` and `print_build_plan()`) actually correct?**
  _`validate_proposal_set()` has 4 INFERRED edges - model-reasoned connections that need verification._
- **Are the 8 inferred relationships involving `read_to_string()` (e.g. with `is_v3_package()` and `export_praxis_context_pack()`) actually correct?**
  _`read_to_string()` has 8 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ValidationSeverity`, `CapsuleBundleIndex`, `CapsuleLayerIndex` to the rest of the system?**
  _74 weakly-connected nodes found - possible documentation gaps or missing edges._
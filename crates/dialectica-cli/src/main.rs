//! Local DIALECTICA command-line tooling.

use std::path::Path;

use dialectica_capsule::{
    export_schema_dir, CapsuleBundle, CapsuleInspection, CapsuleManifest, PraxisCapsulePackage,
    ReviewState, CAPSULE_SPEC_VERSION,
};

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "help".to_owned());

    match command.as_str() {
        "welcome" => print_welcome(),
        "doctor" => print_doctor(),
        "build-docs" => {
            let remaining = args.collect::<Vec<_>>();
            build_documents_from_args(&remaining);
        }
        "validate" => {
            let Some(path) = args.next() else {
                eprintln!("missing bundle directory");
                std::process::exit(2);
            };
            validate_bundle(Path::new(&path));
        }
        "inspect" => {
            let Some(path) = args.next() else {
                eprintln!("missing bundle directory");
                std::process::exit(2);
            };
            inspect_bundle(Path::new(&path));
        }
        "ontology-plan" => {
            let Some(path) = args.next() else {
                eprintln!("missing bundle directory");
                std::process::exit(2);
            };
            print_ontology_plan(Path::new(&path));
        }
        "source-pack-check" => {
            let Some(path) = args.next() else {
                eprintln!("missing source pack path");
                std::process::exit(2);
            };
            check_source_pack(Path::new(&path));
        }
        "proposal-check" => {
            let Some(build_request_path) = args.next() else {
                eprintln!("missing build request path");
                std::process::exit(2);
            };
            let Some(source_pack_path) = args.next() else {
                eprintln!("missing source pack path");
                std::process::exit(2);
            };
            let Some(proposal_dir) = args.next() else {
                eprintln!("missing proposal directory");
                std::process::exit(2);
            };
            check_proposals(
                Path::new(&build_request_path),
                Path::new(&source_pack_path),
                Path::new(&proposal_dir),
            );
        }
        "build-plan" => {
            let Some(build_request_path) = args.next() else {
                eprintln!("missing build request path");
                std::process::exit(2);
            };
            let Some(source_pack_path) = args.next() else {
                eprintln!("missing source pack path");
                std::process::exit(2);
            };
            let Some(proposal_dir) = args.next() else {
                eprintln!("missing proposal directory");
                std::process::exit(2);
            };
            print_build_plan(
                Path::new(&build_request_path),
                Path::new(&source_pack_path),
                Path::new(&proposal_dir),
            );
        }
        "review-check" => {
            let Some(build_request_path) = args.next() else {
                eprintln!("missing build request path");
                std::process::exit(2);
            };
            let Some(source_pack_path) = args.next() else {
                eprintln!("missing source pack path");
                std::process::exit(2);
            };
            let Some(proposal_dir) = args.next() else {
                eprintln!("missing proposal directory");
                std::process::exit(2);
            };
            let Some(review_dir) = args.next() else {
                eprintln!("missing review decision directory");
                std::process::exit(2);
            };
            check_review_decisions(
                Path::new(&build_request_path),
                Path::new(&source_pack_path),
                Path::new(&proposal_dir),
                Path::new(&review_dir),
            );
        }
        "promote-check" => {
            let Some(build_request_path) = args.next() else {
                eprintln!("missing build request path");
                std::process::exit(2);
            };
            let Some(source_pack_path) = args.next() else {
                eprintln!("missing source pack path");
                std::process::exit(2);
            };
            let Some(proposal_dir) = args.next() else {
                eprintln!("missing proposal directory");
                std::process::exit(2);
            };
            let Some(review_dir) = args.next() else {
                eprintln!("missing review decision directory");
                std::process::exit(2);
            };
            check_promotion(
                Path::new(&build_request_path),
                Path::new(&source_pack_path),
                Path::new(&proposal_dir),
                Path::new(&review_dir),
            );
        }
        "build-fixture" => {
            let Some(fixture_dir) = args.next() else {
                eprintln!("missing fixture directory");
                std::process::exit(2);
            };
            let Some(output_dir) = parse_option_value(args.next(), args.next(), "--out") else {
                eprintln!("missing --out <directory>");
                std::process::exit(2);
            };
            build_fixture(Path::new(&fixture_dir), Path::new(&output_dir));
        }
        "archive" => {
            let Some(package_dir) = args.next() else {
                eprintln!("missing compiled package directory");
                std::process::exit(2);
            };
            let Some(output_file) = parse_option_value(args.next(), args.next(), "--out") else {
                eprintln!("missing --out <file.capsule>");
                std::process::exit(2);
            };
            archive_package(Path::new(&package_dir), Path::new(&output_file));
        }
        "context-pack" => {
            let Some(package_dir) = args.next() else {
                eprintln!("missing compiled package directory");
                std::process::exit(2);
            };
            let workflow = parse_option_value(args.next(), args.next(), "--workflow")
                .unwrap_or_else(|| "decision_brief".to_owned());
            print_context_pack(Path::new(&package_dir), &workflow);
        }
        "eval" => {
            let Some(package_dir) = args.next() else {
                eprintln!("missing compiled package directory");
                std::process::exit(2);
            };
            let workflow = parse_option_value(args.next(), args.next(), "--workflow")
                .unwrap_or_else(|| "decision_brief".to_owned());
            print_eval_report(Path::new(&package_dir), &workflow);
        }
        "praxis-pack" => {
            let remaining = args.collect::<Vec<_>>();
            write_context_pack_from_args(&remaining);
        }
        "mcp-config" => print_mcp_config(),
        "ladybug-plan" => {
            let Some(path) = args.next() else {
                eprintln!("missing capsule directory");
                std::process::exit(2);
            };
            ladybug_plan(Path::new(&path));
        }
        "ladybug-build" => {
            let Some(path) = args.next() else {
                eprintln!("missing capsule directory");
                std::process::exit(2);
            };
            ladybug_build(Path::new(&path));
        }
        "ladybug-check" => {
            let Some(path) = args.next() else {
                eprintln!("missing capsule directory");
                std::process::exit(2);
            };
            ladybug_check(Path::new(&path));
        }
        "ladybug-query" => {
            let Some(path) = args.next() else {
                eprintln!("missing capsule directory");
                std::process::exit(2);
            };
            let Some(query) = args.next() else {
                eprintln!("missing Cypher query");
                std::process::exit(2);
            };
            ladybug_query(Path::new(&path), &query);
        }
        "schema-export" => {
            let Some(path) = args.next() else {
                eprintln!("missing schema output directory");
                std::process::exit(2);
            };
            if let Err(error) = export_schema_dir(Path::new(&path)) {
                eprintln!("{error}");
                std::process::exit(1);
            }
            if let Err(error) = dialectica_extractor::export_schema_dir(Path::new(&path)) {
                eprintln!("{error}");
                std::process::exit(1);
            }
            println!("schema_exported={path}");
        }
        "help" | "--help" | "-h" => print_help(),
        other => {
            eprintln!("unknown command: {other}");
            print_help();
            std::process::exit(2);
        }
    }
}

fn print_help() {
    println!("dialectica-cli");
    println!("commands:");
    println!("  welcome                 print the DIALECTICA operator welcome");
    println!("  doctor                  print scaffold health");
    println!("  build-docs --type <user|situation|tool|output> --input <dir> --out <dir> [--title <title>] [--workflow <workflow>] [--mode <assisted|auto-draft|plus-promoted>]");
    println!("  validate <bundle-dir>   validate a capsule bundle directory");
    println!("  inspect <bundle-dir>    print capsule bundle summary");
    println!("  ontology-plan <dir>     print capsule-specific ontology blueprint");
    println!("  source-pack-check <path> validate a builder source pack");
    println!("  proposal-check <request> <source-pack> <proposal-dir>");
    println!("  build-plan <request> <source-pack> <proposal-dir>");
    println!("  review-check <request> <source-pack> <proposal-dir> <review-dir>");
    println!("  promote-check <request> <source-pack> <proposal-dir> <review-dir>");
    println!("  build-fixture <fixture-dir> --out <dir>");
    println!("  archive <compiled-dir> --out <file.capsule>");
    println!("  context-pack <compiled-dir> [--workflow <workflow>]");
    println!("  eval <compiled-dir> [--workflow <workflow>]");
    println!("  praxis-pack <compiled-dir> --out <file.json> [--workflow <workflow>]");
    println!("  mcp-config              print a Codex MCP config snippet");
    println!("  ladybug-plan <dir>      print embedded Ladybug projection plan");
    println!("  ladybug-build <dir>     build graph/ladybug/capsule.lbug from graph.jsonld");
    println!("  ladybug-check <dir>     validate embedded Ladybug projection files");
    println!("  ladybug-query <dir> <q> run a read-only Cypher query against capsule.lbug");
    println!("  schema-export <dir>     export JSON Schema snapshots");
}

fn print_welcome() {
    println!("DIALECTICA by TACITUS");
    println!(
        "Build your PRAXIS context capsule from documents, notes, sources, and review decisions."
    );
    println!("Capsule classes: user, situation, tool, output.");
    println!("Local build loop:");
    println!("  dialectica build-docs --type situation --input ./docs --out ./local-capsules/conflict --workflow conflict_map");
    println!("Outputs:");
    println!("  package/                  canonical v3 capsule package");
    println!("  *.capsule                 portable capsule archive");
    println!("  praxis-context-pack.json  PRAXIS agent context handoff");
    println!("  praxis-import.json        local/cloud bridge receipt");
}

fn print_doctor() {
    let manifest = CapsuleManifest::new(
        "cap_scaffold",
        "Scaffold capsule",
        "situation_capsule",
        ReviewState::Approved,
        "sha256:scaffold",
    );

    println!("dialectica scaffold doctor");
    println!("capsule_spec_version={CAPSULE_SPEC_VERSION}");
    println!("legacy_schema_version={}", manifest.schema_version);
    println!("compiler={}", dialectica_compiler::COMPILER_ID);
    println!("export_ready={}", manifest.is_export_ready());
}

fn build_documents_from_args(args: &[String]) {
    let Some(capsule_type_text) = option_value(args, "--type") else {
        eprintln!("missing --type <user|situation|tool|output>");
        std::process::exit(2);
    };
    let Some(input_dir) = option_value(args, "--input") else {
        eprintln!("missing --input <directory>");
        std::process::exit(2);
    };
    let Some(output_dir) = option_value(args, "--out") else {
        eprintln!("missing --out <directory>");
        std::process::exit(2);
    };
    let capsule_type = match dialectica_builder::parse_capsule_type(&capsule_type_text) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    };
    let mode = option_value(args, "--mode").unwrap_or_else(|| "assisted".to_owned());
    let build_mode = match dialectica_builder::parse_build_mode(&mode) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    };
    let workflow = option_value(args, "--workflow").unwrap_or_else(|| match capsule_type {
        dialectica_extractor::CapsuleType::User => "user_context".to_owned(),
        dialectica_extractor::CapsuleType::Situation => "decision_brief".to_owned(),
        dialectica_extractor::CapsuleType::Tool => "method_application".to_owned(),
        dialectica_extractor::CapsuleType::Output => "output_review".to_owned(),
    });

    match dialectica_builder::build_documents_capsule(&dialectica_builder::BuildDocumentsOptions {
        input_dir: Path::new(&input_dir).to_path_buf(),
        output_dir: Path::new(&output_dir).to_path_buf(),
        capsule_type,
        build_mode,
        title: option_value(args, "--title"),
        workflow,
    }) {
        Ok(receipt) => {
            println!("capsule_id={}", receipt.capsule_id);
            println!("capsule_type={}", receipt.capsule_type);
            println!("title={}", receipt.title);
            println!("workflow={}", receipt.workflow);
            println!("package_dir={}", receipt.package_dir.display());
            println!("archive_path={}", receipt.archive_path.display());
            println!("context_pack_path={}", receipt.context_pack_path.display());
            println!(
                "praxis_import_path={}",
                receipt.praxis_import_path.display()
            );
            println!("source_pack_path={}", receipt.source_pack_path.display());
            println!("proposal_dir={}", receipt.proposal_dir.display());
            println!("review_queue_path={}", receipt.review_queue_path.display());
            println!(
                "review_decision_path={}",
                receipt.review_decision_path.display()
            );
            println!(
                "promotion_summary_path={}",
                receipt.promotion_summary_path.display()
            );
            println!("source_document_count={}", receipt.source_document_count);
            println!("source_span_count={}", receipt.source_span_count);
            println!("skipped_file_count={}", receipt.skipped_file_count);
            println!("proposal_count={}", receipt.proposal_count);
            println!("decision_count={}", receipt.decision_count);
            println!("caveated_record_count={}", receipt.caveated_record_count);
            println!("bundle_digest={}", receipt.bundle_digest);
            println!("archive_digest={}", receipt.archive_digest);
            println!("valid=true");
        }
        Err(error) => {
            eprintln!("{error}");
            println!("valid=false");
            std::process::exit(1);
        }
    }
}

fn write_context_pack_from_args(args: &[String]) {
    let Some(package_dir) = args.first() else {
        eprintln!("missing compiled package directory");
        std::process::exit(2);
    };
    let Some(output_file) = option_value(args, "--out") else {
        eprintln!("missing --out <file.json>");
        std::process::exit(2);
    };
    let workflow = option_value(args, "--workflow").unwrap_or_else(|| "decision_brief".to_owned());

    match dialectica_compiler::export_praxis_context_pack(Path::new(package_dir), &workflow) {
        Ok(pack) => {
            let text = match serde_json::to_string_pretty(&pack) {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("failed to serialize context pack: {error}");
                    std::process::exit(1);
                }
            };
            let path = Path::new(&output_file);
            if let Some(parent) = path.parent() {
                if let Err(error) = std::fs::create_dir_all(parent) {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            }
            if let Err(error) = std::fs::write(path, format!("{text}\n")) {
                eprintln!("{error}");
                std::process::exit(1);
            }
            println!("context_pack_path={}", path.display());
            println!("capsule_id={}", pack.capsule_id);
            println!("workflow={}", pack.workflow);
            println!("valid=true");
        }
        Err(error) => {
            eprintln!("{error}");
            println!("valid=false");
            std::process::exit(1);
        }
    }
}

fn print_mcp_config() {
    let cwd = std::env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| ".".to_owned())
        .replace('\\', "\\\\");
    println!("[mcp_servers.dialectica]");
    println!("command = \"cargo\"");
    println!("args = [\"run\", \"-p\", \"dialectica-mcp\", \"--\"]");
    println!("cwd = \"{cwd}\"");
}

fn validate_bundle(path: &Path) {
    let report = if is_v3_package(path) {
        match PraxisCapsulePackage::load_from_dir(path) {
            Ok(package) => package.validate(),
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
    } else {
        load_legacy_or_exit(path).validate()
    };

    for finding in &report.findings {
        println!(
            "{:?} {} {}: {}",
            finding.severity, finding.code, finding.path, finding.message
        );
    }

    println!("valid={}", !report.has_errors());
    if report.has_errors() {
        std::process::exit(1);
    }
}

fn inspect_bundle(path: &Path) {
    if is_v3_package(path) {
        let inspection = match PraxisCapsulePackage::load_from_dir(path) {
            Ok(package) => package.inspection(),
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        };

        print_v3_inspection(path, &inspection);
        print_ladybug_projection_inspection(path);
        return;
    }

    let inspection = load_legacy_or_exit(path).inspection();
    print_inspection(&inspection);
}

fn print_inspection(inspection: &CapsuleInspection) {
    println!("capsule_id={}", inspection.capsule_id);
    println!("capsule_type={}", inspection.capsule_type);
    println!("review_state={:?}", inspection.review_state);
    println!("source_count={}", inspection.source_count);
    println!("claim_count={}", inspection.claim_count);
    println!("graph_node_count={}", inspection.graph_node_count);
    println!("graph_edge_count={}", inspection.graph_edge_count);
    println!("warning_count={}", inspection.warning_count);
}

fn print_v3_inspection(path: &Path, inspection: &CapsuleInspection) {
    let graph_counts = dialectica_graph::plan_ladybug_projection(path).ok();

    println!("capsule_id={}", inspection.capsule_id);
    println!("capsule_type={}", inspection.capsule_type);
    println!("review_state={:?}", inspection.review_state);
    println!("source_count={}", inspection.source_count);
    println!("claim_count={}", inspection.claim_count);
    println!(
        "graph_node_count={}",
        graph_counts
            .as_ref()
            .map(|plan| plan.node_count)
            .unwrap_or(inspection.graph_node_count)
    );
    println!(
        "graph_edge_count={}",
        graph_counts
            .as_ref()
            .map(|plan| plan.edge_count)
            .unwrap_or(inspection.graph_edge_count)
    );
    println!("warning_count={}", inspection.warning_count);
}

fn print_ontology_plan(path: &Path) {
    let blueprint = if is_v3_package(path) {
        match PraxisCapsulePackage::load_from_dir(path) {
            Ok(package) => package.manifest.ontology_blueprint(),
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
    } else {
        load_legacy_or_exit(path).ontology_blueprint()
    };

    match serde_json::to_string_pretty(&blueprint) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("failed to serialize ontology blueprint: {error}");
            std::process::exit(1);
        }
    }
}

fn check_source_pack(path: &Path) {
    let source_pack = load_source_pack_or_exit(path);
    let report = dialectica_extractor::validate_source_pack(&source_pack);
    print_build_validation_report(&report);
    println!("source_pack_id={}", source_pack.pack_id);
    println!("source_document_count={}", source_pack.documents.len());
    println!("source_span_count={}", source_pack.spans.len());
    println!("valid={}", !report.has_errors());
    if report.has_errors() {
        std::process::exit(1);
    }
}

fn check_proposals(build_request_path: &Path, source_pack_path: &Path, proposal_dir: &Path) {
    let build_request = load_build_request_or_exit(build_request_path);
    let source_pack = load_source_pack_or_exit(source_pack_path);
    let proposal_set = load_proposal_set_or_exit(proposal_dir);
    let report =
        dialectica_extractor::validate_proposal_set(&source_pack, &build_request, &proposal_set);
    let gates = dialectica_extractor::route_review_gates(&build_request, &proposal_set);

    print_build_validation_report(&report);
    println!("extraction_run_id={}", proposal_set.extraction_run.run_id);
    println!("proposal_count={}", proposal_set.proposals.len());
    println!("review_gate_count={}", gates.len());
    println!(
        "blocking_gate_count={}",
        gates.iter().filter(|gate| gate.blocking).count()
    );
    println!("valid={}", !report.has_errors());
    if report.has_errors() {
        std::process::exit(1);
    }
}

fn print_build_plan(build_request_path: &Path, source_pack_path: &Path, proposal_dir: &Path) {
    let build_request = load_build_request_or_exit(build_request_path);
    let source_pack = load_source_pack_or_exit(source_pack_path);
    let proposal_set = load_proposal_set_or_exit(proposal_dir);
    let report =
        dialectica_extractor::validate_proposal_set(&source_pack, &build_request, &proposal_set);
    if report.has_errors() {
        print_build_validation_report(&report);
        std::process::exit(1);
    }

    let plan =
        dialectica_extractor::plan_capsule_build(&build_request, &source_pack, &proposal_set);
    match serde_json::to_string_pretty(&plan) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("failed to serialize build plan: {error}");
            std::process::exit(1);
        }
    }
}

fn check_review_decisions(
    build_request_path: &Path,
    source_pack_path: &Path,
    proposal_dir: &Path,
    review_dir: &Path,
) {
    let build_request = load_build_request_or_exit(build_request_path);
    let source_pack = load_source_pack_or_exit(source_pack_path);
    let proposal_set = load_proposal_set_or_exit(proposal_dir);
    let decision_set = load_reviewer_decision_set_or_exit(review_dir);
    let report = dialectica_extractor::validate_reviewer_decision_set(
        &source_pack,
        &build_request,
        &proposal_set,
        &decision_set,
    );
    let required_decision_count =
        dialectica_extractor::route_review_gates(&build_request, &proposal_set).len();

    print_build_validation_report(&report);
    println!("decision_set_id={}", decision_set.decision_set_id);
    println!("decision_count={}", decision_set.decisions.len());
    println!("required_decision_count={required_decision_count}");
    println!("valid={}", !report.has_errors());
    if report.has_errors() {
        std::process::exit(1);
    }
}

fn check_promotion(
    build_request_path: &Path,
    source_pack_path: &Path,
    proposal_dir: &Path,
    review_dir: &Path,
) {
    let build_request = load_build_request_or_exit(build_request_path);
    let source_pack = load_source_pack_or_exit(source_pack_path);
    let proposal_set = load_proposal_set_or_exit(proposal_dir);
    let decision_set = load_reviewer_decision_set_or_exit(review_dir);
    match dialectica_extractor::promote_records(
        &build_request,
        &source_pack,
        &proposal_set,
        &decision_set,
    ) {
        Ok(promoted) => {
            println!("ready_for_compiler={}", promoted.ready_for_compiler);
            println!("promoted_record_count={}", promoted.promoted_records.len());
            println!(
                "required_decision_count={}",
                promoted.required_decision_count
            );
            println!("decision_count={}", promoted.decision_count);
            println!("caveated_record_count={}", promoted.caveated_record_count);
            println!(
                "rejected_proposal_count={}",
                promoted.rejected_proposal_ids.len()
            );
            println!(
                "evidence_requested_proposal_count={}",
                promoted.evidence_requested_proposal_ids.len()
            );
            println!("valid={}", promoted.ready_for_compiler);
            if !promoted.ready_for_compiler {
                std::process::exit(1);
            }
        }
        Err(report) => {
            print_build_validation_report(&report);
            println!("valid=false");
            std::process::exit(1);
        }
    }
}

fn build_fixture(fixture_dir: &Path, output_dir: &Path) {
    match dialectica_compiler::compile_fixture(fixture_dir, output_dir) {
        Ok(receipt) => {
            println!("capsule_id={}", receipt.capsule_id);
            println!("output_dir={}", receipt.output_dir.display());
            println!("promoted_record_count={}", receipt.promoted_record_count);
            println!("rejected_record_count={}", receipt.rejected_record_count);
            println!("caveated_record_count={}", receipt.caveated_record_count);
            println!("package_file_count={}", receipt.package_file_count);
            println!("bundle_digest={}", receipt.bundle_digest);
            println!("valid=true");
        }
        Err(error) => {
            eprintln!("{error}");
            println!("valid=false");
            std::process::exit(1);
        }
    }
}

fn archive_package(package_dir: &Path, output_file: &Path) {
    match dialectica_compiler::write_capsule_archive(package_dir, output_file) {
        Ok(receipt) => {
            println!("capsule_id={}", receipt.capsule_id);
            println!("archive_path={}", receipt.archive_path.display());
            println!("entry_count={}", receipt.entry_count);
            println!("archive_digest={}", receipt.archive_digest);
            println!(
                "first_entry={}",
                receipt.entries.first().map(String::as_str).unwrap_or("")
            );
            println!("valid=true");
        }
        Err(error) => {
            eprintln!("{error}");
            println!("valid=false");
            std::process::exit(1);
        }
    }
}

fn print_context_pack(package_dir: &Path, workflow: &str) {
    match dialectica_compiler::export_praxis_context_pack(package_dir, workflow) {
        Ok(pack) => match serde_json::to_string_pretty(&pack) {
            Ok(text) => println!("{text}"),
            Err(error) => {
                eprintln!("failed to serialize context pack: {error}");
                std::process::exit(1);
            }
        },
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

fn print_eval_report(package_dir: &Path, workflow: &str) {
    match dialectica_eval::evaluate_praxis_mvp(package_dir, workflow) {
        Ok(report) => {
            let passed = report.passed;
            match serde_json::to_string_pretty(&report) {
                Ok(text) => println!("{text}"),
                Err(error) => {
                    eprintln!("failed to serialize eval report: {error}");
                    std::process::exit(1);
                }
            }
            if !passed {
                std::process::exit(1);
            }
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

fn print_build_validation_report(report: &dialectica_extractor::BuildValidationReport) {
    for finding in &report.findings {
        println!(
            "{:?} {} {}: {}",
            finding.severity, finding.code, finding.path, finding.message
        );
    }
}

fn ladybug_plan(path: &Path) {
    match dialectica_graph::plan_ladybug_projection(path) {
        Ok(plan) => match serde_json::to_string_pretty(&plan) {
            Ok(json) => println!("{json}"),
            Err(error) => {
                eprintln!("failed to serialize Ladybug projection plan: {error}");
                std::process::exit(1);
            }
        },
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

fn ladybug_build(path: &Path) {
    match dialectica_graph::build_ladybug_projection(path) {
        Ok(manifest) => {
            println!("ladybug_projection_built={}", manifest.database_path);
            println!("capsule_id={}", manifest.capsule_id);
            println!("node_count={}", manifest.node_count);
            println!("edge_count={}", manifest.edge_count);
            println!("projection_digest={}", manifest.projection_digest);
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

fn ladybug_check(path: &Path) {
    match dialectica_graph::check_ladybug_projection(path) {
        Ok(manifest) => {
            print_ladybug_projection_summary(&manifest);
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

fn print_ladybug_projection_inspection(path: &Path) {
    match dialectica_graph::check_ladybug_projection(path) {
        Ok(manifest) => print_ladybug_projection_summary(&manifest),
        Err(error) => {
            println!("ladybug_projection_valid=false");
            println!("ladybug_projection_error={error}");
        }
    }
}

fn print_ladybug_projection_summary(manifest: &dialectica_capsule::LadybugProjectionManifest) {
    println!("ladybug_projection_valid=true");
    println!("ladybug_capsule_id={}", manifest.capsule_id);
    println!("ladybug_node_count={}", manifest.node_count);
    println!("ladybug_edge_count={}", manifest.edge_count);
    println!("ladybug_database_path={}", manifest.database_path);
}

fn ladybug_query(path: &Path, query: &str) {
    match dialectica_graph::query_ladybug_projection(path, query) {
        Ok(output) => {
            println!("{}", output.columns.join("\t"));
            for row in output.rows {
                println!("{}", row.join("\t"));
            }
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

fn load_legacy_or_exit(path: &Path) -> CapsuleBundle {
    match CapsuleBundle::load_from_dir(path) {
        Ok(bundle) => bundle,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

fn load_source_pack_or_exit(path: &Path) -> dialectica_extractor::SourcePack {
    match dialectica_extractor::load_source_pack(path) {
        Ok(source_pack) => source_pack,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

fn load_build_request_or_exit(path: &Path) -> dialectica_extractor::CapsuleBuildRequest {
    match dialectica_extractor::load_build_request(path) {
        Ok(build_request) => build_request,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

fn load_proposal_set_or_exit(path: &Path) -> dialectica_extractor::ProposalSet {
    match dialectica_extractor::ProposalSet::load_from_dir(path) {
        Ok(proposal_set) => proposal_set,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

fn load_reviewer_decision_set_or_exit(path: &Path) -> dialectica_extractor::ReviewerDecisionSet {
    match dialectica_extractor::ReviewerDecisionSet::load_from_dir(path) {
        Ok(decision_set) => decision_set,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

fn parse_option_value(
    first: Option<String>,
    second: Option<String>,
    option_name: &str,
) -> Option<String> {
    match (first, second) {
        (Some(flag), Some(value)) if flag == option_name => Some(value),
        (Some(value), _) if value != option_name => Some(value),
        _ => None,
    }
}

fn option_value(args: &[String], option_name: &str) -> Option<String> {
    args.windows(2).find_map(|pair| {
        if pair[0] == option_name {
            Some(pair[1].clone())
        } else {
            None
        }
    })
}

fn is_v3_package(path: &Path) -> bool {
    let manifest_path = path.join("manifest.json");
    let Ok(text) = std::fs::read_to_string(manifest_path) else {
        return false;
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else {
        return false;
    };

    value.get("spec_version").is_some() || value.get("type").is_some()
}

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
        "doctor" => print_doctor(),
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
    println!("  doctor                  print scaffold health");
    println!("  validate <bundle-dir>   validate a capsule bundle directory");
    println!("  inspect <bundle-dir>    print capsule bundle summary");
    println!("  ontology-plan <dir>     print capsule-specific ontology blueprint");
    println!("  ladybug-plan <dir>      print embedded Ladybug projection plan");
    println!("  ladybug-build <dir>     build graph/ladybug/capsule.lbug from graph.jsonld");
    println!("  ladybug-check <dir>     validate embedded Ladybug projection files");
    println!("  ladybug-query <dir> <q> run a read-only Cypher query against capsule.lbug");
    println!("  schema-export <dir>     export JSON Schema snapshots");
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
    let bundle = load_legacy_or_exit(path);
    let blueprint = bundle.ontology_blueprint();
    match serde_json::to_string_pretty(&blueprint) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("failed to serialize ontology blueprint: {error}");
            std::process::exit(1);
        }
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

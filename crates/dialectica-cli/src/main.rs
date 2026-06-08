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
    let inspection = if is_v3_package(path) {
        match PraxisCapsulePackage::load_from_dir(path) {
            Ok(package) => package.inspection(),
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
    } else {
        load_legacy_or_exit(path).inspection()
    };

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

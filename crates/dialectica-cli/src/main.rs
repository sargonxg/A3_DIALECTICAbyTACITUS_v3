//! Local DIALECTICA command-line tooling.

use std::path::Path;

use dialectica_capsule::{export_schema_dir, CapsuleBundle, CapsuleManifest, ReviewState};

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
    println!("schema_version={}", manifest.schema_version);
    println!("compiler={}", dialectica_compiler::COMPILER_ID);
    println!("export_ready={}", manifest.is_export_ready());
}

fn validate_bundle(path: &Path) {
    let bundle = load_or_exit(path);
    let report = bundle.validate();

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
    let bundle = load_or_exit(path);
    let inspection = bundle.inspection();

    println!("capsule_id={}", inspection.capsule_id);
    println!("capsule_type={}", inspection.capsule_type);
    println!("review_state={:?}", inspection.review_state);
    println!("source_count={}", inspection.source_count);
    println!("claim_count={}", inspection.claim_count);
    println!("graph_node_count={}", inspection.graph_node_count);
    println!("graph_edge_count={}", inspection.graph_edge_count);
    println!("warning_count={}", inspection.warning_count);
}

fn load_or_exit(path: &Path) -> CapsuleBundle {
    match CapsuleBundle::load_from_dir(path) {
        Ok(bundle) => bundle,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

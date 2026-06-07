//! Local DIALECTICA command-line scaffold.

use dialectica_capsule::{CapsuleManifest, ReviewState};

fn main() {
    let command = std::env::args().nth(1).unwrap_or_else(|| "help".to_owned());

    match command.as_str() {
        "doctor" => print_doctor(),
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
    println!("  doctor   print scaffold health");
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

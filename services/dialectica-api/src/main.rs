//! HTTP API service scaffold.

use dialectica_capsule::CAPSULE_SCHEMA_VERSION;

fn main() {
    println!("dialectica-api scaffold");
    println!("health=ok");
    println!("capsule_schema_version={CAPSULE_SCHEMA_VERSION}");
}

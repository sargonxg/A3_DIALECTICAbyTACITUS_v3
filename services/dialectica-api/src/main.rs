//! HTTP API service scaffold.

use dialectica_capsule::CAPSULE_SPEC_VERSION;

fn main() {
    println!("dialectica-api scaffold");
    println!("health=ok");
    println!("capsule_spec_version={CAPSULE_SPEC_VERSION}");
}

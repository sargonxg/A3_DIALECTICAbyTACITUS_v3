//! Local HTTP API service.

use dialectica_api::{app, default_fixture_dir, ApiState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bind = std::env::var("DIALECTICA_API_BIND").unwrap_or_else(|_| "127.0.0.1:8088".to_owned());
    let fixture_dir = default_fixture_dir();
    let listener = tokio::net::TcpListener::bind(&bind).await?;

    println!("dialectica-api");
    println!("health=ok");
    println!("bind={bind}");
    println!("fixture_dir={}", fixture_dir.display());

    axum::serve(listener, app(ApiState::new(fixture_dir))).await?;
    Ok(())
}

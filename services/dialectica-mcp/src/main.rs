#[tokio::main]
async fn main() {
    let http_requested = std::env::args().any(|arg| arg == "--http")
        || std::env::var("DIALECTICA_MCP_TRANSPORT")
            .map(|value| value.eq_ignore_ascii_case("http"))
            .unwrap_or(false);

    let result: Result<(), Box<dyn std::error::Error>> = if http_requested {
        let bind = dialectica_mcp::default_http_bind();
        dialectica_mcp::run_http(&bind, dialectica_mcp::HttpMcpState::from_env())
            .await
            .map_err(Into::into)
    } else {
        dialectica_mcp::run_stdio().map_err(Into::into)
    };

    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

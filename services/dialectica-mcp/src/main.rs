fn main() {
    if let Err(error) = dialectica_mcp::run_stdio() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

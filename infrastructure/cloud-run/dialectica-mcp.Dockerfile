FROM rust:1.81-slim-bookworm AS build

WORKDIR /app
COPY . .
RUN cargo build --locked --release -p dialectica-mcp

FROM debian:bookworm-slim

RUN useradd --create-home --uid 10001 appuser \
    && apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=build /app/target/release/dialectica-mcp /usr/local/bin/dialectica-mcp
COPY fixtures /app/fixtures

USER appuser
ENV DIALECTICA_MCP_TRANSPORT=http
ENV DIALECTICA_MCP_WORKSPACE=/tmp/dialectica-mcp-workspace
EXPOSE 8080

CMD ["dialectica-mcp", "--http"]

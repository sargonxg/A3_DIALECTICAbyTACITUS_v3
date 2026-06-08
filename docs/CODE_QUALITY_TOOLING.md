# Code Quality Tooling

Date: 2026-06-08

Status: active operating procedure for Codex and future agents.

## Purpose

DIALECTICA should use the same repo-intelligence discipline that works in
PRAXIS: first read the source-of-truth docs, then use local code graphing,
semantic code tooling, focused verification, and a review pass before commit.

These tools improve code quality only when they are tied to concrete gates.
They do not replace Rust tests, fixture validation, source review, or product
judgment.

## Required Agent Loop

Use this order for substantial code or architecture work:

1. Confirm branch and remote:
   `git status --short --branch`, `git remote -v`.
2. Read the active docs:
   `docs/SOURCE_OF_TRUTH.md`, `docs/CODING_LEDGER.md`,
   `docs/NEXT_CODE_BUILD_PLAN.md`, and this file.
3. Refresh repo graph context after meaningful code or doc movement:
   `graphify update .`.
4. Use Serena when symbol-level navigation or refactoring would reduce risk.
5. Implement in small slices with tests.
6. Run the command gate from `docs/CODING_LEDGER.md`.
7. Run a code-review/production-risk pass before commit.
8. Update ledgers when status, commands, or missing work changes.

## Graphify

Current local command:

```powershell
graphify update .
```

Current refreshed graph:

- built from commit `7fa7ae81`;
- `382` nodes;
- `829` edges;
- `22` communities;
- report: `graphify-out/GRAPH_REPORT.md`;
- graph JSON: `graphify-out/graph.json`.

Tracked:

- `graphify-out/GRAPH_REPORT.md`;
- `graphify-out/graph.json`.

Ignored:

- `graphify-out/cache/`;
- `graphify-out/.graphify_root`;
- `graphify-out/manifest.json`;
- `graphify-out/graph.html`.

Reason: the report and graph are useful shared orientation artifacts. The cache,
manifest, root file, and HTML contain machine-local or generated state and are
regenerated locally.

Use Graphify for:

- architecture orientation before large refactors;
- finding god nodes and weakly connected symbols;
- checking whether a generated graph is stale against `git rev-parse HEAD`;
- preserving a navigable code map for future agents.

Do not use Graphify as proof by itself. The current graph is AST/code oriented;
it can miss product/document semantics and some inferred edges need source
verification.

## Serena

Serena is installed locally and configured as a Codex MCP server.

Verified local surfaces:

```powershell
serena --help
codex mcp list
```

Expected Codex MCP command shape:

```toml
[mcp_servers.serena]
command = "serena"
args = ["start-mcp-server", "--project-from-cwd", "--context=codex", "--open-web-dashboard=false"]
```

Use Serena for:

- symbol-level code navigation;
- finding references before edits;
- understanding module relationships without loading whole files;
- safer refactors when line-oriented text search is too blunt.

If Serena tools are not exposed in a session, do not block work. Verify the
MCP listing, use `rg` and Graphify, and record that semantic tools were
unavailable for that pass.

## ECC And Codex Skills

Use skills as discipline, not decoration:

- `incremental-implementation`: multi-file work in small verified slices;
- `test-driven-development` or `tdd-workflow`: new behavior starts with tests
  or at least fixture assertions;
- `code-review-and-quality`: correctness, readability, architecture, security,
  and performance review before commit;
- `production-audit`: launch/readiness and "what breaks in production" review;
- `source-driven-development`: when adding or upgrading libraries, consult
  official docs first;
- `documentation-and-adrs`: update source-of-truth docs and ADRs when
  architecture changes.

The current local MCP/tool surface also includes GitHub, Playwright, Context7,
Exa, Parallel Search, OpenAI Developer Docs, node_repl, memory, and sequential
thinking. Use them only when they reduce risk for the current task.

## Research Refresh

Recent live checks confirm:

- Ladybug remains aligned with the required embedded projection: official docs
  and repository describe an embedded, serverless graph database with Cypher,
  Rust bindings via `lbug`, full-text/vector features, and latest repository
  release `v0.17.1` on 2026-06-02.
- Graphify's useful mode for this repo is incremental code graph refresh.
  External guidance describes the generated report and graph outputs as
  useful codebase navigation artifacts, but the repo should still validate
  graph-derived conclusions against source.
- Serena's official docs support Codex setup with
  `serena start-mcp-server --project-from-cwd --context=codex`.

Sources checked:

- <https://github.com/LadybugDB/ladybug>
- <https://docs.ladybugdb.com/installation/>
- <https://docs.ladybugdb.com/get-started/>
- <https://emelia.io/hub/knowledge-graph-graphify-guide>
- <https://oraios.github.io/serena/02-usage/030_clients.html>
- <https://oraios.github.io/serena/02-usage/020_running.html>

## Commit Gate

Before committing code changes:

```powershell
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
python -m compileall tools/python
python -m unittest discover tools/python/tests
graphify update .
git diff --check
```

For capsule behavior changes, also run the full fixture gate in
`docs/CODING_LEDGER.md`.

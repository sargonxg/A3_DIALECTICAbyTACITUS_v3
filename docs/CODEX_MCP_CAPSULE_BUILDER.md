# Codex MCP Capsule Builder

Status: implemented for the local hardened stdio build loop and first hosted
HTTP `/mcp` loop.

This is the current operator path for building a PRAXIS Capsule with Codex.
It runs locally, does not require cloud credentials, and produces artifacts
that PRAXIS can ingest immediately.

```text
             DIALECTICA by TACITUS

  local docs / notes / source snippets
          |
          v
  dialectica build-docs
          |
          +--> build-source/
          |       source pack, proposals, review decisions
          |
          +--> package/
          |       manifest, evidence, claims, graph.jsonld,
          |       reasoning, review, runtime, Ladybug projection
          |
          +--> *.capsule
          |       portable capsule artifact
          |
          +--> praxis-context-pack.json
          |       PRAXIS agent handoff
          |
          +--> praxis-import.json
                  local/cloud bridge receipt
```

## CLI Quick Start

```powershell
cargo run -p dialectica-cli -- welcome

cargo run -p dialectica-cli -- build-docs `
  --type situation `
  --input .\docs `
  --out $env:TEMP\dialectica-situation-capsule `
  --title "Local Situation Capsule" `
  --workflow decision_brief

cargo run -p dialectica-cli -- inspect $env:TEMP\dialectica-situation-capsule\package
cargo run -p dialectica-cli -- validate $env:TEMP\dialectica-situation-capsule\package
cargo run -p dialectica-cli -- eval $env:TEMP\dialectica-situation-capsule\package --workflow decision_brief
```

Install the local CLI binary as `dialectica`:

```powershell
cargo install --path crates/dialectica-cli
dialectica welcome
```

The output directory contains:

| Path | Purpose |
| --- | --- |
| `package/` | canonical v3 PRAXIS Capsule directory |
| `*.capsule` | portable archive for download, sharing, and storage |
| `praxis-context-pack.json` | compact context pack PRAXIS can inject into agent workflows |
| `praxis-import.json` | bridge record with local paths and future cloud handoff fields |
| `build-source/review_queue.json` | object-level human gates Codex can inspect before promoted use |
| `build-source/promotion_summary.json` | promoted, rejected, evidence-requested, and caveated record summary |
| `build-source/` | source pack, proposal set, reviewer decisions, review queue, and promotion trace |

Supported local source files in this slice: `.txt`, `.md`, `.markdown`,
`.json`, `.jsonl`, `.csv`, and `.tsv`. JSONL files with `role` and `content`
turns are captured as user/assistant discussion transcripts. PDF, OCR, scanned
images, and web capture belong to the next ingestion adapter lane.

## Codex MCP Setup

Print a ready-to-paste Codex config snippet:

```powershell
cargo run -p dialectica-cli -- mcp-config
```

Example:

```toml
[mcp_servers.dialectica]
command = "cargo"
args = ["run", "-p", "dialectica-mcp", "--"]
cwd = "C:\\Users\\giuli\\A3_DIALECTICAbyTACITUS_v3"
```

The MCP server uses the official stdio shape: newline-delimited UTF-8 JSON-RPC
messages over stdin/stdout. It writes logs only to stderr and keeps stdout
MCP-only.

The local server supports MCP protocol version `2025-11-25`. It accepts the
normal `initialize` request followed by the `notifications/initialized`
notification. Unsupported protocol versions return a JSON-RPC `-32602` error.

## MCP Surface

Tools:

| Tool | Purpose |
| --- | --- |
| `dialectica_welcome` | returns the operator welcome |
| `dialectica_build_capsule` | builds a capsule from local documents |
| `dialectica_upload_sources` | uploads text source files into the hosted MCP workspace and returns a `build_id` |
| `dialectica_build_uploaded_capsule` | builds a capsule from previously uploaded hosted source files |
| `dialectica_capture_discussion` | writes a user/assistant discussion JSONL source file for capsule ingestion |
| `dialectica_inspect_capsule` | inspects a compiled package and Ladybug projection metadata |
| `dialectica_validate_capsule` | validates a compiled package and returns precise findings |
| `dialectica_capsule_status` | returns manifest, review, Ladybug, archive, PRAXIS pack, and hosted-readiness status |
| `dialectica_review_queue` | reads the local review queue for object-level human gates |
| `dialectica_get_protocol` | returns a typed BUILD elicitation protocol fixture |
| `dialectica_score_protocol_session` | scores a protocol session against completeness rules |
| `dialectica_archive_capsule` | writes a portable `.capsule` archive |
| `dialectica_export_praxis_pack` | emits PRAXIS-readable context JSON |
| `dialectica_ontology_plan` | returns the capsule-specific ontology blueprint |
| `dialectica_ladybug_query` | runs a single read-only Cypher query against a queryable embedded Ladybug projection |
| `dialectica_praxis_handoff` | reads `praxis-import.json` and returns the PRAXIS handoff receipt |
| `dialectica_mcp_config` | returns the local Codex MCP config snippet |

Every tool advertises both `inputSchema` and `outputSchema`. Successful tool
calls return `structuredContent` plus the same JSON serialized in a text content
block for older clients. Invalid tool arguments return an MCP tool result with
`isError: true`; malformed JSON-RPC requests return JSON-RPC errors.

### Tool Contracts

| Tool | Required input | Optional input | Structured output |
| --- | --- | --- | --- |
| `dialectica_welcome` | none | none | `{ "welcome": string }` |
| `dialectica_build_capsule` | `capsule_type`, `input_dir`, `out_dir` | `title`, `workflow`, `mode` | builder receipt paths, counts, digests, and `promotion_note` |
| `dialectica_upload_sources` | `files` | `build_id`, `overwrite` | `build_id`, hosted workspace path, uploaded file count, and `next_tool` |
| `dialectica_build_uploaded_capsule` | `build_id`, `capsule_type` | `title`, `workflow`, `mode` | builder receipt paths, counts, digests, hosted paths, and `promotion_note` |
| `dialectica_capture_discussion` | `out_file`, `turns` | turn `timestamp` values | discussion JSONL path and turn count |
| `dialectica_inspect_capsule` | `package_dir` | none | manifest, review state, counts, validation boolean, Ladybug status |
| `dialectica_validate_capsule` | `package_dir` | none | `valid`, finding counts, and `findings` |
| `dialectica_capsule_status` | `package_dir` | `archive_file`, `praxis_pack_file` | manifest, review state, validation summary, Ladybug status, archive status, PRAXIS pack status, hosted MCP note |
| `dialectica_review_queue` | `review_queue_file` or `build_source_dir` | none | local `review_queue_v1` artifact |
| `dialectica_get_protocol` | `capsule_type` | none | `elicitation_protocol_v1` fixture for `user`, `situation`, `tool`, or `output` |
| `dialectica_score_protocol_session` | `capsule_type`, `session` | none | deterministic completeness score; it does not promote derived records |
| `dialectica_archive_capsule` | `package_dir` | `out_file` | archive receipt with path, entries, and digest |
| `dialectica_export_praxis_pack` | `package_dir` | `workflow`, `out_file` | full context pack or written-pack receipt |
| `dialectica_ontology_plan` | `package_dir` | none | capsule ontology blueprint |
| `dialectica_ladybug_query` | `package_dir`, `query` | none | columns, rows, and row count; query must be one read-only `MATCH` or `RETURN` statement and must not contain mutating Cypher keywords |
| `dialectica_praxis_handoff` | `import_file` or `package_dir` | none | local PRAXIS import receipt plus handoff note |
| `dialectica_mcp_config` | none | none | `{ "config": string }` |

Path inputs are local-stdio only. Existing input directories are canonicalized.
Output targets reject filesystem roots, parent traversal, and archive writes
inside the package directory. Set `DIALECTICA_MCP_ROOTS` to a semicolon-delimited
root list to force local MCP paths under explicit filesystem roots.

### Example Tool Calls

Initialize:

```json
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{"roots":{"listChanged":true}},"clientInfo":{"name":"codex","version":"1"}}}
```

List tools:

```json
{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}
```

Build a local situation capsule:

```json
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"dialectica_build_capsule","arguments":{"capsule_type":"situation","input_dir":"docs","out_dir":"C:\\Users\\giuli\\AppData\\Local\\Temp\\dialectica-mcp-situation","title":"Local Situation Capsule","workflow":"decision_brief","mode":"assisted"}}}
```

Validate and check status:

```json
{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"dialectica_validate_capsule","arguments":{"package_dir":"C:\\Users\\giuli\\AppData\\Local\\Temp\\dialectica-mcp-situation\\package"}}}
{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"dialectica_capsule_status","arguments":{"package_dir":"C:\\Users\\giuli\\AppData\\Local\\Temp\\dialectica-mcp-situation\\package"}}}
```

Read review gates and PRAXIS handoff:

```json
{"jsonrpc":"2.0","id":6,"method":"tools/call","params":{"name":"dialectica_review_queue","arguments":{"review_queue_file":"C:\\Users\\giuli\\AppData\\Local\\Temp\\dialectica-mcp-situation\\build-source\\review_queue.json"}}}
{"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"dialectica_praxis_handoff","arguments":{"package_dir":"C:\\Users\\giuli\\AppData\\Local\\Temp\\dialectica-mcp-situation\\package"}}}
```

Resources:

| URI | Purpose |
| --- | --- |
| `dialectica://welcome` | operator welcome |
| `dialectica://builder/contract` | what each capsule build contains |
| `dialectica://praxis/bridge` | how PRAXIS consumes local and cloud artifacts |
| `dialectica://hosted/mcp` | hosted `/mcp` behavior and auth/path restrictions |

Prompt:

| Prompt | Purpose |
| --- | --- |
| `build_context_capsule` | Codex-mediated flow for classifying, building, inspecting, and handing off a capsule |

## PRAXIS Bridge

Local bridge now:

1. `praxis-context-pack.json` is the immediate PRAXIS agent input.
2. `*.capsule` is the portable artifact to save, download, or share.
3. `praxis-import.json` links the context pack, archive, manifest, graph,
   Ladybug projection metadata, and source pack.

Cloud bridge now:

1. deploy `dialectica-mcp` to Cloud Run with the bearer token from Secret
   Manager;
2. upload source text to `/mcp` with `dialectica_upload_sources`;
3. build with `dialectica_build_uploaded_capsule` using the returned `build_id`;
4. inspect and validate the server-side package before handing the capsule to
   PRAXIS.

Cloud bridge next:

1. upload `*.capsule` to Cloud Storage;
2. persist build, review, artifact, and export state in Cloud SQL PostgreSQL;
3. expose authenticated DIALECTICA API/MCP routes that return the context pack
   and signed artifact URL by `build_id`, `capsule_id`, or `artifact_id`;
4. let PRAXIS load the context pack into Ask PRAXIS and keep the archive as the
   downloadable source-of-truth artifact.

## Local vs Hosted MCP

| Concern | Local stdio MCP now | Hosted HTTP MCP now | Durable hosted MCP later |
| --- | --- | --- | --- |
| Transport | newline-delimited JSON-RPC over stdin/stdout | single `/mcp` endpoint using JSON-RPC POST | same endpoint with resumable/session-aware clients |
| Auth | local process trust and OS permissions | bearer token plus optional Origin allow-list | OAuth/service auth, tenant ownership checks, token audience validation |
| Inputs | local filesystem paths under optional `DIALECTICA_MCP_ROOTS` | uploaded text files addressed by `build_id` | `build_id`, `capsule_id`, and durable artifact IDs |
| Artifact storage | local directories and `.capsule` files | Cloud Run instance workspace | Cloud Storage objects plus Cloud SQL state |
| PRAXIS access | local `praxis-context-pack.json` or archive handoff | MCP-inspected server-side artifact paths | authenticated API/MCP call or signed artifact URL |
| Promotion | draft/assisted outputs with caveats | same review gates; no silent canonical promotion | same review gates; no silent canonical promotion |

## Human Gate Posture

The local builder creates review decisions as `approve_with_caveats` so the
compiler can produce a usable package while preserving the audit warning. This
is deliberately not expert certification.

Future assisted mode should let a human or expert reviewer:

- approve, reject, or request more evidence for proposals;
- edit ontology terms and graph edges;
- harden language, rights, and output rules;
- recompile the capsule with the updated decision set.

## What This Enables

Codex can now mediate the complete local loop:

```text
user asks for a capsule
  -> Codex classifies user/situation/tool/output
  -> MCP builds package + archive + PRAXIS pack
  -> Codex inspects caveats, graph, ontology, and source receipts
  -> user reviews what needs human gating
  -> PRAXIS receives the context pack or capsule archive
```

This keeps the visible user workflow simple while preserving the deep substrate:
sources, claims, temporality, ontology, semantic graph, reasoning devices,
language rules, rights rules, output contracts, review receipts, and a portable
bundle.

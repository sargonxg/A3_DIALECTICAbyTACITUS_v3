# Codex MCP Capsule Builder

Status: implemented for the local text-document build loop.

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
| `build-source/` | source pack, proposal set, and reviewer decision trace |

Supported local source files in this slice: `.txt`, `.md`, `.markdown`,
`.json`, `.jsonl`, `.csv`, and `.tsv`. PDF, OCR, scanned images, web capture,
and conversation ingestion belong to the next ingestion adapter lane.

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
messages over stdin/stdout. It writes logs only to stderr.

## MCP Surface

Tools:

| Tool | Purpose |
| --- | --- |
| `dialectica_welcome` | returns the operator welcome |
| `dialectica_build_capsule` | builds a capsule from local documents |
| `dialectica_inspect_capsule` | inspects a compiled package and Ladybug projection metadata |
| `dialectica_archive_capsule` | writes a portable `.capsule` archive |
| `dialectica_export_praxis_pack` | emits PRAXIS-readable context JSON |
| `dialectica_ontology_plan` | returns the capsule-specific ontology blueprint |
| `dialectica_mcp_config` | returns the local Codex MCP config snippet |

Resources:

| URI | Purpose |
| --- | --- |
| `dialectica://welcome` | operator welcome |
| `dialectica://builder/contract` | what each capsule build contains |
| `dialectica://praxis/bridge` | how PRAXIS consumes local and cloud artifacts |

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

Cloud bridge next:

1. upload `*.capsule` to Cloud Storage;
2. persist `praxis-import.json` in Firestore or Cloud SQL;
3. expose a DIALECTICA API route that returns the context pack and signed
   artifact URL by `capsule_id`;
4. let PRAXIS load the context pack into Ask PRAXIS and keep the archive as the
   downloadable source-of-truth artifact.

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

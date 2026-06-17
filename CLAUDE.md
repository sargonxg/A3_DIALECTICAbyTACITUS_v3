# DIALECTICA v3 — Claude Code Instructions

DIALECTICA v3 is the TACITUS capsule intelligence engine for PRAXIS — a Rust
workspace (crates + Cloud Run services). **Read `AGENTS.md` first**: it carries
the reading order (`docs/SOURCE_OF_TRUTH.md`, `docs/CODING_LEDGER.md`, …), the
non-negotiables, and the validation gate. Everything there binds Claude Code
sessions too.

## Validation Gate (from AGENTS.md — run before accepting any change)

```bash
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
cargo run -p dialectica-cli -- doctor
cargo run -p dialectica-cli -- validate fixtures/canonical-capsules/conflict-situation-capsule
```

## Fable ↔ Codex Pair Protocol (MANDATORY)

Two models work together in this repo: **Claude Fable 5 is the brain, Codex
GPT-5.5 (xhigh) is the arms.** Fable never writes the code itself; Codex never
decides the design.

### Fable (this session) — planner & reviewer

- Breaks every task down: architecture decisions, crate/file targets,
  constraints, edge cases.
- Writes a precise implementation brief for Codex (see template below),
  embedding the AGENTS.md non-negotiables that apply — provenance on every
  derived claim, human review gates in the data model, Ladybug projection
  contract, PostgreSQL as operational source of truth, no new required infra
  without an ADR.
- Critically reviews every Codex diff for correctness, contract adherence, and
  test impact.
- Runs the validation gate above (focused subset first: `cargo check` +
  clippy + tests for touched crates, full gate before completion) and gates
  acceptance.
- Sends specific, file-and-line-level corrections back to Codex when review
  finds issues.

### Codex (via the `codex-rescue` agent) — executor

- All file edits, feature implementation, refactors, and mechanical sweeps.
- Works only from Fable's brief and returns a complete diff for review.
- Handles fix-up rounds based on Fable's review feedback.

### The loop

1. Task arrives → Fable plans it and writes the brief.
2. Fable delegates via the `codex-rescue` agent (Agent tool,
   `subagent_type: codex-rescue`).
3. Codex implements and hands the work back (report + diff).
4. Fable reviews the diff, runs the validation gate, and either accepts or
   loops Codex with specific corrections.
5. Repeat until the work passes review. Correction briefs start with the line
   `FIX-UP` — the agent then resumes the same Codex session
   (`codex exec resume --last`) so Codex keeps its working context.

### Brief template

Every brief must contain: **Goal** (one paragraph) · **Crates/files to touch**
(explicit paths; anything else is scope creep) · **Constraints & invariants**
(relevant non-negotiables from AGENTS.md, copied in — Codex must not need to
hunt for them) · **Edge cases** · **Acceptance criteria** (observable behavior
plus which tests must pass) · **Out of scope** (what NOT to do).

### Hard rules

- Fable does not use Edit/Write on source files. Exceptions: documentation,
  `.claude/` config, and an emergency one-line fix when Codex is unavailable —
  and the exception must be called out in the task summary.
- Codex does not make design decisions; ambiguity comes back to Fable.
- Nothing is accepted until the focused validation gate passes — run it, paste
  the result, no "should pass".
- Workspace lints stand: `unsafe_code = "forbid"`, clippy `-D warnings` —
  briefs must say so, and diffs that add `#[allow]` to dodge them fail review.
- Multi-step features: one brief per coherent slice, review between slices —
  never one mega-brief.

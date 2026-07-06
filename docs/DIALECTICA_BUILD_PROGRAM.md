# DIALECTICA — The Build Program
## Engineering Translation of the Capsule Capture Doctrine
### Document 2 of 2 · Codex-executable program · Companion: `DIALECTICA_CAPSULE_CAPTURE_DOCTRINE.md`

**Repo:** `sargonxg/A3_DIALECTICAbyTACITUS_v3` · **Date:** 2026-07-02 · **Basis:** live audit of `main` (~15.1k Rust LOC, 8 crates, 3 services, capsule-3.0 schemas, ADRs 001–012)
**Standing:** subordinate to `CAPSULE_SPEC.md` → `SOURCE_OF_TRUTH.md` → `CODING_LEDGER.md` → `ENGINEERING_BASELINE.md` → ADRs. This program supersedes the two earlier plan drafts and folds their still-valid lanes in. Rules of engagement for Codex: one work package per PR series; ADR before any new schema family or boundary; nothing enters the ledger as done without command + fixture + test evidence; ADR-004 Rust stack throughout; ADR-007 proposal boundary is inviolable; Ladybug remains the required embedded projection (ADR-008); no graph/vector DB as required infrastructure without an ADR.

---

## 0. What already exists (do not rebuild — verified 2026-07-02)

Working: v3 contract + validators + schema export; local build loop (docs/JSONL → source pack → proposals → review triggers → editable decisions → promotion → deterministic compile → `.capsule` → PRAXIS context pack + import receipt); review gates as data model; Ladybug projection + read-only Cypher; diff + cited change memo (ADR-009); integrity envelope (ADR-011); typed elicitation protocols for all four types + completeness scoring + deterministic transcript→proposal drafts (ADR-012); stdio MCP server (16 tools).
Missing: live model providers; live AI-conducted sessions; PDF/OCR/web ingestion; all connectors; Postgres store; store-backed API; durable task handler; Cloud Run deployment; hosted MCP; composition compiler; manifest-level Purpose; eval harness beyond MVP; L3/L5 graph layers as explicit artifacts.

**Doctrine → repo mapping at a glance**

| Doctrine element (Doc 1) | Repo home |
|---|---|
| Six capture channels C1–C6 | `dialectica-builder` (C1/C5), new `dialectica-connectors` (C2), `dialectica-elicitor` (C3), PRAXIS event feed → builder (C4), fork/import path in `builder` + `marketplace_listing` (C6) |
| Six graph layers L0–L5 | L0 = source ledger + discussion JSONL; L1 = promoted record set; L2 = `graph.jsonld` + Ladybug; L3/L5 = **new** package files (`graph/inferred.jsonld`, `coverage/gap_map.json`); L4 = `ontology_blueprint` (exists) + **new** blueprint version ledger |
| Dynamic ontology (CQs, amendments) | `ontology_blueprint.schema.json` (extend), Graph Profile Registry doc, **new** `blueprint_amendment` proposal type in `dialectica-extractor` |
| Clarification engine T1–T9 | **new** trigger registry in `dialectica-elicitor`, fed by validator findings, review triggers, gap map, staleness |
| Per-type playbooks | protocol fixtures `*.v2` + type-specific builder passes (WP-4) |
| Adversarial pass (R4) | **new** `dialectica-agon` module (critique pass) inside `extractor` or sibling crate |
| Purpose keystone | **new** `purpose.schema.json` + manifest `purpose_profile` (WP-1) |
| Composition payoff | **new** composition compiler in `dialectica-compiler` (WP-7) |

---

## 1. Work packages

Each WP: **Objective · Key moves · Schemas/ADRs · Definition of Done.** Local-first ordering preserved: WP-1…WP-5 run laptop-local; WP-6+ is the cloud arc.

### WP-1 — Purpose Profile (small; first; unblocks everything)
**Objective:** Purpose as a required, machine-checkable manifest layer (Doctrine IV.3).
**Key moves:** `purpose.schema.json` (`serves_decision`, `audience`, `time_horizon`, `scope_in/out`, `success_criteria`, `red_lines[]`, `misuse_conditions[]`, `staleness_policy{warn,refuse}`); required `purpose_profile` on manifest (schema-version bump per repo discipline); validator refuses promoted packages without complete Purpose; `capsule_health` computes staleness against policy; context pack and `agent_context.md` render Purpose **first**; protocol stage-1 records populate Purpose as reviewable proposals.
**Schemas/ADRs:** ADR-013 *Purpose profile as required manifest layer*.
**DoD:** golden fixtures gain `purpose.json`; missing-Purpose fixture fails `validate`; stale-beyond-policy fixture exports machine-readable `refuse_generation`; all existing gates green.

### WP-2 — Provider Layer (live models behind the proposal boundary)
**Objective:** real extraction/interview models without touching promotion semantics.
**Key moves:** `ModelProvider` trait in `dialectica-extractor` (`invoke(prompt, spans) → proposals + ModelInvocationReceipt`); Vertex AI Gemini first implementation (bulk tier for extraction, reasoning tier for dense text and interviewing; thinking-level pass-through); **model strings in exactly one registry module** + CI `rg` guard; retry/error envelopes, fallback policy, cost/latency counters into `extraction_run`; recorded-cassette test mode — zero live calls in CI; source-bound prompting: proposals citing spans outside the pack fail the existing `proposal-check`.
**Schemas/ADRs:** none new (ADR-007 governs); provider-registry note in `ENGINEERING_BASELINE.md`.
**DoD:** `build-docs --mode live` on the golden source pack yields schema-valid proposals with receipts; `--mode fixture` stays byte-deterministic; promotion behavior identical in both modes.

### WP-3 — Layered Graph Artifacts (L3 inferred + L5 gap map + blueprint evolution)
**Objective:** make the doctrine's six-layer model explicit in the package and queryable.
**Key moves:** (1) `graph/inferred.jsonld` (L3): model-derived links/patterns/cluster summaries, visibly non-canonical, each with receipt + confidence, promotable only via review; excluded from promoted context packs by default. (2) `coverage/gap_map.json` (L5): computed at every compile — empty blueprint slots, actors without interests, claims without temporal status, sources without postures, quarantined-unadjudicated spans; this file is the clarification engine's fuel. (3) Blueprint evolution: `blueprint_amendment` proposal type (new class/relation + definition + domain/range + evidence instances from the unmappable ledger); accepted amendments version the blueprint; back-fill runs re-extract under the new version with receipts. (4) Ladybug projection extended to expose layer labels so `ladybug-query` can filter promoted vs. inferred.
**Schemas/ADRs:** `inferred_graph`, `gap_map`, `blueprint_amendment` schemas; ADR-014 *Layered graph artifacts and blueprint evolution* (keeps L3/L5 non-canonical; amendments are proposals).
**DoD:** golden fixture emits deterministic `gap_map.json`; a fixture with 3 planted unmappables produces one amendment proposal; a promoted amendment triggers versioned back-fill; Cypher filter by layer works; default context pack contains zero L3 records.

### WP-4 — Type-Specific Builder Passes (the four playbooks, executable)
**Objective:** four genuinely different pipelines behind one `build` interface (Doctrine Part III).
**Key moves, per type:**
- **User:** stylometry pass over exemplars → span-backed `voice_profile` trait proposals; standing-position mining across documents; record families `audience_profile`, `red_line`, `team_routing`, `privacy_rule` added to the proposal taxonomy; composition firewall metadata (User records marked non-factual).
- **Situation:** thread-aware email + revision-aware doc normalization (lands fully in WP-6); contradiction ledger as a first-class package file; `source_posture` records (assert/attribute/hedge per source); commitment-compliance checker (commitment vs. event records → review triggers).
- **Tool:** record families `device`, `heuristic`, `cue`, `trap`, `precedent`, `salience_prior`, `quality_check`, `human_checkpoint` (extend `reasoning_playbook` — most exist; add `cue`, `quality_check`, `human_checkpoint`, `when_not_to_use`); exemplar cross-check pass (elicited rules tested against the expert's own documents → refinement triggers); completeness meter from the existing scoring machinery, surfaced per Doctrine III.3.
- **Output:** structure-induction pass (section segmentation, ordering, length bands, opening moves) and style-induction pass over exemplars → rule proposals with exemplar spans; divergence report between exemplars → `audience_adaptation` proposals; `anti_exemplar` records; contract self-test (exemplars must pass their own contract; a mutated draft must fail it) wired into `eval`.
**Schemas/ADRs:** record-family extensions across proposal + playbook + output-contract schemas; ADR-015 *Type-specific record families and builder passes*.
**DoD:** four golden builds (one per type) each produce type-correct promoted sets; the Output contract self-test passes/fails as specified; the Tool exemplar cross-check produces ≥1 refinement trigger on a planted inconsistency; User voice traits all carry exemplar spans.

### WP-5 — Live Elicitation Runtime + Clarification Engine (the dialogue engine)
**Objective:** AI-conducted, protocol-governed, gap-driven interviews for all four types — the doctrine's C3 channel, R3 discipline, and T1–T9 trigger taxonomy — with the model strictly on the proposal side.
**Key moves:** (1) `dialectica-elicitor` crate: session runtime walking a protocol's stage machine; the model phrases questions and drafts proposals from answers; every turn persists via the existing discussion-capture path (transcript = source). (2) **Trigger registry (T1–T9):** typed triggers raised by Purpose validation (T1), gap map (T2), contradiction ledger (T3), confidence bands (T4), protocol tacit-residual stages (T5), new-source events (T6), PRAXIS behavioral feed (T7, lands with WP-8 integration), quarantine (T8), staleness (T9). The interviewer consumes triggers, never free-associates; each question records its trigger id. (3) **Protocol v2 fixtures** for all four types implementing the doctrine's stages — including the Tool protocol's CDM adaptation: incident selection, timeline sweep, decision-point probes, counterfactual sweep, generalization + exemplar cross-check, checkpoint mapping. (4) **AGON adversarial pass** as the terminal stage of every session: counterexamples to heuristics, contract self-tests, contradiction sweeps; findings file as review triggers; the attack transcript is a source. (5) Resumable sessions (`elicitation_session` schema exists — extend with trigger log + stage state); deterministic replay of recorded sessions for CI.
**Schemas/ADRs:** `clarification_trigger` schema; protocol `*.v2` fixtures; ADR-016 *Live elicitation boundary* (model may ask and draft; may not promote; every exchange is span-addressed evidence) + ADR-017 *AGON critique pass as promotion prerequisite*.
**DoD:** a live (cassette-recorded) `tool.v2` session yields ≥10 devices/heuristics, ≥3 cues, ≥1 trap, ≥1 precedent, all span-traced; a `situation.v2` session asks ≥1 question whose trigger id resolves to a real gap-map entry and ≥0 questions answerable from the corpus (audited); promotion is blocked until the AGON pass completes; session replay is deterministic.

### WP-6 — Ingestion Adapters: Documents then Connectors (C1 heavy + C2)
**Objective:** from local text files to the user's working environment — PDF/OCR/web, then Gmail, Drive + shared drives/folders, Calendar, feeds — consent-scoped, quarantine-first, idempotent.
**Key moves:** *6a documents:* PDF/OCR/web-capture adapters emitting existing `SourceDocument`/`SourceSpan` types with stable locators + hashes; binary staging (local dir → GCS in cloud mode). *6b connectors:* one `SourceConnector` trait (`discover/fetch/watch`) upstream of the normal source-pack path — never a parallel pipeline; **consent manifest** per tenant (exact labels/folders in scope; read-only OAuth; tokens in Secret Manager; nothing outside the manifest is fetched); idempotent re-sync on external ids; upstream deletions expire records temporally; email specifics (threading preserved, participants/dates as blueprint-governed candidates, quoting stripped with lineage, **quarantine metadata on every external span** with T8 routing); Drive specifics (revision ids, shared-drive permission snapshots into `rights_profile`).
**Schemas/ADRs:** `consent_manifest`, `connector_sync_receipt` schemas; ADR-018 *Connector consent and scope model*; ADR-019 *External-content quarantine and injection posture*.
**DoD:** a consented Gmail label + one shared Drive folder continuously feed a Situation build; re-sync is a no-op absent changes; a planted injection email lands quarantined and review-blocked; permission snapshot appears in the compiled rights profile; CI uses recorded cassettes only.

### WP-7 — Composition Compiler (1U + nS + nT + 1O)
**Objective:** the deterministic multi-capsule compile PRAXIS and TAG depend on.
**Key moves:** type-rule validation; canonical ordering + content-addressed hash (input-order invariant); **strictest-wins merge** of output contracts, language profiles, rights; User-capsule composition firewall enforced (non-factual records only); cross-situation same-entity contradiction surfacing *into* the pack per the Output contract's combination rules; budget tiers S/M/L with compression report; **compile inspector payload** (per-capsule contribution map, dropped-content report, merged contract, hash) rendered verbatim by PRAXIS's Context Inspector; device/heuristic attribution flows through so generation cites which Tool device shaped which judgment (R5).
**Schemas/ADRs:** `composition_request`, `compile_inspector` schemas; ADR-020 *Composition contract and strictest-wins merge*.
**DoD:** golden 1U+2S+2T+1O compile byte-identical across runs/OS with committed expected output; permutation never changes the hash; a fixture memo carries device-id margin attributions end-to-end; a factual claim planted in a User capsule fails composition.

### WP-8 — Durable Service on GCP (Postgres, tasks, Cloud Run, PRAXIS OIDC)
**Objective:** the proven loop, durable and hosted — builds survive restarts.
**Key moves:** SQLx migrations for source ledger, builds, proposals, reviews, sessions, promotions, bundles (`dialectica-store` family names exist — implement); store-backed Axum routes behind the *same* `API_CONTRACT.md`, extended first with `/protocols`, `/sessions`, `/diff`, `/compose`, `/connectors`, `/exports`; Cloud Tasks-compatible handler with idempotent stage execution (crashed builds resume); GCS artifact storage; OIDC for PRAXIS; Cloud Build + Artifact Registry per ADR-002; tracing spans per stage; per-tenant cost counters. PRAXIS behavioral feed (C4/T7) lands here: chat-edit events → `dialectica_capture_discussion`-class ingestion → preference-refinement triggers.
**Schemas/ADRs:** none new (ADR-002/003/004 govern); deployment docs updated.
**DoD:** kill-mid-build → resume → identical promoted package hash; PRAXIS staging POSTs a build and GETs the pack over OIDC; fixture API tests pass unchanged against the store-backed implementation.

### WP-9 — Hosted MCP (Codex connects over the network)
**Objective:** the MCP surface, tenant-scoped, reachable remotely — with gates provably human.
**Key moves:** threat-model ADR **first** (authn: per-tenant keys + OIDC; authz: tenant isolation per tool; path restrictions; rate limits; injection posture on agent-supplied arguments); streamable-HTTP transport alongside stdio over one tool registry; add WP-5 session tools and WP-7 `compose`; **promotion and review mutation never exposed over MCP** (negative tests required); `ladybug_query` stays read-only-guarded.
**Schemas/ADRs:** ADR-021 *Hosted MCP threat model and tool exposure policy*.
**DoD:** Codex against the hosted URL lists capsules, runs a scoped layer-filtered Cypher query, drives a full elicitation session, compiles a composition, and exports a PRAXIS pack — with zero human keystrokes except promotion, which is provably impossible over MCP.

### WP-10 — TAG Serving Modes (TACITUS Augmented Generation)
**Objective:** serve purpose-declared, contract-bound, cited context to any model — the payoff.
**Key moves:** three modes over one contract: (a) **compiled pack** (exists; extend with Purpose-first ordering + tiers); (b) **scoped live query** — QA against a capsule's promoted records via Ladybug + span retrieval, every answer citing record ids (read-only, non-canonical); (c) **diff mode** — the change memo as incremental context (exists; promote to first-class). All modes embed Purpose, merged contracts, language profile, staleness state, and refusal conditions so a downstream agent can itself refuse. Attribution obligations are part of the serving contract; PRAXIS's validator closes the loop against the Output contract. Resolve naming (PRAXIS vs. TACITUS Augmented Generation) in the ADR — recommendation: TAG as the company capability, PRAXIS Capsules as the artifact.
**Schemas/ADRs:** ADR-022 *TAG serving contract* (+ naming note).
**DoD:** a reference fixture agent given a TAG pack produces a draft whose every judgment carries a claim citation or device id; the same agent given a stale capsule refuses with the machine-readable reason.

### WP-11 — Eval Harness & Capture-Quality Metrics
**Objective:** prove the thesis with numbers, per capsule type.
**Key moves:** raw-baseline vs. capsule-augmented golden comparisons scored on source fidelity, temporal correctness, contract compliance, reasoning-device adherence; **capture metrics:** span-traceability rate (target 100% of promoted records), interview efficiency (records per question; % of questions with corpus-answerable answers — target ~0, per R3), Tool depth (devices/traps/precedents per expert-hour; counterexample-survival rate), Output contract discrimination (pass own exemplars, fail mutated drafts), gap-closure velocity (L5 entries resolved per session); scorecard published into `capsule_health`.
**DoD:** eval report shows measurable improvement or actionable failures, including ≥1 per-type capture metric; diff-correctness checks folded in.

---

## 2. Sequencing

```
LOCAL (Demo Gate stays laptop-local, per NEXT_CODE_BUILD_PLAN):
  WP-1 Purpose → WP-2 Providers → WP-3 Graph layers → WP-4 Type passes → WP-5 Elicitor+Triggers+AGON
        (WP-3 ∥ WP-4 after WP-2; WP-7 Composition may start after WP-1, pure-kernel work)

LOCAL DEMO GATE (extends the existing one):
  real documents + a live interview → four typed, reviewed, Purpose-bearing capsules
  → AGON-passed, promoted, signed → composed 1U+1S+1T+1O → diff → cited change memo.

CLOUD ARC:
  WP-6a PDF/OCR → WP-8 Durable service → WP-6b Connectors → WP-9 Hosted MCP → WP-10 TAG → WP-11 Evals

INVESTOR DEMO this buys: connect a Gmail label + shared Drive folder Monday; DIALECTICA reads,
understands, and proposes; Tuesday you interview the gaps and gate; Wednesday Codex connects over
MCP and drafts under the Output contract with margin citations; Friday's diff is a cited
"what changed this week" memo.
```

## 3. ADR register for this program

ADR-013 Purpose profile · ADR-014 Layered graph artifacts & blueprint evolution · ADR-015 Type-specific record families · ADR-016 Live elicitation boundary · ADR-017 AGON pass as promotion prerequisite · ADR-018 Connector consent/scope · ADR-019 Quarantine/injection posture · ADR-020 Composition contract · ADR-021 Hosted MCP threat model · ADR-022 TAG serving contract (+ naming).

## 4. Anti-patterns Codex must refuse (program-specific; extends the repo's own list)

- "Auto-promote at confidence ≥0.95" — confidence routes review, never bypasses it (ADR-007).
- "Let the interviewer write directly to promoted records to save a round-trip" — the model asks and drafts; humans promote (ADR-016).
- "Merge the four builders into one generic pipeline with a `type` flag" — the type passes differ in substance (WP-4); a flag is not a playbook.
- "Skip the AGON pass for small capsules" — critique is a promotion prerequisite, size-independent (ADR-017).
- "Ask the user something the corpus answers" — R3; audited by the WP-11 interview-efficiency metric.
- "Mutate the ontology blueprint inline when extraction doesn't fit" — amendments are proposals with evidence (WP-3).
- "Put inferred (L3) content in the default context pack" — promoted-only by default; inferred is opt-in and labeled.
- "Fetch the whole mailbox and filter later" — the consent manifest defines fetch scope (ADR-018).
- "Expose promote/review over hosted MCP" — gates are human, permanently (ADR-021, negative-tested).
- "Hardcode a model string outside the provider registry" — one module, CI-guarded (WP-2).
- "Mark a WP done in the ledger with tests to follow" — no ledger claim without command + fixture + test evidence.

## 5. Release checklist (the council's five questions, operationalized)

R1 Analyst: every promoted record defensible — span-traceability 100%, caveats intact through compile. R2 Ontologist: every extraction run bound to a versioned blueprint; amendments only by proposal. R3 Elicitor: interview-efficiency metric shows ~0 corpus-answerable questions. R4 Reviewer: AGON pass complete; quarantine adjudicated; rejection lineage preserved. R5 Runtime engineer: every record addressable by stable id through compile into the served context; contracts machine-checkable. **Ship nothing that fails any of the five.**

*End of Document 2.*

# DIALECTICA — The Capsule Capture Doctrine
## How the Engine Captures, Structures, and Graphs the Four Capsule Types
### Document 1 of 2 · Design doctrine · Companion: `DIALECTICA_BUILD_PROGRAM.md`

**Repo:** `sargonxg/A3_DIALECTICAbyTACITUS_v3` · **Date:** 2026-07-02
**Standing:** This is the *design doctrine* — the reference model for what DIALECTICA must be able to capture, from where, into what structures, and by what conversational and extractive means. It is a planning layer subordinate to repo governance (`CAPSULE_SPEC.md` → `SOURCE_OF_TRUTH.md` → `CODING_LEDGER.md` → ADRs). Its engineering translation — lanes, schemas, ADRs, acceptance gates — lives in Document 2. Nothing here is "implemented" until the ledger says so with command, fixture, and test evidence.

**Research grounding (verify before load-bearing use):** ontology-guided LLM extraction with competency-question scoping and schema-supervised population, and dynamic schema induction (survey arXiv:2510.20345; AutoSchemaKG arXiv:2505.23628; ODKE+ per-entity-type ontology snippets, Apple ML Research); Critical Decision Method / Applied Cognitive Task Analysis for tacit expert knowledge (Hoffman, Crandall & Shadbolt 1998; Brown, Power & Gore 2025); episodic→semantic→community temporal graph memory with bi-temporal validity (Zep/Graphiti, arXiv:2501.13956).

---

# PART I — THE COUNCIL: FIVE PERSONAS DESIGN THE ENGINE

Before specifying pipelines, the doctrine is stress-tested by five personas. Each states what the engine must do *for them*, and each contributes one binding design rule. Codex agents should re-read this council whenever a design choice feels ambiguous: the correct answer is usually the one that satisfies all five simultaneously.

## Persona 1 — The Analyst (former UN desk officer; the paying user)

*"I don't need another chatbot. I need the thing that remembers what I know, the way I know it. When I draft a Council product, I carry in my head: who the actors are and what they actually want (not what they say), which sources I trust for what, what my office's red lines are, how my Director wants an executive summary to open, and which analytical mistakes I've sworn never to repeat. Today all of that dies in my head or in scattered docs. If DIALECTICA captures it, the test is simple: hand a capsule set to an agent, and the draft that comes back should read like it was written by someone who has sat at my desk for a year — and every judgment in it should be checkable."*

**The Analyst's rule (R1 — Fidelity over volume):** a capsule with 40 promoted, source-bound, correctly-caveated records beats one with 4,000 noisy ones. Capture pipelines optimize precision and traceability first, recall second. Every capture channel must answer: *could the Analyst defend this record in front of their Director?*

## Persona 2 — The Knowledge Engineer / Ontologist

*"The fatal error is one universal ontology. A User capsule's world (roles, mandates, preferences, institutions) and a Situation capsule's world (actors, claims, events, leverage) are different worlds. The second fatal error is fully open extraction — schema-free triples are unqueryable mush. The state of the art threads this needle: scope the domain with competency questions, select or synthesize an ontology blueprint, then extract under schema supervision, with a controlled path for the schema to grow when reality doesn't fit it. That is exactly what our Ontology Blueprint + Graph Profile Registry architecture already anticipates — the registry is the interoperability floor, the blueprint is the per-capsule contract, and blueprint evolution is itself a reviewable proposal."*

**The Ontologist's rule (R2 — Blueprint-governed, evolution-by-proposal):** every extraction run is bound to a versioned ontology blueprint chosen or synthesized at build start. Extractors may *propose* new classes/relations when ≥N instances don't fit (dynamic ontology), but a blueprint change is a first-class reviewable proposal like any claim — never a silent mutation.

## Persona 3 — The Elicitor (cognitive-task-analysis practitioner)

*"Experts cannot tell you what they know by being asked 'what do you know?' Fifty years of knowledge-elicitation research says tacit knowledge surfaces through *incidents*, not abstractions: walk me through a specific case; where were the decision points; what cue told you; what would a novice have done; when did your rule fail. That's the Critical Decision Method — multiple passes over a real event with cognitive probes. The engine's interviewer must be built on this, not on generic 'tell me about your workflow' questions. And the interview must know when to stop asking the human and start reading their documents — the cheapest elicitation is extraction from what they already wrote, with dialogue reserved for what documents can't say: purpose, trust, taste, exceptions, and the war stories."*

**The Elicitor's rule (R3 — Documents first, dialogue for the residual):** every protocol stage declares what extraction should attempt *before* a question is asked; the interviewer asks only about (a) gaps extraction couldn't fill, (b) contradictions extraction surfaced, (c) tacit content that never appears in documents. No question the corpus already answers.

## Persona 4 — The Skeptical Reviewer / Auditor

*"I will sign my name on promoted capsules, so: show me the span. Every record — a claim, a preference, a heuristic, a style rule — must carry its source locator, and I must be able to reject it without destroying lineage. I especially distrust three things: model-inferred content dressed as fact (keep the inferred layer visibly separate), content sourced from third parties inside the user's own mailbox (quarantine it), and enthusiasm during interviews — an expert on a roll will state a heuristic far more confidently than their track record supports. The interviewer should *test* elicited rules, not just transcribe them: 'you said X always holds — here's a case from your own documents where it didn't; refine or caveat?'"*

**The Reviewer's rule (R4 — Adversarial pass before the gate):** every capsule build ends with an internal critique step (the AGON pass) that attacks the strongest records — counterexamples to heuristics, contradictions between claims, style rules violated by the user's own exemplars — and files the results as review triggers. Promotion without a completed critique pass is invalid.

## Persona 5 — The Agent Runtime Engineer (the consumer)

*"I integrate capsules into agentic workflows. What I need is boringly practical: a deterministic pack with the Purpose block first; typed records I can filter (give me only promoted claims about Actor X after date T); stable IDs so my agent can cite claim `c-0142` and device `d-007` in its margins; a merged contract that tells my agent what it must and must not do; and a machine-readable staleness/refusal state. What I do *not* need is prose. If capture produces beautiful narratives without IDs, spans, and types, it is useless to me."*

**The Runtime Engineer's rule (R5 — Everything addressable):** every captured record has a stable ID, a type from the blueprint, a review state, temporal validity, and source spans — and remains addressable through compile into the served context. Capture that can't be cited downstream didn't happen.

**Council synthesis.** The five rules compose into the engine's capture creed: *extract before you ask (R3), under a governed blueprint (R2), keeping only what a reviewer could defend (R1), stress-tested before gating (R4), and addressable forever (R5).*

---

# PART II — THE CAPTURE MODEL

## II.1 Six capture channels

Every capsule type draws on the same six channels, weighted differently (Part III gives the per-type mix):

| # | Channel | What it yields | Trust posture |
|---|---|---|---|
| C1 | **Document extraction** — uploads, PDFs, notes | claims, entities, structure, style evidence | source-bound; highest volume |
| C2 | **Connector streams** — Gmail, Drive/shared folders, Calendar, feeds | living evidence, actors, events, commitments, revision history | quarantine-first (third-party content; injection risk); consent-manifest scoped |
| C3 | **Structured dialogue** — the AI interview (protocols) | purpose, trust judgments, tacit heuristics, preferences, exceptions | transcript is a source; every answer span-addressed |
| C4 | **Behavioral capture** — what the user edits, rejects, re-asks in PRAXIS | revealed preferences, implicit style rules, salience priors | inferred layer only until confirmed in dialogue |
| C5 | **Exemplar mining** — past deliverables, "documents you're proud of" | output structure, voice, register, standing positions | strongest signal for User & Output types |
| C6 | **Import & fork** — Exchange capsules, org templates, prior capsules | reasoning devices, ontology blueprints, contracts | lineage-preserved; re-review on fork |

## II.2 The graph: one substrate, six layers

Per capsule (embedded, travels in the package as `graph.jsonld` + the Ladybug projection) and, in the durable service, per tenant (operational store) — the same layered model, aligned with the episodic→semantic→community hierarchy proven in temporal-graph agent memory, extended with DIALECTICA's governance layers:

```
L0  EPISODIC      raw events: document spans, email messages, chat turns,
                  calendar items — the ground-truth corpus; never re-written
L1  CLAIM/RECORD  typed atomic records extracted from L0: claims, preferences,
                  heuristics, style rules, commitments — each with spans,
                  receipt, confidence, review state, bi-temporal validity
L2  ENTITY/       resolved entities + relations under the ontology blueprint:
    SEMANTIC      actors, institutions, interests, events, methods, artifacts;
                  edges carry (valid_from, invalid_at) — supersede, never delete
L3  INFERRED      model-derived links, patterns, salience, community/cluster
                  summaries — visibly non-canonical; promotable via review only
L4  ONTOLOGY      the blueprint itself as graph: classes, relations, constraints,
                  competency questions; blueprint evolution is versioned here
L5  META-         coverage & gap map: which blueprint slots are empty, which
    EPISTEMIC     actors lack interests, which claims lack temporal status,
                  what no source covers — THE FUEL FOR THE INTERVIEWER (R3)
```

Two disciplines hold across all layers: **bi-temporality** (event time vs. ingestion time; contradictions invalidate, never erase) and **provenance closure** (every L1–L3 object resolves to L0 spans). L5 is the engine's self-knowledge: the clarification engine (II.4) reads it to decide what to ask.

## II.3 Dynamic ontology: how blueprints are chosen and how they grow

1. **Scope by competency questions.** At build start, the engine drafts CQs from the declared Purpose and a corpus sample ("Which actors can block implementation?", "What must an executive summary of this type contain?"). CQs are shown to the user for edit/confirmation — they double as the capsule's acceptance tests.
2. **Select or synthesize the blueprint.** Match CQs against the Graph Profile Registry (the interoperability floor). Exact profile → adopt. Partial → adopt + extension slots. None → synthesize a candidate blueprint from CQs (extract classes/properties, then structure the hierarchy) and route it to review as a proposal.
3. **Extract under supervision.** All extraction prompts carry the blueprint (or the relevant ontology snippet per entity type, ODKE+-style) — schema-supervised population, not open mush (R2).
4. **Grow by exception.** Extraction keeps an *unmappable ledger*; when ≥N similar unmappables accumulate, the engine proposes a blueprint amendment (new class/relation with definition, domain/range, examples). Amendments are reviewed like any record; accepted ones version the blueprint; the graph back-fills under the new version with receipts.
5. **Registry feedback.** Amendments recurring across tenants/capsules become candidate registry additions — the mechanism by which the floor itself learns (governance: ADR + human curation, never automatic).

## II.4 The Clarification Engine ("AI chatting with the user") — trigger taxonomy

Dialogue is a scarce resource (user minutes). The interviewer fires only on typed triggers, each traceable to the layer that raised it:

| Trigger | Raised by | Example question |
|---|---|---|
| **T1 Purpose gap** | Purpose block incomplete | "What decision should this capsule support, and what would make it unusable for that?" |
| **T2 Blueprint slot empty** | L5 vs L4 | "Your blueprint tracks *leverage*, but no source mentions what Actor B controls. Do you know?" |
| **T3 Contradiction** | L1/L2 supersession events | "Doc A says the deadline is March; the email thread says June. Which governs, or should I keep both with dates?" |
| **T4 Low-confidence promotion candidate** | extraction confidence bands | "I read this as a commitment by X to Y — is that right, or is it aspirational language?" |
| **T5 Tacit residual** | protocol stage declares undocumentable content | CDM probes: "Walk me through one real case… what cue told you? What would a novice have missed? When did this rule fail?" |
| **T6 Trust & source posture** | new source enters the pack | "How much do you trust this outlet for casualty figures — assert, attribute, or hedge?" |
| **T7 Exception mining** | behavioral capture (C4) contradicts a promoted rule | "You always shorten my executive summaries despite the 200-word rule — should the rule become 120?" |
| **T8 Quarantine adjudication** | C2 quarantined third-party content | "This forwarded memo contains claims I've quarantined — treat as evidence (attributed) or ignore?" |
| **T9 Staleness** | Purpose staleness policy | "This capsule's ceasefire status is 19 days old against a 14-day policy — refresh, re-gate, or extend?" |

Every question cites *why it's being asked* (the trigger + the gap), every answer becomes a span-addressed transcript record, and the engine never re-asks what the corpus or a prior answer already settled (R3).

---

# PART III — THE FOUR CAPSULE TYPES: CAPTURE PLAYBOOKS

Common frame for each: **Essence → Record families → Channel mix → Ontology profile & graph emphasis → Extraction pipeline → Elicitation protocol (stages & probes) → Adversarial pass → Completeness & gate → Failure modes.**
The composition contract downstream is `1 User + 0..n Situation + 0..n Tool + 1 Output` — the four types answer, respectively: **who is asking · what is going on · how to think · what good looks like.**

---

## III.1 USER CAPSULE — *who is asking*

**Essence.** Everything about the person/desk that should condition generation without ever supplying facts about the world: identity & role, mandate & authority, institution and its constraints, team & routing (who reviews, who signs), audiences and their registers, standing positions, values & red lines, working preferences, privacy rules, and the Purpose of the capsule itself. A User capsule biases *tone, stance, framing, and process* — the composition contract forbids it from being a source of situational fact.

**Record families.** `identity_profile`, `role_mandate`, `institution_constraint`, `audience_profile[]`, `team_routing`, `standing_position[]`, `red_line[]`, `preference[]` (format, length, hedging, citation taste), `privacy_rule[]`, `voice_profile` (feeds the language profile), `output_intent`.

**Channel mix.** C5 exemplars (dominant: 2–3 documents the user is proud of) → C1 (CV/bio/role docs) → C3 dialogue (mandate, red lines, audiences — the undocumentables) → C4 behavioral (revealed preferences over time) → C2 light (sent-mail sample, explicit consent, for voice only).

**Ontology profile & graph emphasis.** Person–Role–Institution–Mandate–Audience–Preference profile from the registry; small graph, high edge precision. L2 links user→institution→constraints; audiences as first-class nodes with register attributes. Voice traits live as L1 records with exemplar spans as evidence, distilled into the `language_profile`.

**Extraction pipeline.** Stylometry over exemplars (register, cadence, terminology, hedging ratio, structure habits) → each trait emitted as a *proposal with exemplar spans*, never asserted flatly; role/institution extraction from bio docs; standing-position mining from past analyses ("across 6 documents you consistently argue X — standing position?").

**Elicitation protocol (`user.v2` stages).** 1 *Purpose & decision context* (T1) → 2 *Role, mandate, authority* ("what can you decide alone; what needs sign-off?") → 3 *Audiences & registers* ("who reads this; what makes them stop reading?") → 4 *Voice confirmation* (extracted traits shown as accept/edit/reject cards) → 5 *Red lines & privacy* ("what must never appear under your name; what data may never leave the tenant?") → 6 *Team routing* ("who reviews what, in what order?").

**Adversarial pass.** Test declared preferences against exemplars ("you asked for 'no adjectives in openings' — your best-rated memo opens with two; refine?"); test red lines for collision with standing positions.

**Completeness & gate.** Minimum: Purpose + role/mandate + ≥1 audience with register + voice profile with ≥5 span-backed traits + ≥1 red line. Reviewer = the user themselves (self-gate), but the review event is still recorded.

**Failure modes to design against.** Flattery capture (the interview transcribing self-image rather than practice — behavioral channel C4 is the corrective); over-personalization (preferences leaking into factual judgment — composition firewall); staleness (roles change — role records carry validity windows and T9 fires on org-chart signals).

---

## III.2 SITUATION CAPSULE — *what is going on*

**Essence.** The governed model of a specific situation: actors and their interests/constraints/leverage; claims with contest status; events on a temporal spine; commitments and their compliance; narratives in circulation; source roster with trust postures; reasoning posture for this situation ("attribute, don't assert, on casualty figures"); and the L5 gap map — what is *not* known, made explicit. This is the type where the multi-layer graph earns its keep and where KAIROS diffs produce the flagship "what changed, cited" memo.

**Record families.** `claim[]` (asserted/attributed/hedged/question + contest status), `actor[]`, `interest[]`, `constraint[]`, `leverage[]`, `commitment[]` (+compliance events), `event[]`, `narrative[]`, `source_posture[]` (per-source trust: assert/attribute/hedge), `caveat[]`, `open_question[]`, `situation_reasoning_note[]`.

**Channel mix.** C2 connectors (dominant in steady state: the Drive folder and mail threads where the situation actually lives) + C1 uploads → C3 dialogue for gaps/contradictions/trust (T2/T3/T6) → C6 forked ontology blueprints for the domain → C4 (which claims the user keeps checking = salience).

**Ontology profile & graph emphasis.** Conflict-grammar profile (Actor/Claim/Interest/Constraint/Leverage/Commitment/Event/Narrative) where apt — but per R2 it is *one profile in the registry, not the universal ontology*; an HR dispute, a market entry, and a ceasefire select different blueprints or extensions. Full six-layer stack in play; L2 bi-temporal edges are the substance; L3 holds pattern/community summaries ("these 14 claims cluster into the 'sanctions relief' storyline"); L5 drives the interview.

**Extraction pipeline.** The full understanding chain: normalize (thread-aware for email; revision-aware for Drive) → CQ-scoped blueprint selection → schema-supervised extraction (claims with modality + speaker + date; events with temporal anchors; commitments with parties/terms/deadlines) → entity resolution against the tenant graph (merges are proposals) → temporalization + contradiction detection (invalidate, never erase) → layering → proposals. Connector-sourced third-party content enters quarantined (R4, T8).

**Elicitation protocol (`situation.v2` stages).** 1 *Decision served & misuse conditions* (existing stage, kept) → 2 *Actors & interests* with the missing-actor probe ("who would object to this framing?") → 3 *Contested claims & source posture* (T3/T6: adjudicate contradictions, set per-source trust) → 4 *Temporal spine* ("what's the anchor event; what window matters; what's scheduled?") → 5 *Gap interview* (pure T2 from L5: the engine lists what it couldn't learn) → 6 *Reasoning posture* ("in this situation, what may be asserted vs. only attributed? what inference is off-limits?").

**Adversarial pass.** Contradiction sweep across the whole promoted set; commitment-vs-event compliance check ("X committed to withdraw by May; events show presence in June — flag?"); narrative-vs-claim tension ("the dominant narrative asserts Y; promoted claims only support attributed-Y").

**Completeness & gate.** Minimum: Purpose + ≥1 consented/uploaded source + actor roster with ≥1 interest each for principal actors + temporal spine + contradiction ledger (may be non-empty — *unresolved is a valid, honest state*) + source postures for every source in the pack. Gate blocks if any quarantined span was promoted without T8 adjudication.

**Failure modes.** Silent contradiction resolution (forbidden — surface, never settle); recency capture (connectors flooding L1 with noise — salience priors and Purpose scope_out throttle ingestion); single-narrative capture (the missing-actor probe and narrative records exist precisely to prevent it); ontology lock-in (the unmappable ledger + blueprint amendment path is the release valve).

---

## III.3 TOOL CAPSULE — *how to think* (tacit knowledge made portable, repeatable, auditable)

**Essence.** An intellectual or practical method captured as an executable-by-agents, auditable-by-humans object: stakeholder analysis, assumption-checking, ceasefire-claim assessment, scenario stress-testing, negotiation-position mapping, red-teaming a memo — any "way of doing something with the mind." The point is to make expert reasoning *explicit, repeatable, and attributable*: when an agent later runs the method, each judgment cites the device that shaped it. Tool capsules are the Exchange's seed asset: expertise becomes a portable, authored artifact.

**Record families.** `method_overview` (purpose, when-to-use, when-NOT-to-use), `step[]` (ordered, with required inputs & expected intermediate artifacts), `device[]` (named analytical moves: "invert the stakeholder map", "steelman the weakest actor"), `heuristic[]` (if/then judgment rules with scope conditions), `cue[]` (what an expert perceives that a novice misses — the CDM's central yield), `trap[]` (known failure modes + tells), `precedent[]` (worked cases + what each teaches), `salience_prior[]` (what to examine first), `quality_check[]` (how to know the method was run well), `philosophical_lens[]` (the stance the method assumes), `human_checkpoint[]` (steps that must return to a person — hybrid workflows).

**Channel mix.** C3 dialogue dominant — this is where the Critical Decision Method lives → C5 exemplar mining (the expert's past analyses, mined for *implicit* devices: "in these 4 memos you always check spoiler incentives before capability — is that a rule?") → C6 forked methods refined by the expert → C1 (method literature the expert endorses, as attributed sources).

**Ontology profile & graph emphasis.** Method–Step–Device–Input–FailureMode–Precedent profile. The graph is a *process* graph: steps as ordered nodes; devices attached to steps; traps edged to the steps where they bite; precedents edged to the heuristics they support or refute; human checkpoints as typed gate nodes. L3 may hold cross-tool pattern clusters ("your three assessment tools share a 'check incentives before capability' device — promote to a shared device?").

**Extraction + elicitation pipeline (interleaved by design — the CDM adaptation).**
*Stage 1 — Scope & when-not-to-use:* "What does this method produce, for what decision? Give me a case where using it would be a mistake." (when-NOT-to-use is required — methods without contraindications are advertising, not knowledge).
*Stage 2 — Incident selection:* pick 1–3 *real* cases where the expert ran the method under stakes. Not hypotheticals — CDM's power comes from real-event retrospection.
*Stage 3 — Sweep 1, timeline:* reconstruct the case chronologically; the engine builds the step skeleton from the narrative.
*Stage 4 — Sweep 2, decision-point deepening:* at each decision point, cognitive probes — "What cue told you? What were you weighing? What would a novice have done here? What information did you *not* need?" → cues, heuristics, salience priors.
*Stage 5 — Sweep 3, counterfactual pass:* "What would have changed your call? When has this rule failed you?" → traps, scope conditions, precedents-of-failure.
*Stage 6 — Generalization & exemplar cross-check:* the engine drafts the method (steps/devices/heuristics) and immediately tests it against the expert's own past documents (C5): "your 2024 memo violates step 3 as stated — was that an exception, or is the step wrong?"
*Stage 7 — Checkpoints & handoff contract:* which steps an agent may run autonomously, which require a human, what the method's outputs must look like.
A **completeness meter** ("2 cases · 12 devices · 6 cues · 4 traps · 3 precedents · 2 checkpoints") gamifies depth across resumable sessions.

**Adversarial pass (strongest of the four types).** The engine attacks the elicited method: generates counterexample scenarios per heuristic; hunts contradictions between devices; checks every heuristic has ≥1 supporting precedent and flags those with none as "untested — caveat required." Expert responses to attacks are themselves captured as refinements (the attack transcript is a source).

**Completeness & gate.** Minimum: Purpose + when-not-to-use + ≥1 real precedent + ≥5 devices/heuristics *each with transcript/exemplar spans* + ≥1 trap + quality checks + explicit human-checkpoint map. Attribution metadata (author, lineage) is mandatory — Tool capsules are authored works.

**Failure modes.** Textbook capture (the expert recites the published method rather than their practice — incident anchoring in Stage 2 is the antidote); over-confident heuristics (the adversarial pass + precedent requirement); rigidity (scope conditions on every heuristic; the method records when it does *not* apply); anonymized expertise (attribution is structural, not optional — it's also the Exchange's incentive engine).

---

## III.4 OUTPUT CAPSULE — *what good looks like*

**Essence.** Everything about a deliverable's form, so an agent can produce — and a validator can check — a document that would pass the user's own quality bar: document type and its anatomy (what an executive summary *is* here, how it opens, how long); section logic and ordering; style/register/terminology; citation & sourcing style; hedging and attribution rules; formatting; length norms; audience adaptation; must/never rules; refusal conditions; and worked exemplars with annotations. The Output capsule doubles as the **validation contract** run against generated drafts — it is how "good" becomes checkable.

**Record families.** `artifact_type`, `section_schema[]` (name, purpose, order, length band, opening move), `style_rule[]`, `terminology_rule[]` (use/avoid, with definitions), `citation_style`, `hedging_rule[]` (what may be asserted vs. attributed *in the output*), `formatting_rule[]`, `length_norm[]`, `audience_adaptation[]`, `must_rule[]`, `never_rule[]`, `refusal_condition[]` (e.g., "refuse to draft if attached Situation capsule staleness > policy"), `exemplar[]` (span-annotated), `anti_exemplar[]` (what bad looks like, and why — often more instructive).

**Channel mix.** C5 exemplars dominant (2–3 best instances of the deliverable; ideally 1 anti-exemplar) → C1 (style guides, templates, institutional drafting rules) → C3 dialogue (must/never confirmation, refusal conditions, the "why" behind rules) → C4 behavioral (edits the user repeatedly makes to agent drafts are candidate rules — T7).

**Ontology profile & graph emphasis.** Artifact–Section–Rule–Exemplar profile. Graph links each rule to the exemplar spans that evidence it and each section to the rules that govern it; refusal conditions edge *outward* to Purpose/staleness fields of companion capsules — the graph is what makes the contract composable at compile time (strictest-wins merging needs typed, addressable rules, R5).

**Extraction pipeline.** Structure induction over exemplars (section segmentation, ordering, length distributions, opening/closing moves) → style induction (register, sentence statistics, hedging ratios, citation pattern) → each induced rule emitted as a proposal *with exemplar spans as evidence* → divergence report where exemplars disagree with each other ("your two SITREPs open differently — which is canonical, or does it depend on audience?" → audience_adaptation record).

**Elicitation protocol (`output.v2` stages).** 1 *Purpose & consumer* ("who acts on this document, within what time budget?") → 2 *Structure confirmation* (induced anatomy as cards) → 3 *Must/never* ("what makes this deliverable fail instantly?") → 4 *Terminology & hedging* ("words your institution never uses; what may this document assert flat-out?") → 5 *Refusal conditions* (T9-linked) → 6 *Anti-exemplar walk* ("show me a bad one; tell me the first thing wrong with it").

**Adversarial pass.** Run the induced contract against the exemplars themselves — every exemplar should pass its own contract; violations mean either the rule is wrong or the exemplar is non-canonical (user adjudicates). Then run it against one deliberately mutated draft to verify the contract *fails* things (a contract that passes everything is vacuous).

**Completeness & gate.** Minimum: Purpose + artifact anatomy with ordered sections + ≥1 must + ≥1 never + citation style + ≥1 span-annotated exemplar + refusal conditions referencing companion-capsule staleness. Gate blocks if the contract cannot fail the mutated-draft test.

**Failure modes.** Rule proliferation (200 micro-rules no agent can satisfy — cap active rules; prefer exemplar-weighting over rule-listing); frozen taste (T7 keeps contracts living); contract/exemplar drift (the self-pass adversarial check is rerun on every recompile).

---

# PART IV — CROSS-CUTTING DOCTRINE

**IV.1 Capture ends at the gate.** All four playbooks terminate identically: proposals → review triggers → human decisions (accept / edit / reject / accept-with-caveat) → promotion → deterministic compile → signed `.capsule` with Ladybug projection. The playbooks vary *what* is captured and *how it is asked for*; the honesty machinery never varies.

**IV.2 The transcript is always a source.** Every interview turn, every adversarial exchange, every adjudication is span-addressed evidence. This is what makes elicited knowledge as auditable as extracted knowledge — the Reviewer can trace a heuristic to the minute the expert said it, and the caveat to the counterexample that forced it.

**IV.3 Purpose is the keystone.** Captured first (stage 1 of every protocol), stored on the manifest, rendered first in every served context, and enforced at the gate (staleness/refusal). A capsule that cannot say what decision it serves cannot be promoted.

**IV.4 Composition is the payoff.** `1 User + n Situation + n Tool + 1 Output` compiles into one deterministic, inspectable context: *who is asking* conditions voice and red lines; *what is going on* supplies cited substance; *how to think* supplies attributable method; *what good looks like* supplies the checkable contract. TACITUS Augmented Generation is exactly this compile, served.

**IV.5 What each persona checks at release.** Analyst: "would I sign it?" · Ontologist: "did the blueprint govern, and evolve only by proposal?" · Elicitor: "did we ask only the residual?" · Reviewer: "can I trace and reject everything?" · Runtime engineer: "can my agent address, filter, and cite everything?" Ship nothing that fails any of the five.

*End of Document 1. Engineering translation → `DIALECTICA_BUILD_PROGRAM.md`.*

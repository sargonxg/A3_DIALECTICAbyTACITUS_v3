# Local native Cognitive Capsule export

`cognitive-export` builds a draft-only `cognitive-capsule/vNext` native artifact from explicit semantic proposals and a DIALECTICA source pack. It uses the PRAXIS shared runtime for complete schema, reference and archive validation, preventing two incompatible native formats. Existing spec-3.x bundles, review promotion and Ladybug requirements remain on their existing paths.

```powershell
cargo run --locked -p dialectica-cli -- cognitive-export fixtures/cognitive-vnext/draft_request.json --source-pack fixtures/cognitive-vnext/source_pack.json --praxis-root C:\Users\giuli\tacitus-products\praxis --out $env:TEMP\synthetic-diplomacy.capsule --allow-write
```

The output must be a new `.capsule` file. `--praxis-root` must point to an explicit installed checkout containing `cli/praxis.ts` and `node_modules/tsx/dist/cli.mjs`; Node must be on PATH. The command never installs tools, requests a provider, contacts a registry, stores credentials or writes PRAXIS canonical state. It uses a private temporary draft file and deletes that owned file afterward.

Input contract `dialectica-cognitive-draft/1` requires `capsuleId`, exact semantic `version`, a Cognitive `profile`, and optional `nodes`, `statements`, `knowledge` and exact `dependencies`. Semantic records must supply real provenance and explicit `sourceRefs`, including an empty list for undocumented expert recollection. Records can only be proposed; review decisions and reviewer stamps are rejected. Supply typed structures such as conditions, exceptions, distinctions, language rules, methods and cases explicitly. The exporter does not convert unstructured expert text into invented conditions or provenance.

Source spans must resolve to source documents. Source content versions must be SHA-256 digests; quote digests are recomputed and verified. The exporter preserves original quoted text and binds each span to its document's content version. A source pack's declaration of a document checksum is retained; this command does not reread the original document bytes to verify that separate checksum. Original documents are excluded and artifact rights remain private. Review and publication require the existing PRAXIS/DIALECTICA paths.

The native archive includes manifest, canonical JSON-LD graph, typed expertise/language/method/case records, evidence, governance, runtime contract and structural evaluation posture. Empty layers are valid. Structural integrity does not establish factual truth, author trust, review quality or model improvement.

```powershell
cargo test --locked -p dialectica-compiler --test cognitive_draft
$env:PRAXIS_ROOT = 'C:\Users\giuli\tacitus-products\praxis'
cargo test --locked -p dialectica-compiler --test cognitive_draft native_bridge_roundtrips_with_shared_praxis_schema -- --ignored
```

The second command runs the real cross-repository artifact test. It is opt-in so ordinary Rust tests do not silently depend on another checkout. No additional dependency or hosted service is required. See [ADR-013](decisions/ADR-013-cognitive-draft-native-package-bridge.md).

# Source Pack

Fixture input records for the golden policy capsule builder path.

`source_pack.json` models the pre-ingestion output that DIALECTICA will later
produce from uploaded documents, PDFs, links, notes, and user/assistant
discussion turns. It is intentionally synthetic, source-bound, hash-shaped, and
safe to commit.

Validate it with:

```powershell
cargo run -p dialectica-cli -- source-pack-check fixtures/golden-policy-capsule/source-pack/source_pack.json
```

The legacy `expected-bundle/` directory remains the migration fixture until the
compiler can regenerate a canonical v3 package from this source pack, proposal
records, and reviewer decisions.

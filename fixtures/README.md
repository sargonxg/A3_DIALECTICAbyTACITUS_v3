# Fixtures

Deterministic capsule fixtures, source packs, and eval inputs will live here.

Expected first fixture:

- one stakeholder-analysis policy source pack;
- one expected source ledger;
- one expected capsule bundle;
- one review ledger with at least one correction;
- one eval baseline comparing raw and capsule-augmented PRAXIS output.

Fixtures must not contain secrets or private user documents.

Current examples:

- `example-capsules/user-capsule.example.json`
- `example-capsules/situation-capsule.example.json`
- `example-capsules/thinking-device-capsule.example.json`
- `example-capsules/output-capsule.example.json`

These are single-file projections of the bundle layers. They exist to make the
shape inspectable before the full golden policy fixture is implemented.

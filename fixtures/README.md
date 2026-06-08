# Fixtures

Deterministic capsule fixtures, source packs, and eval inputs live here.

Current golden fixture:

- `golden-policy-capsule/expected-bundle/`

This is the first executable bundle directory. It includes source ledger,
temporal ledger, ontology, embedded graph, reasoning playbook, language profile,
agent guidance, review ledger, rights profile, marketplace listing, health, and
eval report placeholders.

Fixtures must not contain secrets or private user documents.

Current examples:

- `example-capsules/user-capsule.example.json`
- `example-capsules/situation-capsule.example.json`
- `example-capsules/tool-capsule.example.json`
- `example-capsules/output-capsule.example.json`

These are single-file projections of the bundle layers. They exist to make the
shape inspectable alongside the full golden policy fixture.

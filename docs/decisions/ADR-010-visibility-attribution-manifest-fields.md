# ADR-010: Add Visibility And Attribution Fields To Capsule Manifests

## Status
Proposed

## Date
2026-06-10

## Context
PRAXIS attaches multiple capsules into a working context, and the future Capsule
Exchange publishes reviewed expert reasoning. DIALECTICA therefore needs stable
contracts for sharing scope, ownership, lineage, and reasoning attribution.

The current v3 schemas include marketplace listing and signature placeholders,
but key fields are still too loose for cross-repo integration:

- `visibility` is an unconstrained string on marketplace listings;
- manifest signature fields are fixture-oriented;
- author and publisher identity are not separated;
- fork lineage is not a first-class manifest/listing contract;
- PRAXIS context packs do not yet carry device-level attribution for Tool
  capsule reasoning.

PRAXIS Firestore remains canonical for user-facing capsule library state and
cockpit UX state. DIALECTICA should validate and preserve portable capsule
visibility and attribution metadata, not become the PRAXIS UI library store.

## Decision
Extend the v3 manifest, marketplace listing, and PRAXIS context-pack contracts
with explicit visibility and attribution fields.

The minimum stable field families are:

- `visibility`: enum `private`, `org`, or `public`;
- `org_id`: optional organization identifier required when visibility is `org`;
- `share_grants`: zero or more explicit grants for private/org sharing;
- `author_identity`: the person or organization whose reasoning/content is
  represented;
- `publisher_identity`: the person or organization that signs or distributes
  the bundle;
- `lineage`: parent capsule ids, forked-from digest, and derivation notes;
- `reasoning_attribution`: stable device/heuristic ids and author references
  that travel into compiled context packs.

The signature envelope should distinguish author identity from publisher
identity. A capsule author and an org publisher may be different entities, and
both must remain attributable.

## Alternatives Considered

### Keep visibility only in PRAXIS Firestore
Rejected. PRAXIS can own user-facing library state, but the portable capsule
artifact still needs enough visibility and rights metadata to be validated,
shared, imported, and published safely.

### Put all sharing policy in `rights_profile`
Rejected. Rights policy and visibility overlap but are not identical. Rights
describe allowed uses and duties; visibility describes audience and grants.

### Defer attribution until Exchange implementation
Rejected. Composition and generation need device-level attribution before the
Exchange UI exists, otherwise PRAXIS cannot cite whose expert reasoning shaped
an answer.

## Consequences
Positive:

- PRAXIS Context Inspector can show what each capsule contributed.
- Generated outputs can cite not only sources but the Tool-capsule devices that
  shaped a judgment.
- Exchange publication can validate author, publisher, lineage, rights,
  freshness, caveats, and review level before listing.

Negative:

- Existing fixtures and schema snapshots will need compatibility updates.
- Manifest validation becomes stricter, so migration notes are required.
- Store-backed API routes must preserve PRAXIS as the user-facing visibility
  mirror, not duplicate its library state.

## Acceptance Criteria
- v3 schemas constrain `visibility` and require valid identity/lineage shapes.
- Canonical fixtures include author, publisher, lineage, and visibility fields.
- PRAXIS context packs include reasoning attribution ids for Tool devices.
- Composition output carries per-capsule contribution and attribution maps.
- Marketplace listing validation rejects missing review, rights, lineage,
  caveat, freshness, author, or publisher data with actionable findings.
- No capsule is silently published or promoted without review state and
  signature/attribution checks.

# ADR-011: Use A Separate Signed Integrity Envelope

## Status
Accepted

## Date
2026-06-10

## Context
DIALECTICA capsules need deterministic verification before store-backed jobs,
hosted MCP, Exchange publication, or PRAXIS production import can treat a
portable package as trustworthy. The existing v3 manifest has
`provenance_root_hash` and `signature` fields, but the compiler still writes
fixture-oriented placeholders and package validation only warns when integrity
is incomplete.

The integrity design must avoid a circular digest problem: if the manifest
contains the final root and signature, and the root also hashes the manifest
bytes, then updating the manifest changes the root being signed.

## Decision
Add a separate `integrity/envelope.json` file owned by `dialectica-compiler`.

The envelope contains:

- deterministic leaf digests for every signed package file;
- an explicit canonical-file scope;
- a Merkle root over those leaf digests;
- author and publisher identity records;
- Ed25519 public keys and signatures over a stable payload containing the
  capsule id, scope, and Merkle root;
- DSSE/Sigstore-compatible reserved fields for future promotion.

The v1 scope excludes only the envelope itself and rebuildable projection
outputs:

- `integrity/envelope.json`;
- `graph/ladybug/*`.

All other package files, including `manifest.json`, are signed by digest. The
manifest's existing integrity fields remain compatibility metadata until a
future manifest revision resolves their exact relationship to the external
envelope.

## Alternatives Considered

### Put The Final Signature In `manifest.json`
Rejected for v1 because it requires manifest canonicalization rules before the
local verifier can ship safely.

### Sign Only The Archive Digest
Rejected because DIALECTICA also needs verification of open package
directories before archive assembly and PRAXIS context-pack export.

### Defer Ed25519 Until Cloud Key Management Exists
Rejected because the local Demo Gate needs a real tamper-detection story now.
Cloud key management can replace the local fixture key later without changing
the envelope shape.

## Consequences
Positive:

- `dialectica verify <compiled-dir>` can catch one-byte tampering locally.
- Open package directories and `.capsule` archives can share the same integrity
  model later.
- Author and publisher attribution are represented without waiting for PRAXIS
  or Secret Manager.

Negative:

- The manifest still carries legacy placeholder integrity fields until a future
  schema revision reconciles them with the envelope.
- Fixture signing uses a deterministic local key and must not be represented as
  production trust.

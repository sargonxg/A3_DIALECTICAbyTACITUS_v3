# ADR-006: Use Apache-2.0 With TACITUS Attribution And Citation Metadata

## Status

Accepted

## Date

2026-06-07

## Context

DIALECTICA is moving from a proprietary repository posture to an open-source
repository posture. The project needs an open software license that lets
engineers inspect, fork, build, and integrate the capsule engine while
preserving TACITUS attribution. The repository also needs a clear citation path
for researchers, analysts, benchmark authors, public demos, and derivative
capsule-engine work.

The license must be familiar to engineers, recognized by GitHub and package
ecosystems, compatible with commercial and research use, and appropriate for a
Rust service stack.

## Decision

Use the Apache License, Version 2.0 for the repository.

Add and maintain:

- `LICENSE` with Apache-2.0 text;
- `NOTICE` with TACITUS attribution and citation request;
- `CITATION.cff` with machine-readable citation metadata;
- Cargo workspace metadata `license = "Apache-2.0"`;
- README, support, security, contribution, and GitHub profile copy aligned with
  the open-source posture.

## Alternatives Considered

### MIT

- Pros: very short, familiar, permissive.
- Cons: weaker explicit patent posture and no dedicated NOTICE mechanism.
- Rejected: Apache-2.0 better fits a capsule engine that may become a platform
  component and needs preserved attribution notices.

### BSD-3-Clause

- Pros: permissive and common.
- Cons: less explicit patent posture than Apache-2.0 and no NOTICE mechanism.
- Rejected: Apache-2.0 is stronger for this project.

### MPL-2.0

- Pros: file-level copyleft can preserve openness of modified source files.
- Cons: more restrictive and less aligned with broad platform adoption.
- Rejected: the current goal is adoption, inspection, and ecosystem trust with
  attribution preserved.

### Custom Citation License

- Pros: could attempt to require citation in every use.
- Cons: would be nonstandard, harder for engineers and companies to approve,
  and may not be considered open source.
- Rejected: use a standard open-source license plus `NOTICE` and
  `CITATION.cff` instead.

## Consequences

- DIALECTICA is open source under Apache-2.0.
- Redistributors must preserve license and required notices under the license.
- Public academic/product citation is requested and made easy through
  `CITATION.cff`, but the license itself remains standard Apache-2.0.
- Future dependency and contribution checks should use SPDX identifier
  `Apache-2.0`.
- Trademarks, product names, and company names remain owned by their respective
  owners; Apache-2.0 does not grant trademark rights beyond customary origin
  descriptions and reproduction of the NOTICE file.

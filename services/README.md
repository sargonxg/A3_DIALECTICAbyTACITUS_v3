# Services

Deployable service crates and container entrypoints will live here.

Active services:

- `dialectica-api`: HTTP API for PRAXIS and operators;
- `dialectica-task-handler`: Cloud Tasks HTTP worker;

Current state: both service crates are scaffolds. They compile, but they do not
open HTTP ports or serve routes yet. Build `dialectica-api` only after the local
v3 compiler and context-pack export work.

`dialectica-worker` remains optional and should only be added when pull-based
background processing is justified by implementation evidence. Cloud Run is the
first deployment target.

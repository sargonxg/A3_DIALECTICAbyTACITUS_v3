# Services

Deployable service crates and container entrypoints will live here.

Active services:

- `dialectica-api`: HTTP API for PRAXIS and operators;
- `dialectica-task-handler`: Cloud Tasks HTTP worker;

`dialectica-worker` remains optional and should only be added when pull-based
background processing is justified by implementation evidence. Cloud Run is the
first deployment target.

# PRAXIS Integration Contract

## Goal

PRAXIS should be able to request, inspect, combine, and use PRAXIS Capsules
without exposing DIALECTICA complexity to users.

The user should see a simple capsule library and stronger PRAXIS answers. The
engine can stay hidden.

## Integration Principles

- PRAXIS is the user-facing cockpit.
- DIALECTICA owns capsule compilation and canonical capsule artifacts.
- PRAXIS may mirror capsule metadata for UI speed.
- PRAXIS must not treat a capsule as trusted unless status and review gates pass.
- PRAXIS should expose source and review receipts when a capsule affects output.

## Minimal API Surface

### Create Capsule Job

```http
POST /v1/capsule-jobs
```

Use when PRAXIS starts a new capsule build.

Inputs:

- tenant;
- project;
- user;
- source references;
- target capsule type;
- intended workflows;
- review requirements.

Output:

- `job_id`
- `status`
- `next_poll_after`
- `links`

### Get Job

```http
GET /v1/capsule-jobs/{job_id}
```

Output:

- job status;
- current phase;
- errors;
- created records;
- review requirements;
- capsule id if compiled.

### Get Capsule Manifest

```http
GET /v1/capsules/{capsule_id}/manifest
```

Output:

- manifest fields;
- review state;
- freshness;
- source count;
- digest;
- compatible PRAXIS workflows.

### Get PRAXIS Context Pack

```http
GET /v1/capsules/{capsule_id}/praxis-context-pack
```

Output:

- compact summary;
- selected retrieval records;
- source hints;
- reasoning playbook subset;
- output contracts;
- warnings.

### Combine Capsules

```http
POST /v1/capsule-sets
```

Use when PRAXIS wants to concatenate multiple capsules into one workflow.

The response must include:

- compatibility warnings;
- conflicting claims;
- freshness warnings;
- combined retrieval plan;
- combined reasoning playbook summary.

## PRAXIS UI Requirements

PRAXIS should surface:

- capsule title and status;
- freshness and last compile time;
- source count and top source types;
- human review state;
- warnings for stale or contested context;
- source receipts in answer views;
- capsule contribution in agent run receipts.

PRAXIS should avoid exposing:

- internal DIALECTICA service names;
- raw extraction internals;
- graph engine implementation details;
- model provider internals unless needed for audit.

## Agent Workflow Requirements

When PRAXIS uses a capsule, the agent should receive:

- the capsule manifest;
- task-specific capsule summary;
- retrieval pack snippets;
- source and citation hints;
- temporal warnings;
- reasoning devices relevant to the requested output;
- forbidden claims and escalation criteria;
- output contract.

The agent should return:

- answer or artifact;
- capsule ids used;
- source ids cited;
- unresolved uncertainties;
- capsule warnings triggered;
- follow-up source gaps.

## Compatibility With Existing PRAXIS Direction

This integration is designed to keep PRAXIS simple:

- no new top-level product surface is required for the MVP;
- capsule status can appear inside existing Ask, workbench, or library surfaces;
- deeper graph and ontology inspection can remain behind expert workflows;
- runtime proof should be truthful and based on actual capsule receipts.

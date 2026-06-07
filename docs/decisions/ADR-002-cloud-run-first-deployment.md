# ADR-002: Cloud Run First Deployment

Status: accepted

Date: 2026-06-07

## Context

DIALECTICA v3 needs APIs, background workers, jobs, durable dispatch, PostgreSQL,
object storage, secrets, and observability. The team considered whether to start
with Kubernetes or a managed serverless container platform.

## Decision

Start with Cloud Run services, Cloud Run jobs, Cloud Tasks, Cloud SQL
PostgreSQL, Cloud Storage, Secret Manager, Artifact Registry, and CI/CD.

Do not start with Kubernetes.

Use GKE Autopilot later only if Cloud Run cannot support validated production
requirements.

## Evidence

Google Cloud documents Cloud Run as managed container execution with scaling and
integrations with Cloud SQL, Firestore, Cloud Storage, logging, and monitoring.
Google Cloud also documents Cloud Run worker pools for non-HTTP pull-based
processing.

Cloud Tasks can invoke HTTP handlers on Cloud Run, GKE, Compute Engine, or
external endpoints with retry and scheduling controls.

Google Cloud documents Cloud Run and GKE as portable container runtimes, which
keeps future migration feasible.

Source anchors:

- <https://docs.cloud.google.com/run/docs/overview/what-is-cloud-run>
- <https://docs.cloud.google.com/tasks/docs/creating-http-target-tasks>
- <https://docs.cloud.google.com/kubernetes-engine/docs/concepts/gke-and-cloud-run>

## Consequences

Positive:

- lower operational overhead;
- faster MVP deployment;
- easier PRAXIS integration;
- reversible container strategy;
- built-in Google Cloud observability.

Negative:

- less control than Kubernetes;
- worker pools require manual scaling unless custom autoscaling is built;
- some long-running or hardware-specific workloads may need GKE later.

## Promotion Criteria For GKE

Move selected workloads to GKE Autopilot if:

- Cloud Run limits block validated workloads;
- graph or semantic services require long-running clustered state;
- custom autoscaling and topology become material;
- GPU or hardware-specific scheduling is required;
- network isolation requirements exceed Cloud Run's practical fit.

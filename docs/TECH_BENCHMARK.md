# Tech Benchmark

Date: 2026-06-07

This document records the final pre-build research pass. The conclusion is not
that DIALECTICA should copy any single memory or RAG product. The conclusion is
that DIALECTICA should use the best ideas from the ecosystem while staying a
policy-grade capsule compiler for PRAXIS.

## Benchmark Summary

| Project or pattern | What it proves | What DIALECTICA should do |
| --- | --- | --- |
| Graphiti / Zep | Temporal context graphs with provenance are highly relevant for agents operating on changing facts. | Adopt temporal/provenance semantics, but keep Postgres and bundle export canonical. |
| Cognee | Agent memory can become a shared graph memory layer with entities, relations, ontologies, permissions, MCP, and agent integrations. | Build a memory/ontology adapter interface, but keep capsule bundles and Postgres canonical. |
| Mem0 | Multi-level user/session/agent memory and production memory evals matter. | Treat memory as scoped capsule evidence with review state, not automatic personalization. |
| Khoj | Personal "second brain" systems emphasize user-owned docs, local/cloud access, and chat over personal knowledge. | Learn from the UX loop, but make DIALECTICA team/policy-grade with review, provenance, and portable capsules. |
| Letta MemFS | Git-backed context repositories and reflection agents make memory inspectable and versioned. | Use versioned capsule bundles, review ledgers, and build agents that can propose doc updates. |
| Microsoft GraphRAG | Graph extraction over private text can improve synthesis, but indexing can be costly and prompt tuning matters. | Start with small deterministic graph slices and evals before large indexing. |
| LangGraph | Long-running agents need persistence, human-in-the-loop, streaming, and memory. | PRAXIS owns visible agent runs; DIALECTICA supplies durable capsule context and receipts. |
| OpenAI Agents SDK | Specialist agents, tools, guardrails, state, tracing, and evals are now standard building blocks. | Keep agent boundaries explicit and make capsule validation/human review guardrails. |
| Model Context Protocol | Resources/tools are becoming a standard way to expose context to agents. | Expose capsule resources later through MCP, with strict read-only defaults and provenance. |

## Source Notes

- Graphiti describes temporal context graphs that track changing facts, source
  provenance, prescribed and learned ontology, incremental updates, and hybrid
  retrieval.
- Cognee positions itself as an open-source agent memory platform that turns raw
  data into graph memory with entities, relations, ontologies, permissions,
  feedback, MCP, and agent integrations.
- Mem0 emphasizes user/session/agent memory, managed/self-hosted modes, hybrid
  search, and memory evaluations.
- Khoj is an open-source AI second brain for web/docs/local notes and custom
  agents across personal and enterprise modes.
- Letta Code uses git-backed memory files, direct memory edits, commits, and
  reflection subagents.
- Microsoft GraphRAG is a graph-based RAG pipeline for extracting structured
  data from unstructured text, with explicit warnings about indexing cost,
  prompt tuning, versioning, and responsible AI.
- LangGraph is a low-level orchestration runtime for long-running stateful
  agents with persistence, human-in-the-loop, memory, and tracing.
- The OpenAI Agents SDK is appropriate when an application owns orchestration,
  tool execution, approvals, custom storage, and state.
- MCP resources provide URI-addressed context objects that clients can read and
  use in model interactions.

## Design Consequences

### 1. Capsule Before Memory

Memory alone is too loose for policy work. DIALECTICA must first build a
capsule with source, time, review, and output contracts. Memory is a capsule
component, not the whole product.

### 2. Temporal Graph Semantics Without Graph Lock-In

Graphiti's temporal validity windows and provenance are exactly right for
policy analysis. The MVP should encode those semantics in Postgres and bundle
exports before adopting a dedicated temporal graph runtime.

### 3. Human Review As The Differentiator

Generic memory layers usually optimize recall. DIALECTICA should optimize
trusted policy use. Review decisions, caveats, and promotion gates should be
first-class records.

### 4. Portable Context As The Product

Letta's git-backed memory and GraphRAG's repo hygiene show that inspectability
matters. DIALECTICA should produce portable capsule bundles with checksums,
signatures, and ledgers.

### 5. Evals Before Adapters

Graph, vector, and memory adapters should be promoted only if they improve
source fidelity, temporal accuracy, reasoning transfer, or PRAXIS outcome evals.

## MVP Recommendation

Build DIALECTICA v3 as:

```text
Rust service + CLI
Cloud Run API and task handler
Cloud SQL PostgreSQL canonical records
Cloud Storage signed bundle exports
Cloud Tasks durable dispatch
PRAXIS context-pack API
Optional adapter interfaces for graph, vector, MCP, and memory
```

Do not start with:

- required Graphiti;
- required Cognee;
- required LangGraph;
- required Kubernetes;
- required standalone vector database;
- autonomous memory promotion.

## Source Anchors

- Graphiti: <https://github.com/getzep/graphiti>
- Zep Graphiti docs: <https://help.getzep.com/graphiti/getting-started/welcome>
- Cognee: <https://www.cognee.ai/>
- Mem0: <https://github.com/mem0ai/mem0>
- Khoj: <https://github.com/khoj-ai/khoj>
- Khoj docs: <https://docs.khoj.dev/>
- Letta memory: <https://docs.letta.com/letta-code/memory/>
- Microsoft GraphRAG: <https://github.com/microsoft/graphrag>
- LangGraph: <https://docs.langchain.com/langgraph>
- OpenAI Agents SDK: <https://platform.openai.com/docs/guides/agents-sdk/>
- Model Context Protocol: <https://github.com/modelcontextprotocol/modelcontextprotocol>
- MCP resources: <https://modelcontextprotocol.info/docs/concepts/resources/>

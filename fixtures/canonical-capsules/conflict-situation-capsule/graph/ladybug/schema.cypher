CREATE NODE TABLE CapsuleNode(id STRING, node_type STRING, label STRING, review_state STRING, source_span_ids_json STRING, properties_json STRING, PRIMARY KEY(id));
CREATE REL TABLE CapsuleEdge(FROM CapsuleNode TO CapsuleNode, id STRING, edge_type STRING, confidence DOUBLE, temporal_scope_json STRING, source_span_ids_json STRING, review_state STRING, explanation STRING);

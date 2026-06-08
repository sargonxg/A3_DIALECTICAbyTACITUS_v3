MATCH (n:CapsuleNode) RETURN count(n) AS node_count;
MATCH ()-[e:CapsuleEdge]->() RETURN count(e) AS edge_count;
MATCH (a:CapsuleNode)-[e:CapsuleEdge]->(b:CapsuleNode) RETURN a.id AS from_node_id, e.edge_type AS edge_type, b.id AS to_node_id LIMIT 50;

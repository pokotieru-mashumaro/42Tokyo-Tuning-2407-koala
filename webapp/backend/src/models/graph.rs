use sqlx::FromRow;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(FromRow, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(FromRow, Clone, Debug)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}


#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>,
    pub edges: HashMap<i32, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges
            .entry(edge.node_a_id)
            .or_default()
            .push(edge.clone());

        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        self.edges
            .entry(reverse_edge.node_a_id)
            .or_default()
            .push(reverse_edge);
    }

    pub fn shortest_path(&self, from_node_id: i32, to_node_id: i32) -> i32 {
        if !self.nodes.contains_key(&from_node_id) || !self.nodes.contains_key(&to_node_id) {
            return i32::MAX;
        }
    
        let node_ids: Vec<i32> = self.nodes.keys().cloned().collect();
        let id_to_index: HashMap<i32, usize> = node_ids.iter().enumerate().map(|(i, &id)| (id, i)).collect();
    
        let mut d = vec![i32::MAX; self.nodes.len()];
        let mut bh = BinaryHeap::new();
        let mut color: Vec<isize> = vec![-1; self.nodes.len()];
    
        let from_index = *id_to_index.get(&from_node_id).unwrap();
        d[from_index] = 0;
        bh.push((0, from_node_id));
        color[from_index] = 0;
    
        while let Some((dist, u)) = bh.pop() {
            let dist = -dist;
    
            if u == to_node_id {
                return dist;
            }
    
            let u_index = *id_to_index.get(&u).unwrap();
            if d[u_index] < dist {
                continue;
            }
    
            color[u_index] = 1;
    
            if let Some(edges) = self.edges.get(&u) {
                for edge in edges {
                    let v = edge.node_b_id;
                    let v_index = *id_to_index.get(&v).unwrap();
                    if color[v_index] != 1 {
                        let next_dist = dist.saturating_add(edge.weight);
                        if next_dist < d[v_index] {
                            d[v_index] = next_dist;
                            bh.push((-next_dist, v));
                            color[v_index] = 0;
                        }
                    }
                }
            }
        }
    
        i32::MAX
    }
}

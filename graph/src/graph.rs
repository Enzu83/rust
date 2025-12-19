use std::{collections::{HashMap, HashSet}, hash::Hash};

pub struct Graph<T> {
    nodes: HashSet<T>,
    edges: HashMap<T, HashSet<T>>,
}

impl<T> Graph<T>
where
    T: Clone + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    pub fn from_nodes(nodes: HashSet<T>) -> Self {
        let edges = nodes
            .clone()
            .into_iter()
            .map(|n| (n, HashSet::new()))
            .collect();
        
        Self { nodes, edges }
    }

    pub fn from_edges(edges: HashMap<T, HashSet<T>>) -> Self {
        let nodes = edges
            .clone()
            .into_keys()
            .collect();

        Self { nodes, edges }
    }

    pub fn from_nodes_and_edges(nodes: HashSet<T>, edges: HashMap<T, HashSet<T>>) -> Self {
        Self { nodes, edges }
    }

    pub fn has_node(&self, node: &T) -> bool {
        self.nodes.contains(node)
    }

    pub fn has_edge(&self, source: &T, target: &T) -> bool {
        match self.edges.get(source) {
            Some(neighbors) => neighbors.contains(target),
            None => false,
        }
    }

    pub fn add_node(&mut self, node: T) {
        self.nodes.insert(node);
    }

    pub fn add_edge(&mut self, source: &T, target: T) {
        match self.edges.get_mut(source) {
            Some(neighbors) => { neighbors.insert(target); },
            None => { self.edges.insert(source.clone(), HashSet::new()); },
        }
    }

    pub fn get_neighbors(&self, node: &T) -> Option<&HashSet<T>> {
        self.edges.get(node)
    }
}
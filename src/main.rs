use std::collections::HashMap;
use std::collections::HashSet;

struct Graph {
    graph: HashMap<String, Vec<String>>,
    visited: HashSet<String>
}

impl Graph {
    fn new() -> Self {
        Self { 
            graph: HashMap::new(),
            visited: HashSet::new()
        }
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        self.graph.entry(from.to_string())
            .or_default()
            .push(to.to_string());
    }

    fn dfs(&mut self, start: &str) {
        if self.visited.contains(start) {
            return;
        }
        self.visited.insert(start.to_string());
        println!("Visited -> {}", start);

        if let Some(neighbours) = self.graph.get(start).cloned() {
            for neighbour in neighbours {
            self.dfs(neighbour.as_str());
        }
        }
    }
}

fn main() {
    let mut g = Graph::new();

    // connect A to B and C
    g.add_edge("A", "B");
    g.add_edge("A", "C");

    // connect B to A and D
    g.add_edge("B", "A");
    g.add_edge("B", "D");

    // connect C to A and E
    g.add_edge("C", "A");
    g.add_edge("C", "E");

    // connect D to B
    g.add_edge("D", "B");

    // connect E to C
    g.add_edge("E", "C");

    // dfs traversal
    g.dfs("A");
}
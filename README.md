---

# Graph Traversal in Rust (DFS)

This project implements a **basic directed graph** using `HashMap` and explores it using **Depth-First Search (DFS)**.

---

## 🔧 Features

* 📌 **Graph represented as an adjacency list** (`HashMap<String, Vec<String>>`)
* 🧠 **DFS traversal** with cycle handling using a `HashSet<String>`
* ✅ Simple, readable code suitable for learning or quick prototyping

---

## 📦 Structure

```rust
struct Graph {
    graph: HashMap<String, Vec<String>>,
    visited: HashSet<String>,
}
```

* `graph`: Adjacency list storing directed edges
* `visited`: Keeps track of visited nodes to avoid infinite recursion in cyclic graphs

---

## 🧮 Example Graph

```text
   A
  / \
 B   C
 |    \
 D     E
```

Edges are added manually to simulate a graph with bidirectional links:

```rust
g.add_edge("A", "B");
g.add_edge("A", "C");
g.add_edge("B", "A");
g.add_edge("B", "D");
g.add_edge("C", "A");
g.add_edge("C", "E");
g.add_edge("D", "B");
g.add_edge("E", "C");
```

---

## 🚀 How It Works

### DFS Traversal

```rust
fn dfs(&mut self, start: &str)
```

* Starts at a node
* Prints visited node
* Recursively visits unvisited neighbors

### Output

```bash
Visited -> A
Visited -> B
Visited -> D
Visited -> C
Visited -> E
```

> DFS dives deep along one branch before backtracking. The `visited` set prevents revisiting nodes in cycles.

---

## 📚 How to Run

1. Save the code to a file like `main.rs`
2. Run with:

```bash
cargo run
```

---

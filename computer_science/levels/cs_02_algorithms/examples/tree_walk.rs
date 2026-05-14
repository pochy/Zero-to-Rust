use std::collections::VecDeque;

#[derive(Debug)]
struct Node {
    name: String,
    children: Vec<Node>,
}

impl Node {
    fn new(name: &str, children: Vec<Node>) -> Self {
        Self {
            name: name.to_string(),
            children,
        }
    }
}

fn dfs(node: &Node, out: &mut Vec<String>) {
    out.push(node.name.clone());
    for child in &node.children {
        dfs(child, out);
    }
}

fn bfs(root: &Node) -> Vec<String> {
    let mut out = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while let Some(node) = queue.pop_front() {
        out.push(node.name.clone());
        for child in &node.children {
            queue.push_back(child);
        }
    }

    out
}

fn main() {
    let tree = Node::new(
        "root",
        vec![
            Node::new("src", vec![Node::new("main.rs", vec![])]),
            Node::new(
                "docs",
                vec![
                    Node::new("README.md", vec![]),
                    Node::new("guide.md", vec![]),
                ],
            ),
        ],
    );

    let mut dfs_order = Vec::new();
    dfs(&tree, &mut dfs_order);

    println!("dfs: {}", dfs_order.join(", "));
    println!("bfs: {}", bfs(&tree).join(", "));
}

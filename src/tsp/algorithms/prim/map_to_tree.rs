use crate::tsp::algorithms::prim::prim::Node;

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub node: usize,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

pub fn map_nodes_to_tree(nodes: &Vec<Node>) -> Vec<TreeNode> {
    let mut tree: Vec<TreeNode> = vec![];
    for node in nodes {
        tree.push(TreeNode {
            node: node.index,
            parent: node.parent,
            children: vec![],
        })
    }
    for i in 0..nodes.len() {
        let node_parent = tree[i].parent.clone();
        if let Some(node_parent) = node_parent {
            tree[node_parent].children.push(i)
        }
    }
    tree
}

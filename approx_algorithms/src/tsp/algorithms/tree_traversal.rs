use crate::tsp::algorithms::prim::Node;
use crate::tsp::helpers::{map_nodes_to_tree, TreeNode};

pub fn preorder_traversal(nodes: &Vec<Node>) -> Vec<usize> {
    let tree = map_nodes_to_tree(nodes);
    let mut path = vec![];

    path.push(tree[0].node);
    for child in &tree[0].children {
        traverse(child.to_owned(), &tree, &mut path)
    }
    path
}

fn traverse(node: usize, tree: &Vec<TreeNode>, path: &mut Vec<usize>) {
    path.push(node);

    for child in &tree[node].children {
        traverse(child.to_owned(), tree, path);
    }
}

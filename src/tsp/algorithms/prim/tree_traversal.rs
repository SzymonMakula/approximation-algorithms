use crate::tsp::algorithms::prim::map_to_tree::{map_nodes_to_tree, TreeNode};
use crate::tsp::algorithms::prim::prim::Node;

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

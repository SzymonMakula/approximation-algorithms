use crate::prim::prim::Node;

pub fn preorder_traversal(nodes: &[Node], node_index: usize) -> Vec<usize> {
    let mut traversal_order: Vec<usize> = vec![];
    traverse(nodes, node_index, &mut traversal_order);
    traversal_order
}

fn traverse(nodes: &[Node], node_index: usize, order_vector: &mut Vec<usize>) {
    let node = &nodes[node_index];
    order_vector.push(node.index);

    for child in nodes.iter().filter(|n| n.parent == Some(node_index)) {
        traverse(nodes, child.index, order_vector);
    }
}

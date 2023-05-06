use crate::prim::prim::{prim_algorithm, print_mst, Matrix};
use crate::prim::tree_traversal::preorder_traversal;

pub fn approx_tsp_tour(matrix: Matrix) -> i64 {
    let mst = prim_algorithm(matrix.clone(), 0);
    let mut order = preorder_traversal(&mst, 0);
    println!("the order is {:?}", order);

    let current = order.pop().unwrap();
    let cost = sum_path(&matrix, order, current, 0);
    cost
}

fn sum_path(matrix: &Matrix, mut order: Vec<usize>, current: usize, sum: i64) -> i64 {
    if order.is_empty() {
        return sum + matrix[current][matrix.len() - 1];
    }

    let next = order.pop().unwrap();
    return sum_path(matrix, order, next, sum + matrix[current][next]);
}

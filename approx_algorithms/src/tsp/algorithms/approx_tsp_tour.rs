use crate::tsp::algorithms::prim::{prim_algorithm, Matrix};
use crate::tsp::algorithms::tree_traversal::preorder_traversal;

pub fn approx_tsp_tour(matrix: Matrix) -> (i64, Vec<usize>) {
    let mst = prim_algorithm(matrix.clone(), 0);
    let mut order = preorder_traversal(&mst);
    let mut tour = order
        .clone()
        .into_iter()
        .map(|stop| stop + 1)
        .collect::<Vec<usize>>();
    tour.push(tour[0]);

    let current = order.pop().unwrap();
    let cost = sum_path(&matrix, order, current, 0);
    (cost, tour)
}

pub fn sum_path(matrix: &Matrix, mut order: Vec<usize>, current: usize, sum: i64) -> i64 {
    if order.is_empty() {
        return sum + matrix[current][matrix.len() - 1];
    }

    let next = order.pop().unwrap();
    return sum_path(matrix, order, next, sum + matrix[current][next]);
}

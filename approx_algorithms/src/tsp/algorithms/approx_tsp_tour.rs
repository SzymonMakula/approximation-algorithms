use crate::tsp::algorithms::christofides_algorithm::calculate_cost;
use crate::tsp::algorithms::prim::{prim_algorithm, Matrix};
use crate::tsp::algorithms::tree_traversal::preorder_traversal;

pub fn approx_tsp_tour(matrix: Matrix) -> (i64, Vec<usize>) {
    let mst = prim_algorithm(matrix.clone(), 0);
    let mut order = preorder_traversal(&mst);
    let cost = calculate_cost(&matrix, &order);
    let tour = create_tour(order);

    (cost, tour)
}

fn create_tour(order: Vec<usize>) -> Vec<usize> {
    let mut tour = order
        .into_iter()
        .map(|stop| stop + 1)
        .collect::<Vec<usize>>();
    tour.push(tour[0]);

    tour
}

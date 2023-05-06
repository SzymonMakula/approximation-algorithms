use magisterka_projekt::prim::prim::{prim_algorithm, print_mst, Node};
use magisterka_projekt::prim::tree_traversal::preorder_traversal;
use magisterka_projekt::tsp::algorithms::approx_tsp_tour::approx_tsp_tour;
use magisterka_projekt::tsp::parsers::parsers::{construct_adjacency_matrix, get_data_set};

fn main() {
    let other = vec![
        vec![0, 10, 15, 20],
        vec![10, 0, 35, 25],
        vec![15, 35, 0, 30],
        vec![20, 25, 30, 0],
    ];
    let data_set = get_data_set();
    println!("{:?}", data_set);
    let matrix = construct_adjacency_matrix(data_set);
    println!("{:?}", matrix);
    let mst = approx_tsp_tour(matrix);
    println!("{}", mst);
}

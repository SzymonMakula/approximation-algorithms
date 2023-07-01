use std::time::Instant;

use magisterka_projekt::knapsack::algorithms::dynamic_programming::dynamic_programming_knapsack;
use magisterka_projekt::knapsack::algorithms::fptas::fptas_knapsack;
use magisterka_projekt::knapsack::algorithms::greedy::greedy_algorithm;
use magisterka_projekt::tsp::algorithms::approx_tsp_tour::approx_tsp_tour;
use magisterka_projekt::tsp::algorithms::christofides_algorithm::christofides_algorithm;
use magisterka_projekt::tsp::algorithms::munkers::munkers;
use magisterka_projekt::tsp::algorithms::prim::prim_algorithm;
use magisterka_projekt::tsp::parsers::parsers::{
    construct_adjacency_matrix, get_data_set, get_data_sets,
};

fn main() {
    run_christofides();
    run_tsp_dtree()
}

fn run_christofides() {
    let data_sets = get_data_sets();
    data_sets.into_iter().for_each(|data_set| {
        let opt = data_set.optimum;
        let now = Instant::now();
        let res = christofides_algorithm(data_set);
        println!(
            "got {}, opt {}, ratio {}, elapsed {:?}",
            res,
            opt,
            res as f64 / opt as f64,
            now.elapsed()
        )
    })
}

fn run_tsp_dtree() {
    let data_sets = get_data_sets();
    data_sets.into_iter().for_each(|data_set| {
        let opt = data_set.optimum;
        let now = Instant::now();
        let matrix = construct_adjacency_matrix(&data_set);
        let res = approx_tsp_tour(matrix);
        println!(
            "got {}, opt {}, ratio {}, elapsed {:?}",
            res,
            opt,
            res as f64 / opt as f64,
            now.elapsed()
        )
    })
}

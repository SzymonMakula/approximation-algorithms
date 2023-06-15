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
    // christofides_algorithm(test)
    read_tsp_set()
}

fn read_tsp_set() {
    let data_set = get_data_set();
    let matrix = construct_adjacency_matrix(&data_set);
    let time = Instant::now();
    let test = vec![
        vec![i64::MAX - 10, 29, 35, 71, 67, 98],
        vec![46, i64::MAX - 10, 26, 89, 85, 48],
        vec![57, 92, i64::MAX - 10, 72, 86, 3],
        vec![81, 37, 44, i64::MAX - 10, 39, 64],
        vec![77, 31, 16, 55, i64::MAX - 10, 61],
        vec![69, 75, 30, 45, 23, i64::MAX - 10],
    ];

    let adjacency_matrix = vec![
        vec![i64::MAX, 1, 2, 1, 1],
        vec![1, i64::MAX, 1, 2, 1],
        vec![2, 1, i64::MAX, 1, 1],
        vec![1, 2, 1, i64::MAX, 1],
        vec![1, 1, 1, 1, i64::MAX],
    ];

    let mst = christofides_algorithm(adjacency_matrix);
    // println!(
    //     "Error is: {:.2}% | cities: {} | elapsed: {:?}",
    //     (1.0 - (opt as f64 / mst as f64) as f64) * 100.0,
    //     data_set.dimension,
    //     time.elapsed()
    // )
}

// fn read_kp_set() {
//     let data_set = get_data_set("/home/szymon/FunProjects/magisterka/magisterka-projekt/src/knapsack/datasets/knapPI_3_100_10000.csv");
//     for set in data_set {
//         let result = dynamic_programming_knapsack(set);
//         println!(
//             "ratio: {:.6} | time {:?} | capacity: {} | max profit: {} | items count {} ",
//             result.ratio,
//             result.execution_time,
//             result.data_set.capacity,
//             result
//                 .data_set
//                 .records
//                 .iter()
//                 .map(|record| record.value)
//                 .sum::<i64>(),
//             result.data_set.records.len()
//         )
//     }
// }

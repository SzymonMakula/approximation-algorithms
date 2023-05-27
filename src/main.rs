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
        vec![i64::MAX, 10, 15, 20],
        vec![10, i64::MAX, 5, 25],
        vec![15, 35, i64::MAX, 3],
        vec![20, 25, 30, i64::MAX],
    ];
    let adjacency_matrix = vec![
        vec![i64::MAX - 10, 29, 35, 71, 67, 98, 58, 93, 43, 19],
        vec![46, i64::MAX - 10, 26, 89, 85, 48, 4, 40, 36, 42],
        vec![57, 92, i64::MAX - 10, 72, 86, 3, 76, 51, 15, 27],
        vec![81, 37, 44, i64::MAX - 10, 39, 64, 47, 73, 94, 5],
        vec![77, 31, 16, 55, i64::MAX - 10, 61, 91, 88, 65, 13],
        vec![69, 75, 30, 45, 23, i64::MAX - 10, 84, 68, 9, 79],
        vec![95, 53, 82, 66, 97, 10, i64::MAX - 10, 50, 2, 20],
        vec![12, 87, 7, 96, 70, 28, 32, i64::MAX - 10, 80, 63],
        vec![24, 41, 14, 54, 74, 56, 6, 38, i64::MAX - 10, 60],
        vec![22, 8, 18, 83, 33, 62, 34, 11, 59, i64::MAX - 10],
    ];

    let mst = christofides_algorithm(adjacency_matrix);
    println!("val is {}", mst);
    let opt = data_set.optimum;
    println!(
        "Error is: {:.2}% | cities: {} | elapsed: {:?}",
        (1.0 - (opt as f64 / mst as f64) as f64) * 100.0,
        data_set.dimension,
        time.elapsed()
    )
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

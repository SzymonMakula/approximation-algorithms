use std::time::Instant;

use magisterka_projekt::knapsack::algorithms::dynamic_programming::dynamic_programming_knapsack;
use magisterka_projekt::knapsack::algorithms::fptas::fptas_knapsack;
use magisterka_projekt::knapsack::algorithms::greedy::greedy_algorithm;
use magisterka_projekt::knapsack::helpers::helpers::get_data_set;
use magisterka_projekt::tsp::algorithms::approx_tsp_tour::approx_tsp_tour;
use magisterka_projekt::tsp::algorithms::munkers::munkers;
use magisterka_projekt::tsp::parsers::parsers::{construct_adjacency_matrix, get_data_sets};

// let other = vec![
//     vec![0, 10, 15, 20],
//     vec![10, 0, 35, 25],
//     vec![15, 35, 0, 30],
//     vec![20, 25, 30, 0],
// ];
fn main() {
    let other = vec![vec![1, 2, 3], vec![2, 4, 6], vec![3, 6, 9]];
    let other_2 = vec![vec![8, 4, 7], vec![5, 2, 3], vec![9, 4, 8]];

    munkers(other_2)
}

fn read_tsp_set() {
    let other = vec![
        vec![0, 10, 15, 20],
        vec![10, 0, 35, 25],
        vec![15, 35, 0, 30],
        vec![20, 25, 30, 0],
    ];
    let data_sets = get_data_sets();
    data_sets.iter().for_each(|set| {
        let matrix = construct_adjacency_matrix(set);
        let time = Instant::now();
        let mst = approx_tsp_tour(matrix);
        let opt = set.optimum;
        println!(
            "Error is: {:.2}% | cities: {} | elapsed: {:?}",
            (1.0 - (opt as f64 / mst as f64) as f64) * 100.0,
            set.dimension,
            time.elapsed()
        )
    });
}

fn read_kp_set() {
    let data_set = get_data_set("/home/szymon/FunProjects/magisterka/magisterka-projekt/src/knapsack/datasets/knapPI_3_100_10000.csv");
    for set in data_set {
        let result = dynamic_programming_knapsack(set);
        println!(
            "ratio: {:.6} | time {:?} | capacity: {} | max profit: {} | items count {} ",
            result.ratio,
            result.execution_time,
            result.data_set.capacity,
            result
                .data_set
                .records
                .iter()
                .map(|record| record.value)
                .sum::<i64>(),
            result.data_set.records.len()
        )
    }
}

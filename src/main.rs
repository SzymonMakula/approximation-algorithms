use std::time::Instant;

use magisterka_projekt::knapsack::algorithms::fptas::fptas_knapsack;
use magisterka_projekt::knapsack::helpers::helpers::get_data_set;
use magisterka_projekt::tsp::algorithms::approx_tsp_tour::approx_tsp_tour;
use magisterka_projekt::tsp::parsers::parsers::{construct_adjacency_matrix, get_data_sets};

fn main() {
    let data_set = get_data_set("/home/szymon/FunProjects/magisterka/magisterka-projekt/src/knapsack/datasets/knapPI_6_100_10000.csv");
    for set in data_set {
        let result = fptas_knapsack(set, 0.9);
        println!(
            " val {} vs {:}, ratio: {}, capacity: {}, and time {:?}",
            result.result,
            result.data_set.optimal_value,
            result.ratio,
            result.data_set.capacity,
            result.execution_time
        )
    }
}

fn test() {
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
            "Error is: {:.2}%,Ratio is: {}, got: {}, optimum: {}, elapsed: {:?}",
            (1.0 - (opt as f64 / mst as f64) as f64) * 100.0,
            (opt as f64 / mst as f64) as f64,
            mst,
            opt,
            time.elapsed()
        )
    });
}

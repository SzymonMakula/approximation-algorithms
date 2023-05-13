use std::time::Instant;

use magisterka_projekt::tsp::algorithms::approx_tsp_tour::approx_tsp_tour;
use magisterka_projekt::tsp::parsers::parsers::{
    construct_adjacency_matrix, get_data_set, get_data_sets,
};

fn main() {
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

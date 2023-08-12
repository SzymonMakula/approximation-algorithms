use std::fs;
use std::path::Path;
use std::time::Instant;

use crate::knapsack::algorithms::dynamic_programming::{
    dynamic_programming_knapsack, kp_dynamic_by_weight,
};
use crate::knapsack::algorithms::fptas::fptas_knapsack;
use crate::knapsack::algorithms::greedy::greedy_algorithm;
use crate::knapsack::algorithms::types::SolveResult;
use crate::knapsack::parsers::parsers::{parse_entry, DataSet, InstanceType};

pub fn get_solve_results(algorithm: fn(DataSet) -> SolveResult) -> Vec<SolveResult> {
    let folder = fs::read_dir("../datasets").unwrap();

    let mut values: Vec<SolveResult> = Vec::new();
    for file in folder {
        let path = file.unwrap().path();
        let contents = fs::read_to_string(path).unwrap();
        let data_sets = parse_file_to_datasets(contents);

        let mut results = data_sets
            .into_iter()
            .map(algorithm)
            .collect::<Vec<SolveResult>>();
        values.append(&mut results);
    }
    values
}

pub fn get_data_sets() -> Vec<DataSet> {
    let folder = fs::read_dir("../datasets").unwrap();
    let mut result: Vec<DataSet> = Vec::new();
    for file in folder {
        let path = file.unwrap().path();
        let contents = fs::read_to_string(path).unwrap();
        let mut data_sets = parse_file_to_datasets(contents);

        result.append(&mut data_sets);
    }
    result
}

pub fn get_data_set(path: &str) -> Vec<DataSet> {
    let content = fs::read_to_string(path).unwrap();
    parse_file_to_datasets(content)
}

fn parse_file_to_datasets(file_content: String) -> Vec<DataSet> {
    let files = file_content.split("-----").collect::<Vec<&str>>();

    files
        .iter()
        .map(|&entry| parse_entry(entry))
        .filter_map(|entry| entry)
        .collect::<Vec<DataSet>>()
}

fn get_instance_average(results: &Vec<SolveResult>, instance_type: InstanceType) -> f64 {
    let uncorrelated_results_iter = results
        .iter()
        .filter(|result| matches!(&result.data_set.instance_type, instance_type))
        .map(|result| result.ratio);
    let len = uncorrelated_results_iter
        .clone()
        .collect::<Vec<f64>>()
        .len();
    uncorrelated_results_iter.sum::<f64>() / len as f64
}

pub fn run_greedy_kp() {
    let DATA_SETS_PATHS = vec![
        "knapPI_1_100_1000",
        "knapPI_1_100_10000",
        "knapPI_1_1000_1000",
        "knapPI_1_1000_10000",
        "knapPI_3_100_1000",
        "knapPI_3_100_10000",
        "knapPI_3_1000_1000",
        "knapPI_3_1000_10000",
    ];
    for path in DATA_SETS_PATHS {
        let data_sets = get_data_set(&format!("../datasets/knapsack/{}.csv", path));
        for set in data_sets {
            let name = set.title.to_string();
            let optimum_value = set.optimal_value;
            let now = Instant::now();

            greedy_algorithm(set);
            let elapsed = now.elapsed().as_micros();
            println!("solving greedy with {} in time {}um", name, elapsed)
        }
    }
}

pub fn run_dynamic_kp() {
    let DATA_SETS_PATHS = vec![
        "knapPI_1_100_1000",
        "knapPI_1_100_10000",
        "knapPI_1_1000_1000",
        "knapPI_1_1000_10000",
        "knapPI_3_100_1000",
        "knapPI_3_100_10000",
        "knapPI_3_1000_1000",
        "knapPI_3_1000_10000",
    ];
    for path in DATA_SETS_PATHS {
        let data_sets = get_data_set(&format!("../datasets/knapsack/{}.csv", path));
        for set in data_sets {
            let name = set.title.to_string();
            let optimum_value = set.optimal_value;
            let now = Instant::now();

            dynamic_programming_knapsack(set);
            let elapsed = now.elapsed().as_micros();
            println!("solving dynamic with {} in time {}um", name, elapsed)
        }
    }
}

pub fn run_dynamic_weight_kp() {
    let DATA_SETS_PATHS = vec![
        "knapPI_1_100_1000",
        "knapPI_1_100_10000",
        "knapPI_1_1000_1000",
        "knapPI_1_1000_10000",
        "knapPI_3_100_1000",
        "knapPI_3_100_10000",
        "knapPI_3_1000_1000",
        "knapPI_3_1000_10000",
    ];

    for path in DATA_SETS_PATHS {
        let data_sets = get_data_set(&format!("../datasets/knapsack/{}.csv", path));
        for set in data_sets {
            let name = set.title.to_string();
            let optimum_value = set.optimal_value;
            let now = Instant::now();
            let values = set
                .records
                .iter()
                .map(|record| record.value)
                .collect::<Vec<i64>>();
            let weights = set
                .records
                .iter()
                .map(|record| record.weight)
                .collect::<Vec<i64>>();

            kp_dynamic_by_weight(values, weights, set.capacity);
            let elapsed = now.elapsed().as_micros();
            println!("solving dynamic weight with {} in time {}um", name, elapsed)
        }
    }
}

pub fn run_fptas_kp(e: f64) {
    let DATA_SETS_PATHS = vec![
        "knapPI_1_100_1000",
        "knapPI_1_100_10000",
        "knapPI_1_1000_1000",
        "knapPI_1_1000_10000",
        "knapPI_3_100_1000",
        "knapPI_3_100_10000",
        "knapPI_3_1000_1000",
        "knapPI_3_1000_10000",
    ];
    for path in DATA_SETS_PATHS {
        let data_sets = get_data_set(&format!("../datasets/knapsack/{}.csv", path));
        for set in data_sets {
            let name = set.title.to_string();
            let optimum_value = set.optimal_value;
            let now = Instant::now();

            let result = fptas_knapsack(set, e);
            let elapsed = now.elapsed().as_secs();
            let err = (optimum_value - result) as f64 * 100.0 / result as f64;
            println!(
                "solving fptas with {} in time {}s, error {}",
                name, elapsed, err
            )
        }
    }
}

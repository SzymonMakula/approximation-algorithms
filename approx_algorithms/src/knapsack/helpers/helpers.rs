use std::fs;
use std::path::Path;
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::knapsack::algorithms::dynamic_programming::{
    dynamic_programming_knapsack, kp_dynamic_by_weight, Estimate,
};
use crate::knapsack::algorithms::fptas::fptas_knapsack;
use crate::knapsack::algorithms::greedy::greedy_knapsack;
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
        let mut runs = vec![];

        for set in data_sets {
            let name = set.title.to_string();
            let optimum_value = set.optimal_value;
            let capacity = set.capacity;
            let now = Instant::now();

            let result = greedy_knapsack(set);
            let elapsed = now.elapsed().as_micros();
            println!("solving greedy with {}", name);
            let run_data = KnapsackRunData {
                time_micros: elapsed,
                optimum_value,
                result,
                name,
                capacity,
            };
            runs.push(run_data)
        }

        let knapsack_solve_result = KnapsackSolveResult {
            runs,
            name: path.to_string(),
        };

        let save_path = format!("../dist/knapsack/greedy/{}.json", path);
        fs::create_dir_all(format!("../dist/knapsack/greedy"))
            .expect("Failed to create directories");
        let json_string = serde_json::to_string(&knapsack_solve_result).unwrap();
        fs::write(save_path, json_string).unwrap();
    }
}

pub fn run_dynamic_kp() {
    let DATA_SETS_PATHS = vec![
        // "knapPI_1_100_1000",
        "knapPI_1_100_10000",
        // "knapPI_1_1000_1000",
        // "knapPI_1_1000_10000",
        // "knapPI_3_100_1000",
        // "knapPI_3_100_10000",
        // "knapPI_3_1000_1000",
        // "knapPI_3_1000_10000",
    ];
    for path in DATA_SETS_PATHS {
        let data_sets = get_data_set(&format!("../datasets/knapsack/{}.csv", path));

        let mut runs = vec![];
        let mut estimates = vec![];
        for set in data_sets {
            let name = set.title.to_string();
            let optimum_value = set.optimal_value;
            let capacity = set.capacity;
            let now = Instant::now();
            let (result, estimate) = dynamic_programming_knapsack(set);

            let elapsed = now.elapsed().as_micros();
            println!("solving dynamic with {} in time {}", name, elapsed);

            let run_data = KnapsackRunData {
                time_micros: elapsed,
                optimum_value,
                result,
                name,
                capacity,
            };
            estimates.push(estimate);
            runs.push(run_data)
        }
        let knapsack_solve_result = KnapsackSolveResult {
            runs,
            name: path.to_string(),
        };
        let save_path = format!("../dist/knapsack/dynamic/{}.json", path);
        fs::create_dir_all(format!("../dist/knapsack/dynamic/"))
            .expect("Failed to create directories");
        let json_string = serde_json::to_string(&knapsack_solve_result).unwrap();
        fs::write(save_path, json_string).unwrap();

        let estimatesStruct = Estimates { estimates };

        let json_string = serde_json::to_string(&estimatesStruct).unwrap();
        fs::write("../dist/knapsack/dynamic/estimate.json", json_string).unwrap();
    }
}

pub fn run_dynamic_weight_kp() {
    let DATA_SETS_PATHS = vec![
        // "knapPI_1_100_1000",
        // "knapPI_1_100_10000",
        // "knapPI_1_1000_1000",
        "knapPI_1_1000_10000",
        // "knapPI_3_100_1000",
        // "knapPI_3_100_10000",
        "knapPI_3_1000_1000",
        // "knapPI_3_1000_10000",
    ];

    for path in DATA_SETS_PATHS {
        let mut runs = vec![];

        let data_sets = get_data_set(&format!("../datasets/knapsack/{}.csv", path));
        for set in data_sets {
            let name = set.title.to_string();
            let optimum_value = set.optimal_value;
            let capacity = set.capacity;
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

            let result = kp_dynamic_by_weight(values, weights, set.capacity);
            let elapsed = now.elapsed().as_micros();
            println!("solving dynamic weight with {} in time {}um", name, elapsed);

            let run_data = KnapsackRunData {
                time_micros: elapsed,
                optimum_value,
                result,
                name,
                capacity,
            };
            runs.push(run_data)
        }
        let knapsack_solve_result = KnapsackSolveResult {
            runs,
            name: path.to_string(),
        };
        let save_path = format!("../dist/knapsack/dynamicW/{}.json", path);
        fs::create_dir_all(format!("../dist/knapsack/dynamicW/"))
            .expect("Failed to create directories");

        let json_string = serde_json::to_string(&knapsack_solve_result).unwrap();
        fs::write(save_path, json_string).unwrap();
    }
}

pub fn run_fptas_kp(e: f64) {
    let DATA_SETS_PATHS = vec![
        // "knapPI_1_100_1000",
        // "knapPI_1_100_10000",
        "knapPI_1_1000_1000",
        // "knapPI_1_1000_10000",
        // "knapPI_3_100_1000",
        "knapPI_3_100_10000",
        // "knapPI_3_1000_1000",
        // "knapPI_3_1000_10000",
    ];
    for path in DATA_SETS_PATHS {
        let data_sets = get_data_set(&format!("../datasets/knapsack/{}.csv", path));
        let mut runs = vec![];

        for set in data_sets {
            let name = set.title.to_string();
            let optimum_value = set.optimal_value;
            let capacity = set.capacity;

            let now = Instant::now();
            let result = fptas_knapsack(set, e);
            let elapsed = now.elapsed().as_micros();

            println!("solving fptas with {}", name);
            let run_data = KnapsackRunData {
                time_micros: elapsed,
                optimum_value,
                result,
                name,
                capacity,
            };
            runs.push(run_data)
        }

        let knapsack_solve_result = KnapsackSolveResult {
            runs,
            name: path.to_string(),
        };

        let save_path = format!("../dist/knapsack/fptas/{}/{}.json", e.to_string(), path);
        fs::create_dir_all(format!("../dist/knapsack/fptas/{}", e.to_string()))
            .expect("Failed to create directories");
        let json_string = serde_json::to_string(&knapsack_solve_result).unwrap();
        fs::write(save_path, json_string).unwrap();
    }
}

#[derive(Serialize, Deserialize)]
pub struct Estimates {
    estimates: Vec<Estimate>,
}

#[derive(Serialize, Deserialize)]
pub struct KnapsackSolveResult {
    name: String,
    runs: Vec<KnapsackRunData>,
}

#[derive(Serialize, Deserialize)]
pub struct KnapsackRunData {
    name: String,
    optimum_value: i64,
    time_micros: u128,
    capacity: i64,
    result: i64,
}

use std::fs;
use std::path::Path;

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

pub fn get_uncorrelated_data_set() -> Vec<DataSet> {
    let content = fs::read_to_string("../datasets/knapPI_1_100_1000.csv").unwrap();
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

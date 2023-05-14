use std::time::Instant;

use crate::knapsack::algorithms::dynamic_programming::{
    _dynamic_programming_knapsack, kp_dynamic_by_values, kp_dynamic_by_weight,
};
use crate::knapsack::algorithms::types::SolveResult;
use crate::knapsack::parsers::parsers::DataSet;

pub fn fptas_knapsack(data_set: DataSet, e: f64) -> SolveResult {
    let now = Instant::now();
    let items_count = data_set.records.len() as f64;
    let values = data_set
        .records
        .iter()
        .map(|record| record.value)
        .collect::<Vec<i64>>();
    let weights = data_set
        .records
        .iter()
        .map(|record| record.weight)
        .collect::<Vec<i64>>();

    let max_value = values.iter().max().unwrap().to_owned() as f64;

    let mut K = (e * max_value) / items_count;

    if K.floor() == 0.0 {
        K = 1.0;
    }

    let new_values = values
        .iter()
        .map(|&value| (value as f64 / K as f64).floor() as i64)
        .collect::<Vec<i64>>();

    let result = _dynamic_programming_knapsack(new_values, weights, data_set.capacity) as f64;

    SolveResult {
        result: (result * K) as i64,
        ratio: (result * K) / data_set.optimal_value as f64,
        data_set,
        execution_time: now.elapsed(),
    }
}

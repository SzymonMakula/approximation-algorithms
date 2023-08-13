use std::time::Instant;

use crate::knapsack::algorithms::dynamic_programming::{
    _dynamic_programming_knapsack, kp_dynamic_by_values, kp_dynamic_by_weight,
};
use crate::knapsack::algorithms::types::SolveResult;
use crate::knapsack::parsers::parsers::DataSet;

pub fn fptas_knapsack(data_set: DataSet, e: f64) -> i64 {
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

    let values_sum = new_values.iter().sum::<i64>();
    println!(
        "values is greater than capacity: {}. K is {}, set is {}",
        values_sum > data_set.capacity,
        K,
        data_set.title
    );

    let (val, items) = _dynamic_programming_knapsack(new_values, weights, data_set.capacity);

    let result = items.into_iter().map(|index| values[index]).sum::<i64>();
    result
}

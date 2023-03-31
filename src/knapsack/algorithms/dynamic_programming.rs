use std::cmp::max;
use std::io::ErrorKind::TimedOut;
use std::time::Instant;

use crate::knapsack::algorithms::types::SolveResult;
use crate::knapsack::parsers::parsers::DataSet;

type Pair = (i64, i64);

pub fn _dynamic_programming_knapsack(values: Vec<i64>, weights: Vec<i64>, capacity: i64) -> i64 {
    let items_count = values.len();
    let first_item = (weights[0], values[0]);

    let mut A: Vec<Vec<Pair>> = vec![vec![(0, 0), first_item]; items_count];

    for j in 1..items_count {
        A[j] = A[j - 1].clone();
        for (weight, value) in &A[j - 1].clone() {
            if weight + weights[j] <= capacity {
                A[j].push((weight + weights[j], value + values[j]))
            }
        }
        A[j] = get_dominating_pairs(A[j].clone())
    }
    let max_value = A
        .iter()
        .map(|v| v.iter().map(|(w, v)| v).max().unwrap())
        .max()
        .unwrap()
        .to_owned();
    max_value
}

pub fn dynamic_programming_knapsack(data_set: DataSet) -> SolveResult {
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

    let now = Instant::now();
    let result = _dynamic_programming_knapsack(values, weights, data_set.capacity);

    SolveResult {
        result,
        execution_time: now.elapsed(),
        ratio: result as f64 / data_set.optimal_value as f64,
        data_set,
    }
}

fn get_dominating_pairs(mut pairs: Vec<Pair>) -> Vec<Pair> {
    pairs.sort_by_key(|&(weight, value)| (weight, std::cmp::Reverse(value)));
    let mut result: Vec<Pair> = vec![];
    for (weight, value) in pairs {
        if result.is_empty() || value > result.last().unwrap().1 {
            result.push((weight, value));
        }
    }
    result
}

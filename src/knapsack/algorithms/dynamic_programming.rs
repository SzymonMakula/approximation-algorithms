use std::cmp::min;
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

fn kp_dynamic_by_values(old_values: Vec<i64>, old_weights: Vec<i64>, capacity: i64) -> i64 {
    let items_count = old_values.len();

    let mut values = vec![0];
    values.append(&mut old_values.clone());
    let mut weights = vec![0];
    weights.append(&mut old_weights.clone());

    let max_value = values.iter().sum::<i64>().to_owned() as usize;

    let mut dp: Vec<Vec<i64>> = vec![vec![0; max_value]; items_count + 1];

    dp[0] = dp[0]
        .clone()
        .into_iter()
        .map(|val| 1000000000000000)
        .collect();
    dp[0][0] = 0;
    for i in 1..items_count + 1 {
        for k in 0..max_value {
            if values[i] <= k as i64
                && ((dp[i - 1][(k as i64 - values[i]) as usize] + weights[i])
                    <= min(capacity, dp[i - 1][k]))
            {
                dp[i][k] = dp[i - 1][(k as i64 - values[i]) as usize] + weights[i]
            } else {
                dp[i][k] = dp[i - 1][k]
            }
        }
    }

    let mut max_k = 0;
    for i in 1..items_count + 1 {
        for k in 0..max_value {
            if k > max_k && dp[i][k] < 1000000000000000 {
                max_k = k;
            }
        }
    }
    // println!("{:?} and optimal {}", max_k, data_set.optimal_value);
    max_value as i64
}

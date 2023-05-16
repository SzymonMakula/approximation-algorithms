use std::cmp::{max, min};
use std::time::Instant;

use crate::knapsack::algorithms::types::SolveResult;
use crate::knapsack::parsers::parsers::DataSet;

type Pair = (i64, i64);

#[derive(Clone)]
struct Record {
    pub value: i64,
    pub weight: i64,
    pub index: usize,
    chosen_items: Vec<usize>,
}

pub fn _dynamic_programming_knapsack(
    values: Vec<i64>,
    weights: Vec<i64>,
    capacity: i64,
) -> (i64, Vec<usize>) {
    let items_count = values.len();

    let mock = Record {
        index: usize::MAX,
        weight: 0,
        value: 0,
        chosen_items: vec![],
    };

    let first_item = Record {
        index: 0,
        value: values[0],
        weight: weights[0],
        chosen_items: vec![0],
    };

    let mut A: Vec<Vec<Record>> = vec![vec![mock, first_item]; items_count];

    for j in 1..items_count {
        A[j] = A[j - 1].clone();
        for record in &A[j - 1].clone() {
            if record.weight + weights[j] <= capacity {
                let item = Record {
                    weight: record.weight + weights[j],
                    value: record.value + values[j],
                    index: j,
                    chosen_items: {
                        let mut new_vec = record.chosen_items.clone();
                        new_vec.push(j);
                        new_vec
                    },
                };
                A[j].push(item)
            }
        }
        A[j] = get_dominating_pairs(A[j].clone())
    }

    let record = A
        .remove(A.len() - 1)
        .into_iter()
        .max_by_key(|record| record.value)
        .unwrap();

    (record.value, record.chosen_items)
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
    let (result, items) = _dynamic_programming_knapsack(values, weights, data_set.capacity);

    SolveResult {
        result,
        execution_time: now.elapsed(),
        ratio: result as f64 / data_set.optimal_value as f64,
        data_set,
    }
}

fn get_dominating_pairs(mut pairs: Vec<Record>) -> Vec<Record> {
    pairs.sort_by_key(|record| (record.weight, std::cmp::Reverse(record.value)));
    let mut result: Vec<Record> = vec![];
    for record in pairs {
        if result.is_empty() || record.value > result.last().unwrap().value {
            result.push(record);
        }
    }
    result
}

pub fn kp_dynamic_by_values(old_values: Vec<i64>, old_weights: Vec<i64>, capacity: i64) -> i64 {
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

pub fn kp_dynamic_by_weight(old_values: Vec<i64>, old_weights: Vec<i64>, capacity: i64) -> i64 {
    let items_count = old_values.len();

    let mut values: Vec<i64> = vec![];
    values.push(0);
    old_values
        .iter()
        .for_each(|record| values.push(record.to_owned()));

    let mut weights: Vec<i64> = vec![];
    weights.push(0);

    old_weights
        .iter()
        .for_each(|record| weights.push(record.to_owned()));

    let mut solutions: Vec<Vec<i64>> = vec![];

    for _ in 0..=items_count {
        let mut vector = Vec::with_capacity(capacity as usize);
        for _ in 0..=capacity {
            vector.push(0)
        }
        solutions.push(vector)
    }

    for n in 1..=items_count {
        for w in 0..=capacity {
            if weights[n] > w {
                solutions[n][w as usize] = solutions[n - 1][w as usize]
            } else {
                solutions[n][w as usize] = max(
                    solutions[n - 1][w as usize],
                    solutions[n - 1][(w - weights[n]) as usize] + values[n],
                )
            }
        }
    }
    let best = solutions
        .into_iter()
        .map(|solution| solution.into_iter().map(|val| val).max().unwrap())
        .max()
        .unwrap();

    best
}

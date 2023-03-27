use std::cmp::max;
use std::fs;
use std::time::Instant;

use magisterka_projekt::knapsack::algorithms::greedy::greedy_algorithm;
use magisterka_projekt::knapsack::algorithms::types::SolveResult;
use magisterka_projekt::knapsack::helpers::helpers::{
    get_data_sets, get_solve_results, get_uncorrelated_data_set,
};
use magisterka_projekt::knapsack::parsers::parsers::DataSet;

fn pseudo_polynomial_knapsack(data_set: DataSet) -> SolveResult {
    let now = Instant::now();
    let records = &data_set.records;
    let items_count = records.len();

    let mut values: Vec<i64> = vec![];
    values.push(0);
    records.iter().for_each(|record| values.push(record.value));

    let mut weights: Vec<i64> = vec![];
    weights.push(0);

    records
        .iter()
        .for_each(|record| weights.push(record.weight));

    let mut solutions: Vec<Vec<i64>> = vec![];

    for _ in 0..=items_count {
        let mut vector = Vec::with_capacity(data_set.capacity as usize);
        for _ in 0..=data_set.capacity {
            vector.push(0)
        }
        solutions.push(vector)
    }

    for n in 1..=items_count {
        for w in 0..=data_set.capacity {
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

    SolveResult {
        ratio: best as f64 / data_set.optimal_value as f64,
        data_set,
        result: best,
        execution_time: now.elapsed(),
    }
}

fn dynamic_programming_knapsack(data_set: DataSet) {
    let mut records = &data_set.records;
    let first_item = &records[0];

    let mut A = vec![];
    let first_entry = vec![(0, 0), (first_item.weight, first_item.value)];
    A.push(first_entry);

    for j in 1..records.len() {
        A.push(A[j - 1].clone());
        for (weight, value) in &A[j - 1].clone() {
            if weight + records[j].weight <= data_set.capacity {
                A[j].push((weight + records[j].weight, value + records[j].value))
            }
        }
        A[j].sort_by_key(|&(space, value)| (space, std::cmp::Reverse(value)));

        // Keep the non-dominated pairs
        let mut result: Vec<(i64, i64)> = vec![];
        for (space, value) in A[j].clone() {
            if result.is_empty() || value > result.last().unwrap().1 {
                result.push((space, value));
            }
        }
        A[j] = result
    }
    let max = A
        .iter()
        .map(|v| v.iter().map(|(w, v)| v).max().unwrap())
        .max()
        .unwrap();
    println!("{} and optimal is {}", max, data_set.optimal_value);

    // println!("{:?}", A)
}

fn main() {
    let data_sets = get_uncorrelated_data_set();
    let data_set = data_sets.into_iter().nth(9).unwrap();
    let solve = dynamic_programming_knapsack(data_set);

    // data_sets.into_iter().for_each(|data_set| {
    //     let solve = pseudo_polynomial_knapsack(data_set);
    //     println!(
    //         "best: {:?} optimal: {}, time: {:?}, size: {}, capacity: {}",
    //         solve.result,
    //         solve.data_set.optimal_value,
    //         solve.execution_time,
    //         solve.data_set.items_count,
    //         solve.data_set.capacity
    //     );
    // })
}

fn print_solve_result(solve_result: SolveResult) {
    println!(
        "best: {:?} optimal: {}, time: {:?}, size: {}, capacity: {}",
        solve_result.result,
        solve_result.data_set.optimal_value,
        solve_result.execution_time,
        solve_result.data_set.items_count,
        solve_result.data_set.capacity
    );
}

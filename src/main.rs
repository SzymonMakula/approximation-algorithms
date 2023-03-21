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
    let values = records
        .iter()
        .map(|record| record.value)
        .collect::<Vec<i64>>();

    let weights = records
        .iter()
        .map(|record| record.weight)
        .collect::<Vec<i64>>();

    let mut solutions: Vec<Vec<i64>> = vec![];

    for _ in 0..items_count {
        let mut vector = Vec::with_capacity(data_set.capacity as usize);
        for _ in 0..data_set.capacity {
            vector.push(0)
        }
        solutions.push(vector)
    }

    for n in 1..items_count {
        for w in 1..data_set.capacity {
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

fn main() {
    let data_sets = get_uncorrelated_data_set();
    let data_set = data_sets.into_iter().nth(9).unwrap();
    let solve = pseudo_polynomial_knapsack(data_set);
    println!(
        "best is {:?} and optimal is {} and time is {:?}",
        solve.result, solve.data_set.optimal_value, solve.execution_time
    );
    // data_sets.into_iter().for_each(|data_set| {
    //     let solve = pseudo_polynomial_knapsack(data_set);
    //     println!(
    //         "best is {:?} and optimal is {} and time is {:?}",
    //         solve.result, solve.data_set.optimal_value, solve.execution_time
    //     );
    // })

    // let a = solutions.len();
    // println!(
    //     "how many weights: {}, and how many items {}",
    //     solutions[99].len(),
    //     a
    // )
}

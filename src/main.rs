use std::cmp::max;
use std::fs;
use std::time::Instant;

use magisterka_projekt::knapsack::algorithms::dynamic_programming::dynamic_programming_knapsack;
use magisterka_projekt::knapsack::algorithms::fptas::fptas_knapsack;
use magisterka_projekt::knapsack::algorithms::greedy::greedy_algorithm;
use magisterka_projekt::knapsack::algorithms::types::SolveResult;
use magisterka_projekt::knapsack::helpers::helpers::{
    get_data_sets, get_solve_results, get_uncorrelated_data_set,
};
use magisterka_projekt::knapsack::parsers::parsers::{parse_entry, DataSet};

fn main() {
    // let data_sets = get_data_sets();
    // let data_set = data_sets.into_iter().nth(9).unwrap();
    //
    // let solve = fptas_knapsack(data_set, 0.05);
    // println!(
    //     "best: {:?} optimal: {},ratio: {}, time: {:?}, size: {}, capacity: {}, name {}",
    //     solve.result,
    //     solve.data_set.optimal_value,
    //     solve.ratio,
    //     solve.execution_time,
    //     solve.data_set.items_count,
    //     solve.data_set.capacity,
    //     solve.data_set.title
    // );

    let content = fs::read_to_string("./src/knapsack/datasets/knapPI_6_1000_1000.csv").unwrap();
    let files = content.split("-----").collect::<Vec<&str>>();
    let file = files[2];
    let entry = parse_entry(file).unwrap();
    let records = entry.records.clone();

    let result = fptas_knapsack(entry, 0.00001);
    println!("my result {} and ratio {}", result.result, result.ratio);

    // data_sets.into_iter().for_each(|data_set| {
    //     let solve = fptas_knapsack(data_set, 0.1);
    //     println!(
    //         "best: {:?} optimal: {},ratio: {}, time: {:?}, size: {}, capacity: {}, name {}",
    //         solve.result,
    //         solve.data_set.optimal_value,
    //         solve.ratio,
    //         solve.execution_time,
    //         solve.data_set.items_count,
    //         solve.data_set.capacity,
    //         solve.data_set.title
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

// let mut output = String::new();
// output.push_str("0.1\n");
// output.push_str(&format!("{}\n", entry.capacity));
//
// output.push_str(&format!("{}\n", records.len()));
//
// let weights = records
//     .iter()
//     .map(|record| record.weight)
//     .collect::<Vec<i64>>();
// let values = records
//     .iter()
//     .map(|record| record.value)
//     .collect::<Vec<i64>>();
//
// weights
//     .into_iter()
//     .for_each(|weight| output.push_str(&format!("{}\n", weight)));
// values
//     .into_iter()
//     .for_each(|val| output.push_str(&format!("{}\n", val)));
// fs::write("./src/output.txt", output).unwrap();

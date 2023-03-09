use std::time::Instant;

use crate::knapsack::algorithms::types::SolveResult;
use crate::knapsack::parsers::parsers::{DataSet, Record};

#[derive(Debug)]
struct Backpack {
    weight: i64,
    items: Vec<Record>,
}

pub fn greedy_algorithm(data_set: DataSet) -> SolveResult {
    let now = Instant::now();
    let DataSet {
        optimal_value,
        capacity,
        records,
        instance_type,
        ..
    } = data_set;

    let mut records_clone = records.to_vec();

    records_clone.sort_by(|a, b| {
        let val1 = a.value as f64 / a.weight as f64;
        let val2 = b.value as f64 / b.weight as f64;
        val2.partial_cmp(&val1).unwrap()
    });

    let mut backpack = Backpack {
        items: Vec::new(),
        weight: 0,
    };

    for elem in records_clone {
        if backpack.weight + elem.weight > capacity {
            continue;
        }
        backpack.weight = backpack.weight + elem.weight;
        backpack.items.push(elem);
    }
    let value = backpack.items.iter().map(|item| item.value).sum::<i64>();
    let ratio = value as f64 / optimal_value as f64;
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    SolveResult {
        result: value,
        optimal: optimal_value,
        ratio,
        instance_type,
    }
}

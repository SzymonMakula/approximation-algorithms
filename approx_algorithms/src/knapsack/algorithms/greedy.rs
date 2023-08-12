use std::time::Instant;

use crate::knapsack::algorithms::types::SolveResult;
use crate::knapsack::parsers::parsers::{DataSet, Record};

#[derive(Debug)]
struct Backpack {
    weight: i64,
    items: Vec<Record>,
    value: i64,
}

pub fn greedy_algorithm(data_set: DataSet) -> i64 {
    let mut records_clone = data_set.records.to_vec();

    records_clone.sort_by(|a, b| {
        let val1 = a.value as f64 / a.weight as f64;
        let val2 = b.value as f64 / b.weight as f64;
        val2.partial_cmp(&val1).unwrap()
    });

    let mut backpack = Backpack {
        items: Vec::new(),
        value: 0,
        weight: 0,
    };

    for elem in records_clone {
        if backpack.weight + elem.weight > data_set.capacity {
            if elem.value > backpack.value && elem.weight <= data_set.capacity {
                backpack.weight = elem.weight;
                backpack.value = elem.value;
                backpack.items = vec![elem];
            };
            break;
        }
        backpack.weight = backpack.weight + elem.weight;
        backpack.value = backpack.value + elem.value;
        backpack.items.push(elem);
    }

    let value = backpack.items.iter().map(|item| item.value).sum::<i64>();

    value
}

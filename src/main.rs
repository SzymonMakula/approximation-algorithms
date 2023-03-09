use std::collections::{vec_deque, HashMap};
use std::fs;
use std::io::Read;
use std::ops::Add;

use serde::Deserialize;

use magisterka_projekt::knapsack::algorithms::greedy::greedy_algorithm;
use magisterka_projekt::knapsack::algorithms::types::SolveResult;
use magisterka_projekt::knapsack::parsers::parsers::{parse_entry, DataSet, InstanceType, Record};

#[derive(Debug)]
struct Test {
    value: f64,
    instance_type: InstanceType,
}

fn main() {
    let folder = fs::read_dir("./src/knapsack/datasets").unwrap();

    let mut values: Vec<Test> = Vec::new();
    for file in folder {
        let path = file.unwrap().path();
        let contents = fs::read_to_string(path).unwrap();

        let files = contents.split("-----").collect::<Vec<&str>>();

        let data_sets = files
            .iter()
            .map(|&entry| parse_entry(entry))
            .filter_map(|entry| entry)
            .collect::<Vec<DataSet>>();

        let mut results = data_sets
            .into_iter()
            .map(|set| greedy_algorithm(set))
            .map(|result| Test {
                value: result.ratio,
                instance_type: result.instance_type,
            })
            .collect::<Vec<Test>>();
        values.append(&mut results);
    }
    values.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
    println!("{:?}", values)
}

// fn test() {
//     let mut reader = csv::Reader::from_path("./src/knapPI_1_100_1000_1").unwrap();
//     let mut data = reader
//         .deserialize::<Record>()
//         .map(|val| val.unwrap())
//         .collect::<Vec<Record>>();
//
//     data.sort_by(|a, b| {
//         let val1: f64 = (a.value as f64 / a.weight as f64) as f64;
//         let val2 = (b.value as f64 / b.weight as f64) as f64;
//         val2.partial_cmp(&val1).unwrap()
//     });
//     let mut content = String::new();
//     fs::File::open("./src/capacity.txt")
//         .unwrap()
//         .read_to_string(&mut content)
//         .unwrap();
//     let capacity: i64 = content.parse::<i64>().unwrap();
//
//     let mut backpack = Backpack {
//         items: Vec::new(),
//         weight: 0,
//     };
//
//     for elem in data {
//         if backpack.weight + elem.weight > capacity {
//             continue;
//         }
//         backpack.weight = backpack.weight + elem.weight;
//         backpack.items.push(elem);
//     }
//
//     let optimal = fs::read_to_string("./src/optimal.txt")
//         .unwrap()
//         .parse::<i64>()
//         .unwrap();
//     println!("capacity left: {:?}", capacity - backpack.weight);
//     let value = backpack.items.iter().map(|val| val.value).sum::<i64>();
//     println!(
//         "difference between optimal {:?}",
//         (value as f64 / optimal as f64)
//     );
// }

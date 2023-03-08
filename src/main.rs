use std::collections::{HashMap, vec_deque};
use std::fs;
use std::io::Read;
use std::ops::Add;

use serde::de::Unexpected::Str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    lo: i64,
    value: i64,
    weight: i64,
    take: i8,
}

#[derive(Debug)]
struct Backpack {
    weight: i64,
    items: Vec<Record>,
}

fn main() {
    let mut reader = csv::Reader::from_path("./src/knapPI_1_100_1000_1").unwrap();
    let mut data = reader.deserialize::<Record>().map(|val| {
        val.unwrap()
    }).collect::<Vec<Record>>();

    data.sort_by(|a, b| {
        let val1: f64 = (a.value as f64 / a.weight as f64) as f64;
        let val2 = (b.value as f64 / b.weight as f64) as f64;
        val2.partial_cmp(&val1).unwrap()
    });
    let mut content = String::new();
    fs::File::open("./src/capacity.txt").unwrap().read_to_string(&mut content).unwrap();
    let capacity: i64 = content.parse::<i64>().unwrap();

    let mut backpack = Backpack { items: Vec::new(), weight: 0 };

    for elem in data {
        if backpack.weight + elem.weight > capacity {
            continue;
        }
        backpack.weight = backpack.weight + elem.weight;
        backpack.items.push(elem);
    }

    let optimal = fs::read_to_string("./src/optimal.txt").unwrap().parse::<i64>().unwrap();
    println!("capacity left: {:?}", capacity - backpack.weight);
    let value = backpack.items.iter().map(|val| val.value).sum::<i64>();
    println!("difference between optimal {:?}", (value as f64 / optimal as f64));
}

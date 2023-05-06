use std::fs;

use crate::prim::prim::Matrix;

#[derive(Debug, Clone)]
pub struct DataSet {
    name: String,
    comment: String,
    dimension: i64,
    nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct Node {
    index: i64,
    x: i64,
    y: i64,
}

pub fn parse_data_set(data: &str) -> DataSet {
    let mut lines = data.lines();
    let name = lines.next().unwrap().to_owned();
    let comment = lines.next().unwrap().to_owned();
    let dimension = lines
        .nth(1)
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let mut nodes: Vec<Node> = vec![];

    lines.skip(2).for_each(|entry| {
        if entry.eq("EOF") {
            return;
        }
        let mut entry_iter = entry
            .split(" ")
            .into_iter()
            .map(|val| val.parse::<i64>().unwrap());
        let index = entry_iter.next().unwrap();
        let x = entry_iter.next().unwrap();
        let y = entry_iter.next().unwrap();

        let node = Node { index, x, y };
        nodes.push(node)
    });
    DataSet {
        dimension,
        name,
        comment,
        nodes,
    }
}

pub fn get_data_set() -> DataSet {
    let content = fs::read_to_string("./src/tsp/datasets/pr76.tsp").unwrap();
    parse_data_set(&content)
}

pub fn construct_adjacency_matrix(data_set: DataSet) -> Matrix {
    let mut matrix = vec![];
    let nodes = data_set.nodes.clone();
    data_set.nodes.iter().for_each(|node| {
        let mut list = vec![];
        let index = (node.index - 1) as usize;
        for target in &nodes {
            let xd = node.x - target.x;
            let yd = node.y - target.y;
            let distance = f64::ceil(((xd.pow(2) + yd.pow(2)) as f64).sqrt()) as i64;
            list.push(distance)
        }
        matrix.push(list)
    });
    matrix
}

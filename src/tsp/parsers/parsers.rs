use std::fs;

use crate::tsp::algorithms::prim::Matrix;

#[derive(Debug, Clone)]
pub struct DataSet {
    pub name: String,
    pub comment: String,
    pub dimension: i64,
    pub nodes: Vec<CityNode>,
    pub optimum: i64,
}

#[derive(Debug, Clone)]
pub struct CityNode {
    pub index: i64,
    pub x: i64,
    pub y: i64,
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

    let mut nodes: Vec<CityNode> = vec![];
    let optimum = lines.nth(2).unwrap().parse::<i64>().unwrap();

    lines.for_each(|entry| {
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

        let node = CityNode { index, x, y };
        nodes.push(node)
    });
    DataSet {
        optimum,
        dimension,
        name,
        comment,
        nodes,
    }
}

pub fn get_data_sets() -> Vec<DataSet> {
    let dir = fs::read_dir("./src/tsp/datasets").unwrap();
    dir.map(|entry| {
        let path = entry.unwrap().path();
        let content = fs::read_to_string(path).unwrap();
        parse_data_set(&content)
    })
    .collect::<Vec<DataSet>>()
}

pub fn get_data_set() -> DataSet {
    let content = fs::read_to_string("./src/tsp/datasets/pr107.tsp").unwrap();
    parse_data_set(&content)
}

pub fn construct_adjacency_matrix(data_set: &DataSet) -> Matrix {
    let mut matrix = vec![];
    let nodes = data_set.nodes.clone();
    data_set.nodes.iter().for_each(|node| {
        let mut list = vec![];
        for target in &nodes {
            let xd = node.x - target.x;
            let yd = node.y - target.y;
            let mut distance = f64::ceil(((xd.pow(2) + yd.pow(2)) as f64).sqrt()) as i64;
            if distance == 0 {
                distance = i64::MAX
            }
            list.push(distance)
        }
        matrix.push(list)
    });
    matrix
}

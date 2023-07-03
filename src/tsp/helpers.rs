use std::fs;
use std::time::Instant;

use crate::tsp::algorithms::approx_tsp_tour::approx_tsp_tour;
use crate::tsp::algorithms::christofides_algorithm::christofides_algorithm;
use crate::tsp::algorithms::prim::Node;
use crate::tsp::parsers::parsers::{construct_adjacency_matrix, get_data_sets};
use crate::tsp::result::{TspRunResult, TspSolveResult};

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub node: usize,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

pub fn map_nodes_to_tree(nodes: &Vec<Node>) -> Vec<TreeNode> {
    let mut tree: Vec<TreeNode> = vec![];
    for node in nodes {
        tree.push(TreeNode {
            node: node.index,
            parent: node.parent,
            children: vec![],
        })
    }
    for i in 0..nodes.len() {
        let node_parent = tree[i].parent.clone();
        if let Some(node_parent) = node_parent {
            tree[node_parent].children.push(i)
        }
    }
    tree
}

pub fn run_christofides() {
    let data_sets = get_data_sets();
    data_sets.into_iter().for_each(|set| {
        let mut tsp_runs = vec![];
        let runs_count = 100;

        for i in 0..runs_count {
            println!("### Running christofides {} {}/{}", set.name, i, runs_count);
            let now = Instant::now();
            let (res, tour) = christofides_algorithm(&set);
            let tsp_run = TspRunResult {
                result: res,
                tour,
                elapsed_micros: now.elapsed().as_micros(),
            };
            tsp_runs.push(tsp_run)
        }

        let path = format!("dist/tsp/christofides/{}.json", set.name);

        let tsp_solve_result = TspSolveResult {
            runs: runs_count,
            data_set: set,
            run_results: tsp_runs,
        };
        let json_string = serde_json::to_string(&tsp_solve_result).unwrap();

        fs::create_dir_all("dist/tsp/christofides").expect("Failed to create directories");
        fs::write(path, json_string).unwrap();
    });
}

pub fn run_approx_tsp() {
    let data_sets = get_data_sets();
    data_sets.into_iter().for_each(|set| {
        let mut tsp_runs = vec![];
        let runs_count = 100;

        for i in 0..runs_count {
            println!("### Running dtree {} {}/{}", set.name, i, runs_count);

            let now = Instant::now();
            let matrix = construct_adjacency_matrix(&set);
            let (res, tour) = approx_tsp_tour(matrix);
            let tsp_run = TspRunResult {
                result: res,
                tour,
                elapsed_micros: now.elapsed().as_micros(),
            };
            tsp_runs.push(tsp_run)
        }

        let path = format!("dist/tsp/dtree/{}.json", set.name);

        let tsp_solve_result = TspSolveResult {
            runs: runs_count,
            data_set: set,
            run_results: tsp_runs,
        };
        fs::create_dir_all("dist/tsp/dtree").expect("Failed to create directories");
        let json_string = serde_json::to_string(&tsp_solve_result).unwrap();

        std::fs::write(path, json_string).unwrap();
    });
}

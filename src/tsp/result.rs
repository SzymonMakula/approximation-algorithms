use serde::{Deserialize, Serialize};

use crate::tsp::parsers::parsers::DataSet;

#[derive(Serialize, Deserialize)]
pub struct TspRunResult {
    pub elapsed_micros: u128,
    pub result: i64,
    pub tour: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct TspSolveResult {
    pub runs: usize,
    pub data_set: DataSet,
    pub run_results: Vec<TspRunResult>,
}

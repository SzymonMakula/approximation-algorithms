use std::time::Duration;

use crate::knapsack::parsers::parsers::DataSet;

#[derive(Debug)]
pub struct SolveResult {
    pub result: i64,
    pub ratio: f64,
    pub data_set: DataSet,
    pub execution_time: Duration,
}

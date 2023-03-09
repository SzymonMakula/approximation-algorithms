use crate::knapsack::parsers::parsers::InstanceType;

#[derive(Debug)]
pub struct SolveResult {
    pub optimal: i64,
    pub result: i64,
    pub ratio: f64,
    pub instance_type: InstanceType,
}

use approx_algorithms::knapsack::algorithms::greedy::greedy_algorithm;
use approx_algorithms::knapsack::helpers::helpers::{
    run_dynamic_kp, run_dynamic_weight_kp, run_fptas_kp, run_greedy_kp,
};

fn main() {
    run_greedy_kp();
    run_fptas_kp(0.4);
    run_dynamic_kp();
    run_dynamic_weight_kp();
}

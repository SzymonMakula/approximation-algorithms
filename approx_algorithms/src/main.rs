use approx_algorithms::knapsack::algorithms::fptas::fptas_knapsack;
use approx_algorithms::knapsack::algorithms::greedy::greedy_knapsack;
use approx_algorithms::knapsack::helpers::helpers::{
    get_data_set, run_dynamic_kp, run_dynamic_weight_kp, run_fptas_kp, run_greedy_kp,
};

fn main() {
    run_greedy_kp();
    run_fptas_kp(0.1);
    run_dynamic_kp();
    run_dynamic_weight_kp();
}

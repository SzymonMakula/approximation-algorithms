use std::collections::HashSet;
use std::time::Instant;
use std::{fs, thread};

use magisterka_projekt::knapsack::algorithms::dynamic_programming::dynamic_programming_knapsack;
use magisterka_projekt::knapsack::algorithms::fptas::fptas_knapsack;
use magisterka_projekt::knapsack::algorithms::greedy::greedy_algorithm;
use magisterka_projekt::tsp::algorithms::approx_tsp_tour::approx_tsp_tour;
use magisterka_projekt::tsp::algorithms::christofides_algorithm::christofides_algorithm;
use magisterka_projekt::tsp::algorithms::munkers::munkers;
use magisterka_projekt::tsp::algorithms::prim::prim_algorithm;
use magisterka_projekt::tsp::helpers::{run_approx_tsp, run_christofides};
use magisterka_projekt::tsp::parsers::parsers::{
    construct_adjacency_matrix, get_data_set, get_data_sets,
};
use magisterka_projekt::tsp::result::{TspRunResult, TspSolveResult};

fn main() {
    run_christofides();
    run_approx_tsp();
}

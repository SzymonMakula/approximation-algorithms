use std::collections::HashMap;
use std::collections::VecDeque;

use crate::tsp::algorithms::approx_tsp_tour::sum_path;
use crate::tsp::algorithms::munkers::munkers;
use crate::tsp::algorithms::prim::{prim_algorithm, Matrix};
use crate::tsp::helpers::map_nodes_to_tree;
use crate::tsp::parsers::parsers::construct_adjacency_matrix;

#[derive(Debug)]
struct GraphNode {
    index: usize,
    adjacent: Vec<usize>,
}

type Graph = HashMap<usize, GraphNode>;

fn print_2d(input: Vec<Vec<i64>>) {
    for row in input {
        print!("{:?}\n", row)
    }
    println!("----")
}

pub fn christofides_algorithm(matrix: Matrix) -> i64 {
    let mst = prim_algorithm(matrix.clone(), 1);
    let tree = map_nodes_to_tree(&mst);
    let mut graph: Graph = HashMap::new();

    for node in tree {
        let mut graph_node = GraphNode {
            adjacent: node.children,
            index: node.node,
        };

        if node.parent.is_some() {
            graph_node.adjacent.push(node.parent.unwrap())
        }
        graph.insert(graph_node.index, graph_node);
    }

    println!("mst is {:?}", graph);

    let mut odd_indices = graph
        .iter()
        .filter(|(_, node)| node.adjacent.len() % 2 != 0)
        .map(|(i, node)| node.index)
        .collect::<Vec<usize>>();
    odd_indices.sort();

    let mut is_odd_index = vec![false; matrix.len()];
    for i in &odd_indices {
        is_odd_index[*i] = true
    }
    let mut secondary_matrix = vec![];

    for i in &odd_indices {
        let mut row = matrix[*i]
            .iter()
            .enumerate()
            .filter(|(i, item)| is_odd_index[*i])
            .map(|(i, val)| val.to_owned())
            .collect::<Vec<i64>>();

        secondary_matrix.push(row)
    }
    print_2d(matrix.clone());
    print_2d(secondary_matrix.clone());

    let output = munkers(secondary_matrix);
    println!("{:?}", output);

    // each record n[i] represent i-th node that is matched with n[i]-th node
    let mut perfect_min_match = vec![];
    for i in 0..output.len() {
        let row = &output[i];
        let (min_index, a) = row
            .iter()
            .enumerate()
            .max_by_key(|(i, &val)| val.to_owned())
            .unwrap();
        let pair = (odd_indices[i], odd_indices[min_index]);
        perfect_min_match.push(pair)
    }

    println!("odd indices {:?}", odd_indices);
    println!("perfect min match {:?}", perfect_min_match);
    println!("graph before {:?}", graph);

    for pair in perfect_min_match {
        let (incoming, outgoing) = pair;
        let adjacent_to_incoming = &mut graph.get_mut(&incoming).unwrap().adjacent;
        adjacent_to_incoming.push(outgoing);

        let adjacent_to_outgoing = &mut graph.get_mut(&outgoing).unwrap().adjacent;
        // if !adjacent_to_outgoing.contains(&incoming) {
        //     adjacent_to_outgoing.push(incoming)
        // }
    }
    println!("graph after {:?}", graph);
    let mut is_even = graph.iter().all(|(_, node)| node.adjacent.len() % 2 == 0);
    println!("is even? {}", is_even);
    let eulerian_circuit = hierholzer_algorithm(graph);
    let hamiltonian_circuit = shortcut_circuit(eulerian_circuit);
    let cost = calculate_cost(&matrix, hamiltonian_circuit);
    12
}

fn hierholzer_algorithm(mut graph: Graph) -> Vec<usize> {
    let mut stack = vec![];
    let mut visited_edges = vec![vec![false; graph.len().pow(2)]; graph.len().pow(2)];

    let mut tour = vec![];

    graph
        .iter()
        .for_each(|(u, node)| stack.push(node.index.to_owned()));

    let starting_node = stack.last().unwrap().to_owned();
    tour.push(starting_node);
    while !stack.is_empty() {
        let mut start = stack.last().unwrap().clone().to_owned();
        let is_complete = graph
            .get(&start)
            .unwrap()
            .adjacent
            .iter()
            .all(|out| visited_edges[start][*out] && visited_edges[*out][start]);
        if is_complete {
            stack.pop();
            continue;
        }

        let mut current = start;
        let mut sub_tour = vec![current];
        loop {
            let mut unvisited = usize::MAX;
            for adjacent in &graph.get(&current).unwrap().adjacent {
                if !visited_edges[current][*adjacent] && !visited_edges[*adjacent][current] {
                    unvisited = *adjacent;
                    visited_edges[current][*adjacent] = true;
                    visited_edges[*adjacent][current] = true;
                    break;
                }
            }

            sub_tour.push(unvisited);
            current = unvisited;

            if current == start {
                break;
            }
        }

        for i in 0..tour.len() {
            if tour[i] == sub_tour[0] {
                tour.splice(i..i + 1, sub_tour);
                break;
            }
        }
    }
    println!("the FINAL tour {:?} and stack {:?}", tour, stack);
    tour
}

fn shortcut_circuit(tour: Vec<usize>) -> Vec<usize> {
    print!("the tour is {:?}", tour);
    let mut visited = vec![false; tour.len()];
    let mut shorted = vec![];
    for stop in tour {
        if !visited[stop] {
            shorted.push(stop);
            visited[stop] = true
        }
    }
    shorted
}

fn calculate_cost(matrix: &Matrix, tour: Vec<usize>) -> i64 {
    let mut cost = 0;
    let start = tour[0];
    let mut previous_stop = start;
    for stop in tour.iter().skip(1) {
        print!(
            "stop {} and prev {} and cost {:?}\n",
            stop, previous_stop, matrix[previous_stop][*stop]
        );
        cost = cost + matrix[previous_stop][*stop];
        previous_stop = *stopz
    }
    cost = cost + matrix[previous_stop][start];

    cost
}

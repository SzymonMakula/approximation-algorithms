use std::collections::HashMap;
use std::collections::VecDeque;

use crate::tsp::algorithms::approx_tsp_tour::sum_path;
use crate::tsp::algorithms::munkers::munkers;
use crate::tsp::algorithms::prim::{prim_algorithm, Matrix};
use crate::tsp::helpers::map_nodes_to_tree;
use crate::tsp::parsers::parsers::construct_adjacency_matrix;

#[derive(Debug, Clone)]
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
    let mst = prim_algorithm(matrix.clone(), 0);
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
    println!("odd indices {:?}", odd_indices);
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
    let output = munkers(secondary_matrix);
    let mut matches = vec![];

    for (index, row) in output.iter().enumerate() {
        let mut match_pos = 0;
        for col in 0..row.len() {
            if row[col] == 1 {
                match_pos = col
            }
        }
        matches.push((index, match_pos))
    }
    println!("matches {:?}", matches);
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

    println!("perfect min match {:?}", perfect_min_match);
    println!("graph before {:?}", graph);

    for pair in perfect_min_match {
        let (incoming, outgoing) = pair;
        let adjacent_to_incoming = &mut graph.get_mut(&incoming).unwrap().adjacent;
        adjacent_to_incoming.push(outgoing);
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
    let mut tour = vec![];
    stack.push(graph.get(&0).unwrap().index);
    while !stack.is_empty() {
        let mut vertex_index = stack.last().unwrap();
        let vertex = graph.get_mut(&vertex_index).unwrap();
        if vertex.adjacent.is_empty() {
            tour.push(vertex.index);
            stack.pop();
        } else {
            let index = vertex.adjacent.pop().unwrap();
            let outgoing_vertex = graph.get_mut(&index).unwrap();

            // println!("\n the ingoing vertex {:?}", vertex_index);
            // println!("\n the outgoing vertex {:?}\n", outgoing_vertex);
            let element_pos = outgoing_vertex
                .adjacent
                .iter()
                .position(|el| el == vertex_index)
                .unwrap();
            outgoing_vertex.adjacent.swap_remove(element_pos);
            stack.push(index);
        }
    }
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
        cost = cost + matrix[previous_stop][*stop];
        previous_stop = *stop
    }
    cost = cost + matrix[previous_stop][start];

    cost
}

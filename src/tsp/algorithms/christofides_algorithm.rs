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

// let mut graph_2 = HashMap::new();
// graph_2.insert(
// 0,
// GraphNode {
// index: 0,
// adjacent: vec![1, 2, 4, 5],
// },
// );
// graph_2.insert(
// 1,
// GraphNode {
// index: 1,
// adjacent: vec![0, 2],
// },
// );
// graph_2.insert(
// 2,
// GraphNode {
// index: 2,
// adjacent: vec![1, 3, 0, 4],
// },
// );
// graph_2.insert(
// 3,
// GraphNode {
// index: 3,
// adjacent: vec![2, 4],
// },
// );
// graph_2.insert(
// 4,
// GraphNode {
// index: 4,
// adjacent: vec![5, 0, 2, 3],
// },
// );
// graph_2.insert(
// 5,
// GraphNode {
// index: 5,
// adjacent: vec![0, 4],
// },
// );
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

    let odd_indices = graph
        .iter()
        .filter(|(_, node)| node.adjacent.len() % 2 != 0)
        .map(|(i, node)| node.index)
        .collect::<Vec<usize>>();

    println!("the odd indices are {:?}", odd_indices);
    println!("matrix {:?}", matrix[odd_indices[0]]);
    let formatted = matrix[odd_indices[0]]
        .iter()
        .enumerate()
        .filter(|(i, item)| odd_indices.contains(i))
        .map(|(i, val)| val.to_owned())
        .collect::<Vec<i64>>();
    println!("after filter {:?}\n", formatted);

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
    println!("{:?}", secondary_matrix);

    let output = munkers(secondary_matrix);
    println!("{:?}", output);

    // each record n[i] represent i-th node that is matched with n[i]-th node
    let mut perfect_min_match = vec![];
    for i in 0..output.len() {
        let (min_index, _) = output[i]
            .iter()
            .enumerate()
            .max_by_key(|(index, value)| value.to_owned())
            .unwrap();
        perfect_min_match.push((odd_indices[i], odd_indices[min_index]));
    }

    // println!("perfect min match {:?}", perfect_min_match);
    // println!("graph before {:?}", graph);

    for pair in perfect_min_match {
        let (incoming, outgoing) = pair;
        graph.get_mut(&incoming).unwrap().adjacent.push(outgoing);
    }
    // println!("graph after {:?}", graph);

    let eulerian_circuit = hierholzer_algorithm(graph);
    let hamiltonian_circuit = shortcut_circuit(eulerian_circuit);
    let cost = calculate_cost(&matrix, hamiltonian_circuit);
    cost
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
            // println!("visiting {}", current);
            for adjacent in &graph.get(&current).unwrap().adjacent {
                // println!(
                //     "current node {:?}, was it visited {} {}",
                //     graph.get(&current).unwrap(),
                //     visited_edges[current][*adjacent],
                //     visited_edges[*adjacent][current]
                // );
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
        previous_stop = *stop
    }
    cost = cost + matrix[previous_stop][start];

    cost
}

use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Stdin;
use std::ops::Index;
use std::os::unix::process::parent_id;
use std::process::{Command, Stdio};

use crate::tsp::algorithms::approx_tsp_tour::create_tour;
use crate::tsp::algorithms::munkers::munkers;
use crate::tsp::algorithms::prim::{prim_algorithm, Matrix};
use crate::tsp::helpers::{map_nodes_to_tree, TreeNode};
use crate::tsp::parsers::parsers::{construct_adjacency_matrix, parse_data_set, CityNode, DataSet};

#[derive(Debug, Clone)]
struct GraphNode {
    index: usize,
    adjacent: Vec<usize>,
}

type Graph = HashMap<usize, GraphNode>;

fn map_tree_to_graph(tree: Vec<TreeNode>) -> Graph {
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
    graph
}

fn get_min_perfect_matching(graph: &Graph, data_set: &DataSet) -> Vec<(usize, usize)> {
    let odd_indices = graph
        .iter()
        .filter(|(_, node)| node.adjacent.len() % 2 != 0)
        .map(|(i, node)| node.index)
        .collect::<Vec<usize>>();

    // HashMap<Mapped, Original>
    // Indexing by TSPLIB format, indexing starts at 1.
    let mut indices_mapping = HashMap::new();

    let nodes_to_map = odd_indices
        .into_iter()
        .map(|index| data_set.nodes.get(index).unwrap().to_owned())
        .collect::<Vec<CityNode>>();

    nodes_to_map
        .iter()
        .enumerate()
        .for_each(|(index, city_node)| {
            indices_mapping.insert(index + 1, (city_node.index) as usize);
        });

    let mut tsp_subproblem = String::new();
    tsp_subproblem.push_str("NAME : subproblem\n");
    tsp_subproblem.push_str("COMMENT: NA\n");
    tsp_subproblem.push_str("TYPE : TSP\n");
    tsp_subproblem.push_str(&format!("DIMENSION : {}\n", nodes_to_map.len().to_string()));
    tsp_subproblem.push_str("EDGE_WEIGHT_TYPE : EUC_2D");
    tsp_subproblem.push_str("NODE_COORD_SECTION\n");

    nodes_to_map
        .into_iter()
        .enumerate()
        .for_each(|(index, city_node)| {
            let index_string = (index + 1).to_string();
            let x_string = city_node.x.to_string();
            let y_string = city_node.y.to_string();
            let content = format!("{} {} {}\n", index_string, x_string, y_string);
            tsp_subproblem.push_str(&content)
        });
    tsp_subproblem.push_str("EOF\n");

    let mut content_pipe = Command::new("echo");
    content_pipe.arg(tsp_subproblem);
    let content_pipe_child = content_pipe
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let invoke_blossom = Command::new("lib/blossom5/blossom5")
        .arg("-g")
        .arg("/dev/stdin")
        .arg("-D")
        .arg("-V")
        .arg("-w")
        .arg("./tmp/blossom_out.txt")
        .stdin(Stdio::from(content_pipe_child.stdout.unwrap()))
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to execute process")
        .wait_with_output()
        .expect("failed to execute process");

    let blossom_output = std::fs::read_to_string("tmp/blossom_out.txt").unwrap();
    let pairs = blossom_output
        .lines()
        .skip(1)
        .map(|line| {
            let mut elems = line.split_whitespace();
            let ingoing = elems.next().unwrap().parse::<usize>().unwrap();
            let outgoing = elems.next().unwrap().parse::<usize>().unwrap();
            // Output indexing starts at 0
            (ingoing + 1, outgoing + 1)
        })
        .collect::<Vec<(usize, usize)>>();

    let original_pairs = pairs
        .into_iter()
        .map(|(ingoing, outgoing)| {
            let original_ingoing = indices_mapping.get(&ingoing).unwrap().to_owned() - 1;
            let original_outgoing = indices_mapping.get(&outgoing).unwrap().to_owned() - 1;
            (original_ingoing, original_outgoing)
        })
        .collect::<Vec<(usize, usize)>>();

    original_pairs
}

fn add_edges_to_graph(pairs: Vec<(usize, usize)>, graph: &mut Graph) {
    for pair in pairs {
        let (incoming, outgoing) = pair;
        let adjacent_to_incoming = &mut graph.get_mut(&incoming).unwrap().adjacent;
        adjacent_to_incoming.push(outgoing);
        let adjacent_to_outgoing = &mut graph.get_mut(&outgoing).unwrap().adjacent;
        adjacent_to_outgoing.push(incoming);
    }
}

pub fn christofides_algorithm(data_set: &DataSet) -> (i64, Vec<usize>) {
    let matrix = construct_adjacency_matrix(&data_set);
    let mst = prim_algorithm(matrix.clone(), 0);
    let tree = map_nodes_to_tree(&mst);
    let mut graph = map_tree_to_graph(tree);
    let pairs = get_min_perfect_matching(&graph, &data_set);
    add_edges_to_graph(pairs, &mut graph);

    let eulerian_circuit = hierholzer_algorithm(graph);
    let hamiltonian_circuit = shortcut_circuit(eulerian_circuit);
    let cost = calculate_cost(&matrix, &hamiltonian_circuit);
    let mut tour = create_tour(hamiltonian_circuit);
    (cost, tour)
}

fn hierholzer_algorithm(mut graph: Graph) -> Vec<usize> {
    let mut stack = vec![];
    let mut tour = vec![];
    let first_index = graph.get(&0).unwrap().index;
    stack.push(first_index);
    while !stack.is_empty() {
        let mut vertex_index = stack.last().unwrap();
        let vertex = graph.get_mut(&vertex_index).unwrap();
        if vertex.adjacent.is_empty() {
            tour.push(vertex.index);
            stack.pop();
        } else {
            let index = vertex.adjacent.pop().unwrap();
            let outgoing_vertex = graph.get_mut(&index).unwrap();
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

pub fn calculate_cost(matrix: &Matrix, tour: &Vec<usize>) -> i64 {
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

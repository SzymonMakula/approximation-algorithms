pub type Matrix = Vec<Vec<i64>>;

#[derive(Debug, Clone)]
pub struct Node {
    pub key: i64,
    pub parent: Option<usize>,
    is_in_mst: bool,
    pub index: usize,
}

pub fn prim_algorithm(matrix: Matrix, root: usize) -> Vec<Node> {
    let mut nodes: Vec<Node> = vec![];
    matrix[0].iter().enumerate().for_each(|(index, _)| {
        nodes.push(Node {
            key: i64::MAX,
            index,
            is_in_mst: false,
            parent: None,
        });
    });
    nodes[root].key = 0;
    let nodes_count = nodes.len();

    for _ in 0..nodes_count {
        let min_value_vertex = extract_min(&nodes);
        nodes[min_value_vertex].is_in_mst = true;

        for mut vertex in 0..nodes.len() {
            if matrix[min_value_vertex][vertex] != 0
                && !nodes[vertex].is_in_mst
                && matrix[min_value_vertex][vertex] < nodes[vertex].key
            {
                nodes[vertex].parent = Some(min_value_vertex);
                nodes[vertex].key = matrix[min_value_vertex][vertex];
            }
        }
    }
    nodes
}

fn extract_min(nodes: &Vec<Node>) -> usize {
    let mut min = i64::MAX;
    let mut lowest_index = 0;
    let mut index = 0;
    for node in nodes {
        if !node.is_in_mst && node.key < min {
            min = node.key;
            lowest_index = index;
        }
        index += 1;
    }
    lowest_index
}

pub fn print_mst(nodes: &Vec<Node>) {
    print!("\nEdge \t         Weight\n");
    nodes.iter().for_each(|node| {
        println!("{:?} <--> {} \t {}", node.parent, node.index, node.key);
    });
}

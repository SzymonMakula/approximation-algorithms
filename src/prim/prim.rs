type Matrix = Vec<Vec<i64>>;

pub fn prim_algorithm(G: Matrix, root: i64) -> i64 {
    1
}

#[derive(Debug, Clone)]
struct Node {
    key: i64,
    parent: Option<i64>,
    isInMST: bool,
    index: i64,
}

pub fn run_prim() {
    let matrix: Matrix = vec![
        vec![0, 0, 3, 0, 0],
        vec![0, 0, 10, 4, 0],
        vec![3, 10, 0, 2, 6],
        vec![0, 4, 2, 0, 1],
        vec![0, 0, 6, 1, 0],
    ];

    let mut nodes: Vec<Node> = vec![];

    let mut i = 0;
    for vertex in &matrix[0] {
        nodes.push(Node {
            key: i64::MAX,
            index: i,
            isInMST: false,
            parent: None,
        });
        i += 1;
    }
    nodes[0].key = -1;

    while !is_queue_empty(&nodes) {
        let min_value_vertex = extract_min(&nodes) as usize;
        nodes[min_value_vertex].isInMST = true;

        for mut vertex in 0..nodes.len() {
            let vertex = vertex.to_owned() as usize;
            if matrix[min_value_vertex][vertex] != 0
                && !nodes[vertex].isInMST
                && matrix[min_value_vertex][vertex] < nodes[vertex].key
            {
                nodes[vertex].parent = Some(min_value_vertex as i64);
                nodes[vertex].key = matrix[min_value_vertex][vertex];
            }
        }
    }
    print!("\nEdge \t Weight \n");
    nodes.iter().for_each(|node| {
        println!("{:?} <--> {} \t {}", node.parent, node.index, node.key);
    });
}

fn is_queue_empty(queue: &Vec<Node>) -> bool {
    queue.iter().all(|node| node.isInMST)
}

fn extract_min(queue: &Vec<Node>) -> i64 {
    let mut min = i64::MAX;
    let mut lowest_index = 0;
    let mut index = 0;
    for node in queue {
        if !node.isInMST && node.key < min {
            min = node.key;
            lowest_index = index;
        }
        index += 1;
    }
    lowest_index
}

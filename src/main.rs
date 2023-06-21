use std::time::Instant;

use magisterka_projekt::knapsack::algorithms::dynamic_programming::dynamic_programming_knapsack;
use magisterka_projekt::knapsack::algorithms::fptas::fptas_knapsack;
use magisterka_projekt::knapsack::algorithms::greedy::greedy_algorithm;
use magisterka_projekt::tsp::algorithms::approx_tsp_tour::approx_tsp_tour;
use magisterka_projekt::tsp::algorithms::christofides_algorithm::christofides_algorithm;
use magisterka_projekt::tsp::algorithms::munkers::munkers;
use magisterka_projekt::tsp::algorithms::prim::prim_algorithm;
use magisterka_projekt::tsp::parsers::parsers::{
    construct_adjacency_matrix, get_data_set, get_data_sets,
};

fn main() {
    // christofides_algorithm(test)
    read_tsp_set()
}

fn read_tsp_set() {
    let data_set = get_data_set();
    let matrix = construct_adjacency_matrix(&data_set);

    let test_set = vec![
        vec![
            i64::MAX,
            633,
            257,
            91,
            412,
            150,
            80,
            134,
            259,
            505,
            353,
            324,
            70,
            211,
            268,
            246,
            121,
        ],
        vec![
            633,
            i64::MAX,
            390,
            661,
            227,
            488,
            572,
            530,
            555,
            289,
            282,
            638,
            567,
            466,
            420,
            745,
            518,
        ],
        vec![
            257,
            390,
            i64::MAX,
            228,
            169,
            112,
            196,
            154,
            372,
            262,
            110,
            437,
            191,
            74,
            53,
            472,
            142,
        ],
        vec![
            91,
            661,
            228,
            i64::MAX,
            383,
            120,
            77,
            105,
            175,
            476,
            324,
            240,
            27,
            182,
            239,
            237,
            84,
        ],
        vec![
            412,
            227,
            169,
            383,
            i64::MAX,
            267,
            351,
            309,
            338,
            196,
            61,
            421,
            346,
            243,
            199,
            528,
            297,
        ],
        vec![
            150,
            488,
            112,
            120,
            267,
            i64::MAX,
            63,
            34,
            264,
            360,
            208,
            329,
            83,
            105,
            123,
            364,
            35,
        ],
        vec![
            80,
            572,
            196,
            77,
            351,
            63,
            i64::MAX,
            29,
            232,
            444,
            292,
            297,
            47,
            150,
            207,
            332,
            29,
        ],
        vec![
            134,
            530,
            154,
            105,
            309,
            34,
            29,
            i64::MAX,
            249,
            402,
            250,
            314,
            68,
            108,
            165,
            349,
            36,
        ],
        vec![
            259,
            555,
            372,
            175,
            338,
            264,
            232,
            249,
            i64::MAX,
            495,
            352,
            95,
            189,
            326,
            383,
            202,
            236,
        ],
        vec![
            505,
            289,
            262,
            476,
            196,
            360,
            444,
            402,
            495,
            i64::MAX,
            154,
            578,
            439,
            336,
            240,
            685,
            390,
        ],
        vec![
            353,
            282,
            110,
            324,
            61,
            208,
            292,
            250,
            352,
            154,
            i64::MAX,
            435,
            287,
            184,
            140,
            542,
            238,
        ],
        vec![
            324,
            638,
            437,
            240,
            421,
            329,
            297,
            314,
            95,
            578,
            435,
            i64::MAX,
            254,
            391,
            448,
            157,
            301,
        ],
        vec![
            70,
            567,
            191,
            27,
            346,
            83,
            47,
            68,
            189,
            439,
            287,
            254,
            i64::MAX,
            145,
            202,
            289,
            55,
        ],
        vec![
            211,
            466,
            74,
            182,
            243,
            105,
            150,
            108,
            326,
            336,
            184,
            391,
            145,
            i64::MAX,
            57,
            426,
            96,
        ],
        vec![
            268,
            420,
            53,
            239,
            199,
            123,
            207,
            165,
            383,
            240,
            140,
            448,
            202,
            57,
            i64::MAX,
            483,
            153,
        ],
        vec![
            246,
            745,
            472,
            237,
            528,
            364,
            332,
            349,
            202,
            685,
            542,
            157,
            289,
            426,
            483,
            i64::MAX,
            336,
        ],
        vec![
            121,
            518,
            142,
            84,
            297,
            35,
            29,
            36,
            236,
            390,
            238,
            301,
            55,
            96,
            153,
            336,
            i64::MAX,
        ],
    ];

    let dist_matrix = vec![
        vec![i64::MAX, 12, 29, 22, 13, 24],
        vec![12, i64::MAX, 19, 3, 25, 6],
        vec![29, 19, i64::MAX, 21, 23, 28],
        vec![22, 3, 21, i64::MAX, 4, 5],
        vec![13, 25, 23, 4, i64::MAX, 16],
        vec![24, 6, 28, 5, 16, i64::MAX],
    ];

    let adjacency_matrix = vec![
        vec![i64::MAX, 1, 2, 1, 1],
        vec![1, i64::MAX, 1, 2, 1],
        vec![2, 1, i64::MAX, 1, 1],
        vec![1, 2, 1, i64::MAX, 1],
        vec![1, 1, 1, 1, i64::MAX],
    ];

    let mst = christofides_algorithm(adjacency_matrix);
}

// fn read_kp_set() {
//     let data_set = get_data_set("/home/szymon/FunProjects/magisterka/magisterka-projekt/src/knapsack/datasets/knapPI_3_100_10000.csv");
//     for set in data_set {
//         let result = dynamic_programming_knapsack(set);
//         println!(
//             "ratio: {:.6} | time {:?} | capacity: {} | max profit: {} | items count {} ",
//             result.ratio,
//             result.execution_time,
//             result.data_set.capacity,
//             result
//                 .data_set
//                 .records
//                 .iter()
//                 .map(|record| record.value)
//                 .sum::<i64>(),
//             result.data_set.records.len()
//         )
//     }
// }

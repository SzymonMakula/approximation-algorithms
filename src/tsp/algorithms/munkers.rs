use std::ptr::drop_in_place;

use crate::tsp::algorithms::munkers::Step::{Step1, Step3, Step4};
use crate::tsp::algorithms::prim::Matrix;

enum Step {
    Step1,
    Step2,
    Step3,
    Step4,
    Step5,
    Step6,
    Done,
}

fn print_as_2d(matrix: Vec<i64>, dimension: usize) {
    println!("---Matrix------");
    for i in 0..dimension {
        let mut row = vec![];
        for j in 0..dimension {
            row.push(matrix.get(i * dimension + j).unwrap().to_owned())
        }
        println!("{:?}", row)
    }
    println!("---Matrix END---\n");
}

pub fn munkers(matrix: Matrix) {
    let mut step: Step = Step::Step1;

    let dimension = matrix.len();
    let mut costs = matrix.clone().into_iter().flatten().collect::<Vec<i64>>();
    let get_index = |row: usize, col: usize| -> usize { row * dimension + col };

    print_as_2d(costs.clone(), dimension);

    // covered = 1
    let mut covered_rows = vec![0; dimension];
    let mut covered_cols = vec![0; dimension];

    // 1 = starred, 2 = primed
    let mut markings = vec![0; dimension * dimension];

    let mut done = false;
    while done != true {
        match step {
            Step::Step1 => {
                for i in 0..dimension {
                    let mut row = vec![];
                    for j in 0..dimension {
                        let index = get_index(i, j);
                        row.push(costs.get(index).unwrap().to_owned())
                    }

                    let min = row.iter().min().unwrap().to_owned();

                    for j in 0..dimension {
                        let index = get_index(i, j);
                        costs[index] = row[j] - min
                    }
                }
                step = Step::Step2
            }
            Step::Step2 => {
                for i in 0..dimension {
                    for j in 0..dimension {
                        let index = get_index(i, j);
                        let element = costs.get(index).unwrap().to_owned();
                        if element == 0 && covered_rows[i] == 0 && covered_cols[j] == 0 {
                            markings[index] = 1;
                            covered_cols[j] = 1;
                            covered_rows[i] = 1;
                        }
                    }
                }
                for i in 0..dimension {
                    covered_cols[i] = 0;
                    covered_rows[i] = 0;
                }
                step = Step::Step3
            }
            Step::Step3 => {
                for i in 0..dimension {
                    for j in 0..dimension {
                        let index = get_index(i, j);
                        if markings[index] == 1 {
                            covered_cols[j] = 1
                        }
                    }
                }
                println!("{:?}", covered_rows);
                println!("{:?}", covered_cols);

                let covered_cols_len = covered_cols
                    .clone()
                    .into_iter()
                    .filter(|&col| col == 1)
                    .collect::<Vec<i32>>()
                    .len();

                match covered_cols_len == dimension {
                    true => step = Step::Done,
                    false => step = Step::Step4,
                }
            }
            Step::Step4 => {
                let mut find_starred_zero = || {
                    let mut done = false;
                    let mut i = 0;
                    let mut row: i64 = -1;
                    let mut col: i64 = -1;

                    while done != true {
                        let mut j = 0;
                        while j < dimension {
                            let index = get_index(i, j);
                            if costs[index] == 0 && covered_rows[i] == 0 && covered_cols[j] == 0 {
                                row = i as i64;
                                col = j as i64;
                                done = true;
                                return (row, col);
                            }
                            j = j + 1;
                        }
                        i = i + 1;
                        if i >= dimension {
                            done = true
                        }
                    }
                    return (row, col);
                };

                let is_star_in_row = |row: usize| {
                    for j in 0..dimension {
                        let index = get_index(row, j);
                        if markings[index] == 1 {
                            return true;
                        }
                    }
                    return false;
                };

                let mut done = false;
                while done != true {
                    let (row, col) = find_starred_zero();
                    println!("{}", row);

                    if row == -1 {
                        done = true;
                        step = Step::Step6;
                    } else {
                        let index = get_index(row as usize, col as usize);
                        markings[index] = 2;
                        if is_star_in_row(row as usize) {
                            covered_rows[row as usize] = 1;
                            covered_cols[col as usize] = 0;
                        } else {
                            done = true;
                            step = Step::Step5;
                        }
                    }
                }
            }
            Step::Step5 => {}
            Step::Step6 => {
                print_as_2d(costs.clone(), dimension);

                let find_smallest = || {
                    let mut min_val = i64::MAX;
                    for i in 0..dimension {
                        for j in 0..dimension {
                            let index = get_index(i, j);
                            if covered_rows[i] == 0 && covered_cols[j] == 0 {
                                if min_val > costs[index] {
                                    min_val = costs[index]
                                }
                            }
                        }
                    }
                    min_val
                };
                let smallest_uncovered = find_smallest();
                println!("{}", smallest_uncovered);
                for i in 0..dimension {
                    for j in 0..dimension {
                        let index = get_index(i, j);

                        if covered_rows[i] == 1 {
                            costs[index] = costs[index] + smallest_uncovered;
                        }
                        if covered_cols[j] == 0 {
                            costs[index] = costs[index] - smallest_uncovered;
                        }
                    }
                }

                step = Step::Step4;
            }
            Step::Done => {
                print_as_2d(costs.clone(), dimension);
                done = true
            }
        }
    }
}

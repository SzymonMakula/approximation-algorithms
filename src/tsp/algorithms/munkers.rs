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

pub fn munkers(matrix: Matrix) -> Matrix {
    let mut step: Step = Step::Step1;
    let mut output: Matrix = vec![];

    let dimension = matrix.len();
    let mut costs = matrix.clone().into_iter().flatten().collect::<Vec<i64>>();
    let get_index = |row: usize, col: usize| -> usize { row * dimension + col };

    // covered = 1
    let mut covered_rows = vec![0; dimension];
    let mut covered_cols = vec![0; dimension];

    // 1 = starred, 2 = primed
    let mut markings = vec![0; dimension * dimension];

    let mut z0_row = 0;
    let mut z0_column = 0;

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
                let mut done = false;
                while done != true {
                    let mut find_starred_zero = || {
                        let mut done = false;
                        let mut i = 0;

                        while done != true {
                            let mut j = 0;
                            while j < dimension {
                                let index = get_index(i, j);
                                if costs[index] == 0 && covered_rows[i] == 0 && covered_cols[j] == 0
                                {
                                    let row = i;
                                    let col = j;
                                    done = true;
                                    return Some((row, col));
                                }
                                j = j + 1;
                            }
                            i = i + 1;
                            if i >= dimension {
                                done = true
                            }
                        }
                        return None;
                    };

                    let is_star_in_row = |row: usize, markings: &Vec<i32>| {
                        for j in 0..dimension {
                            let index = get_index(row, j);
                            if markings[index] == 1 {
                                return true;
                            }
                        }
                        return false;
                    };

                    let find_col_of_star = |row: usize, markings: &Vec<i32>| {
                        for j in 0..dimension {
                            let index = get_index(row, j);
                            if markings[index] == 1 {
                                return j;
                            }
                        }
                        panic!("No column with star found")
                    };

                    let starred_zero = find_starred_zero();

                    if starred_zero.is_none() {
                        done = true;
                        step = Step::Step6;
                    } else {
                        let (row, col) = starred_zero.unwrap();
                        let index = get_index(row, col);
                        markings[index] = 2;
                        if is_star_in_row(row as usize, &markings) {
                            let col = find_col_of_star(row, &markings);
                            covered_rows[row] = 1;
                            covered_cols[col] = 0;
                        } else {
                            done = true;
                            z0_row = row;
                            // To moze nie byc poprawny col ^ zobacz wyzej jak go wyznaczasz w if
                            z0_column = col;
                            step = Step::Step5;
                        }
                    }
                }
            }
            Step::Step5 => {
                let col_count = 2;
                let find_row_of_star = |col: usize, markings: &Vec<i32>| {
                    for j in 0..dimension {
                        let index = get_index(j, col);
                        if markings[index] == 1 {
                            return Some(j);
                        }
                    }
                    None
                };
                let find_col_of_prime = |row: usize, markings: &Vec<i32>| {
                    for j in 0..dimension {
                        let index = get_index(row, j);
                        if markings[index] == 2 {
                            return j;
                        }
                    }
                    panic!("No column found for prime")
                };

                let convert_path = |markings: &mut Vec<i32>, path: &Vec<usize>, count: usize| {
                    for i in 0..count + 1 {
                        let path_index_1 = i * col_count + 0;
                        let path_index_2 = i * col_count + 1;

                        let index = get_index(path[path_index_1], path[path_index_2]);
                        if markings[index] == 1 {
                            markings[index] = 0
                        } else {
                            markings[index] = 1
                        }
                    }
                };

                let clear_covers = |covered_cols: &mut Vec<i32>, covered_rows: &mut Vec<i32>| {
                    for i in 0..dimension {
                        covered_cols[i] = 0;
                        covered_rows[i] = 0;
                    }
                };

                let erase_primes = |markings: &mut Vec<i32>| {
                    for i in 0..dimension {
                        for j in 0..dimension {
                            let index = get_index(i, j);
                            if markings[index] == 2 {
                                markings[index] = 0
                            }
                        }
                    }
                };

                let mut count = 0;
                let mut path: Vec<usize> = vec![0; dimension * dimension + 1];
                let mut done = false;
                let path_index_1 = count * 2 + 0;
                let path_index_2 = count * 2 + 1;
                path[path_index_1] = z0_row;
                path[path_index_2] = z0_column;

                while !done {
                    let path_index = count * 2 + 1;
                    let row_with_star = find_row_of_star(path[path_index], &markings);

                    if row_with_star.is_some() {
                        count = count + 1;
                        let path_index_1 = count * 2 + 0;

                        path[path_index_1] = row_with_star.unwrap();
                        let path_index_2 = count * 2 + 1;
                        let path_index_3 = (count - 1) * 2 + 1;

                        path[path_index_2] = path[path_index_3]
                    } else {
                        done = true
                    }
                    if !done {
                        let mut path_index_1 = count * 2 + 0;
                        let prime_col = find_col_of_prime(path[path_index_1], &markings);

                        count = count + 1;
                        path_index_1 = count * 2 + 0;
                        let path_index_2 = count * 2 + 1;
                        let path_index_3 = (count - 1) * 2 + 0;

                        path[path_index_1] = path[path_index_3];
                        path[path_index_2] = prime_col;
                    }
                }
                convert_path(&mut markings, &path, count);
                clear_covers(&mut covered_cols, &mut covered_rows);
                erase_primes(&mut markings);

                step = Step::Step3;
            }
            Step::Step6 => {
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
                for i in 0..dimension {
                    let mut row: Vec<i64> = vec![];
                    for j in 0..dimension {
                        let index = get_index(i, j);
                        row.push(markings[index] as i64)
                    }
                    output.push(row)
                }
                done = true;
            }
        }
    }
    return output;
}

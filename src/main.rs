use std::cmp::Ordering;
use std::collections::{vec_deque, HashMap};
use std::fs;
use std::io::Read;
use std::ops::Add;

use charts::{Chart, ScaleBand, ScaleLinear, VerticalBarView};
use plotters::prelude::*;
use plotters::style::full_palette::{BLUE_100, GREY, ORANGE};

use magisterka_projekt::knapsack::algorithms::greedy::greedy_algorithm;
use magisterka_projekt::knapsack::algorithms::types::SolveResult;
use magisterka_projekt::knapsack::parsers::parsers::{parse_entry, DataSet, InstanceType, Record};

fn get_instance_average(results: &Vec<SolveResult>, instance_type: InstanceType) -> f64 {
    let uncorrelated_results_iter = results
        .iter()
        .filter(|result| matches!(&result.data_set.instance_type, instance_type))
        .map(|result| result.ratio);
    let len = uncorrelated_results_iter
        .clone()
        .collect::<Vec<f64>>()
        .len();
    uncorrelated_results_iter.sum::<f64>() / len as f64
}

fn main() {
    let values = get_solve_results();

    let root_area = BitMapBackend::new("src/2.7.png", (800, 600)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let total_len = values.len();
    let mut ratios = values
        .iter()
        .map(|x| {
            let values_before = values
                .iter()
                .map(|result| result.ratio)
                .filter(|ratio| {
                    let ratio = ratio.partial_cmp(&x.ratio).unwrap();
                    match ratio {
                        Ordering::Less => true,
                        Ordering::Equal => true,
                        Ordering::Greater => false,
                    }
                })
                .collect::<Vec<f64>>();

            let percentage = values_before.len() as f64 / total_len as f64;
            (percentage, x.ratio)
        })
        .collect::<Vec<(f64, f64)>>();

    ratios
        .sort_by(|(a_percentage), (b_percentage)| a_percentage.partial_cmp(&b_percentage).unwrap());

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Scatter Demo", ("sans-serif", 40))
        .build_cartesian_2d(0f64..1f64, 0.4f64..1f64)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let total_len = values.len() as f64;

    ctx.draw_series(values.iter().map(|x| {
        let values_before = values
            .iter()
            .map(|result| result.ratio)
            .filter(|ratio| {
                let ratio = ratio.partial_cmp(&x.ratio).unwrap();
                match ratio {
                    Ordering::Less => true,
                    Ordering::Equal => true,
                    Ordering::Greater => false,
                }
            })
            .collect::<Vec<f64>>();

        let percentage = values_before.len() as f64 / total_len;
        println!("{}", percentage);
        Circle::new((percentage, x.ratio), 1, BLUE)
    }))
    .unwrap();
}

fn get_solve_results() -> Vec<SolveResult> {
    let folder = fs::read_dir("./src/knapsack/datasets").unwrap();

    let mut values: Vec<SolveResult> = Vec::new();
    for file in folder {
        let path = file.unwrap().path();
        let contents = fs::read_to_string(path).unwrap();

        let files = contents.split("-----").collect::<Vec<&str>>();

        let data_sets = files
            .iter()
            .map(|&entry| parse_entry(entry))
            .filter_map(|entry| entry)
            .collect::<Vec<DataSet>>();

        let mut results = data_sets
            .into_iter()
            .map(greedy_algorithm)
            .collect::<Vec<SolveResult>>();
        values.append(&mut results);
    }
    values
}

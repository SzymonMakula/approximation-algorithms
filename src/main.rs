use std::collections::{vec_deque, HashMap};
use std::fs;
use std::io::Read;
use std::ops::Add;

use charts::{Chart, ScaleBand, ScaleLinear, VerticalBarView};
use serde::Deserialize;

use magisterka_projekt::knapsack::algorithms::greedy::greedy_algorithm;
use magisterka_projekt::knapsack::algorithms::types::SolveResult;
use magisterka_projekt::knapsack::parsers::parsers::{parse_entry, DataSet, InstanceType, Record};

fn main() {
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
            .map(|set| greedy_algorithm(set))
            .collect::<Vec<SolveResult>>();
        values.append(&mut results);
    }
    values = values
        .into_iter()
        .filter(|result| result.data_set.items_count == 1000)
        .collect();
    let uncorrelated = values
        .iter()
        .filter(|result| matches!(result.data_set.instance_type, InstanceType::Uncorrelated))
        .collect::<Vec<&SolveResult>>();
    let sum = uncorrelated
        .iter()
        .map(|result| result.ratio)
        .reduce(f64::min)
        .unwrap();
    println!("average uncorr {}", sum);

    let correlated = values
        .iter()
        .filter(|result| {
            matches!(
                result.data_set.instance_type,
                InstanceType::StronglyCorrelated
            )
        })
        .collect::<Vec<&SolveResult>>();
    let sum = correlated
        .iter()
        .map(|result| result.ratio)
        .reduce(f64::min)
        .unwrap();
    println!("average corr {}", sum);

    // Define chart related sizes.
    let width = 800;
    let height = 600;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    // Create a band scale that maps ["A", "B", "C"] categories to values in the [0, availableWidth]
    // range (the width of the chart without the margins).
    let x = ScaleBand::new()
        .set_domain(vec![
            String::from("A"),
            String::from("B"),
            String::from("C"),
        ])
        .set_range(vec![0, width - left - right])
        .set_inner_padding(0.1)
        .set_outer_padding(0.1);

    // Create a linear scale that will interpolate values in [0, 100] range to corresponding
    // values in [availableHeight, 0] range (the height of the chart without the margins).
    // The [availableHeight, 0] range is inverted because SVGs coordinate system's origin is
    // in top left corner, while chart's origin is in bottom left corner, hence we need to invert
    // the range on Y axis for the chart to display as though its origin is at bottom left.
    let y = ScaleLinear::new()
        .set_domain(vec![0.0, 100.])
        .set_range(vec![height - top - bottom, 0]);

    // You can use your own iterable as data as long as its items implement the `BarDatum` trait.
    let data = vec![("A", 90), ("B", 10), ("C", 30)];

    // Create VerticalBar view that is going to represent the data as vertical bars.
    let view = VerticalBarView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .load_data(&data)
        .unwrap();

    // Generate and save the chart.
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Bar Chart"))
        .add_view(&view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Units of Measurement")
        .add_bottom_axis_label("Categories")
        .save("./src/vertical-bar-chart.svg")
        .unwrap();
}

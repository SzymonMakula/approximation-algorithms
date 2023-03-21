use std::cmp::Ordering;

use plotters::prelude::{
    BitMapBackend, ChartBuilder, IntoDrawingArea, LabelAreaPosition, LineSeries, WHITE,
};
use plotters::style::BLUE;

use crate::knapsack::algorithms::types::SolveResult;

pub fn create_result_distribution_diagram(results: &Vec<SolveResult>) {
    let root_area = BitMapBackend::new("src/2.7.png", (800, 600)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let total_len = results.len();
    let mut ratios = results
        .iter()
        .map(|x| {
            let values_before = results
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

    ctx.draw_series(LineSeries::new(
        ratios
            .into_iter()
            .map(|(percentage, ratio)| (percentage, ratio)),
        BLUE,
    ))
    .unwrap();
}

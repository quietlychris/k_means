use plotlib::page::Page;
use plotlib::repr::Scatter;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

use std::process::Command;

use crate::*;

pub fn kmeans_plot(kmeans_list: Vec<KMeansPoint>, centroids: Vec<(f64, f64)>, path_root: String) {
    // Checks the total number of elemnts being visualized
    let mut vis_vec: Vec<Vec<(f64, f64)>> = Vec::new();
    for _ in &centroids {
        vis_vec.push(Vec::new());
    }

    for point in &kmeans_list {
        let mut counter = 0;
        for centroid in &centroids {
            if get_distance(centroids[point.cluster], centroid.clone()) < 0.05 {
                vis_vec[counter].push(point.point);
            }
            counter += 1;
        }
    }

    let mut element_sum = 0;
    for j in 0..vis_vec.len() {
        element_sum = element_sum + vis_vec[j].len();
    }
    println!(
        "The total number of elements in vis_vec is: {}",
        element_sum
    );

    let scatter_plots: Vec<Scatter> = Vec::new();

    // We create our scatter plot from the data
    let s1: Scatter = Scatter::from_slice(&vis_vec[0]).style(
        PointStyle::new()
            .marker(PointMarker::Square) // setting the marker to be a square
            .colour("green"),
    );

    let s2: Scatter = Scatter::from_slice(&vis_vec[1]).style(
        PointStyle::new()
            .marker(PointMarker::Square) // setting the marker to be a square
            .colour("blue"),
    );

    let s3: Scatter = Scatter::from_slice(&vis_vec[2]).style(
        PointStyle::new()
            .marker(PointMarker::Square) // setting the marker to be a square
            .colour("red"),
    );

    //let mut data3: Vec<(f64,f64)> = vec![(-1.6, -2.7),(2.0,1.0)];
    let c1: Scatter = Scatter {
        data: vec![centroids[0].clone()],
        style: PointStyle::new(),
    }
    .style(PointStyle::new().colour("green"));
    let c2: Scatter = Scatter {
        data: vec![centroids[1].clone()],
        style: PointStyle::new(),
    }
    .style(PointStyle::new().colour("blue"));
    let c3: Scatter = Scatter {
        data: vec![centroids[2].clone()],
        style: PointStyle::new(),
    }
    .style(PointStyle::new().colour("red"));

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(s1)
        .add(s2)
        .add(s3)
        .add(c1)
        .add(c2)
        .add(c3)
        .x_range(-5., 5.)
        .y_range(-5., 5.)
        .x_label("x-axis")
        .y_label("y-axis");

    // A page with a single view is then saved to an SVG file
    let svg_path = path_root.clone() + ".svg";
    let png_path = path_root.clone() + ".png";

    Page::single(&v).save(&svg_path).unwrap();
    Command::new("cairosvg")
        .arg(svg_path)
        .arg("-o")
        .arg(png_path)
        .output()
        .expect("failed to convert .svg file to .png file");

    // Sleeps the thread for visualization
    std::thread::sleep(std::time::Duration::from_millis(1500));
    println!("\n");
}

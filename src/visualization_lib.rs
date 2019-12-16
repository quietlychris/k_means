use plotlib::page::Page;
use plotlib::repr::Scatter;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

use rand::Rng;
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

    let mut scatter_plots: Vec<Scatter> = Vec::new();
    for i in 0..centroids.len() {
        let mut rng = rand::thread_rng();
        let color = format!("#{}", rng.gen_range(0, 999999).to_string(),);
        let s: Scatter = Scatter::from_slice(&vis_vec[i]).style(
            PointStyle::new()
                .marker(PointMarker::Square) // setting the marker to be a square
                .colour(&color),
        );
        let c: Scatter = Scatter {
            data: vec![centroids[i].clone()],
            style: PointStyle::new(),
        }
        .style(PointStyle::new().colour(color));
        scatter_plots.push(s);
        scatter_plots.push(c);
    }

    //let mut data3: Vec<(f64,f64)> = vec![(-1.6, -2.7),(2.0,1.0)];

    let mut v = ContinuousView::new()
        .x_range(-5., 5.)
        .y_range(-5., 5.)
        .x_label("x-axis")
        .y_label("y-axis");

    for i in 0..scatter_plots.len() {
        v.representations.push(Box::new(scatter_plots[i].clone()));
    }
    //v.add(scatter_plots[0]);

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

use plotlib::page::Page;
use plotlib::repr::Scatter;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

use rand::Rng;

use std::process::Command;

#[derive(Debug, Clone, Copy)]
struct KMeanPoint {
    point: (f64, f64),
    cluster: usize,
}

impl KMeanPoint {
    fn print(&self) {
        print!(
            "Point: ({:.2},{:.2}), cluster: {}",
            self.point.0, self.point.1, self.cluster
        );
    }
}

fn main() {
    let svg_path = "scatter.svg";
    let png_path = "scatter.png";
    let mut rng = rand::thread_rng();

    /*let data: Vec<(f64,f64)> = vec![
        (-3.,-3.),
        (-3.1,-3.1),
        (-3.1,-2.9),
        (-3.2,3.0),
        (1.9,2.1),
        (2.,2.),
        (1.9,1.9),
        (2.0,1.9),
        (-4.,3.9),
        (-4.2,4.1),
        (-3.9,4.0)
    ];*/
    let mut data = Vec::new();
    let mut centroids: Vec<(f64, f64)> = Vec::new();
    for _ in 0..100 {
        data.push((rng.gen_range(0., 5.) as f64, rng.gen_range(0., 5.) as f64));
        data.push((
            rng.gen_range(-5., -0.) as f64,
            rng.gen_range(-5., 0.) as f64,
        ));
        data.push((
            rng.gen_range(-5., -2.5) as f64,
            rng.gen_range(2.5, 5.) as f64,
        ));
        data.push((rng.gen_range(-5., 5.) as f64, rng.gen_range(-5., 5.) as f64));
    }

    let mut centroids: Vec<_> = Vec::new();
    //centroids.push((-2.0,-2.0));
    //centroids.push((1.5,1.5));
    //centroids.push((-3.2,3.5));
    let num_centroids = 3;
    for i in 0..num_centroids {
        centroids.push((rng.gen_range(-5., 5.) as f64, rng.gen_range(-5., 5.) as f64));
        //println!("{:?}",centroids[i]);
    }
    for centroid in &centroids {
        print_point(*centroid);
    }
    print!("\n");

    // Build the kmeans_list and push a unique point to each of them
    let mut kmeans_list: Vec<KMeanPoint> = Vec::new();
    for point in &data {
        let mut assigned_centroid = centroids[0];
        for centroid in &centroids {
            if get_distance(point.clone(), centroid.clone())
                < get_distance(point.clone(), assigned_centroid.clone())
            {
                assigned_centroid = centroid.clone();
            }
        }
        let index = centroids
            .iter()
            .position(|&value| value == assigned_centroid)
            .unwrap();
        println!("index = {}", index);
        let kmeans_point: KMeanPoint = KMeanPoint {
            point: point.clone(),
            cluster: index,
        };
        kmeans_list.push(kmeans_point);
    }

    println!("The length of the kmeans_list is: {}", kmeans_list.len());

    for iteration in 0..10 {
        println!("**** ITERATION #{}", iteration);

        for point in &mut kmeans_list {
            // Calculate the distance to each centroid in the list. If the distance is smaller than the one that exists, replace
            for centroid in &centroids {
                if get_distance(point.point, *centroid)
                    < get_distance(point.point, centroids[point.cluster])
                {
                    let index = centroids
                        .iter()
                        .position(|&value| value == centroid.clone())
                        .unwrap();
                    point.cluster = index;
                }
            }
        }

        let mut count = 0;
        for centroid in &centroids {
            println!("Centroid ({:.2},{:.2}): ", centroid.0, centroid.1);
            for point in &kmeans_list {
                if centroids[point.cluster] == centroid.clone() {
                    //print_point(point.point);
                    point.print();
                    println!(" #{}", count);
                    count += 1;
                }
            }
        }

        // We now have a kmeans_list of points with their nearest centroid and distance to that centroid
        // Now we need to move the centroids to the center of the cluster of all points assigned to them
        for i in 0..centroids.len() {
            let mut x_sum: f64 = 0.0;
            let mut y_sum: f64 = 0.0;
            let mut counter: f64 = 0.0;

            for point in &kmeans_list {
                //if kmeans_list[j].centroid == centroids[i] {
                if get_distance(centroids[point.cluster].clone(), centroids[i].clone()) < 0.05 {
                    counter = counter + 1.0;
                    x_sum = x_sum + point.point.0;
                    y_sum = y_sum + point.point.1;
                }
            }
            centroids[i] = (x_sum / counter, y_sum / counter);
        }

        print!("\n");
        print!("Current centroids: ");
        for cen in &centroids {
            print!("({:.2},{:.2}), ", cen.0, cen.1);
        }
        print!("\n");

        // Doing the data visualization here
        {
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
            Page::single(&v).save(svg_path).unwrap();
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
    }
}

pub fn get_distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    let dist = ((a.0 - b.0).abs().powf(2.0) + (a.1 - b.1).abs().powf(2.0)).sqrt();
    dist
}

fn print_point(a: (f64, f64)) {
    print!("({:.2},{:.2})", a.0, a.1);
}

mod tests {

    use crate::get_distance;

    #[test]
    fn check_gd() {
        let a = (0.0, 0.0);
        let b = (3.0, -4.0);
        println!("{:?} to {:?} = {}", a, b, get_distance(a, b));
    }
}

use plotlib::page::Page;
use plotlib::repr::{Scatter};
use plotlib::view::ContinuousView;
use plotlib::style::{PointMarker, PointStyle};

use rand::Rng;
use rand::seq::SliceRandom;

use std::process::Command;
use std::fs;

#[derive(Debug,Clone,Copy)]
struct KMeanPoint {
    point: (f64,f64),
    centroid: (f64,f64),
    distance: f64
}

fn main() {

    let svg_path = "scatter.svg";
    let png_path = "scatter.png";
    let mut rng = rand::thread_rng();

    /*let data: Vec<(f64,f64)> = vec![
        (-3.,-3.),
        (-3.1,-3.1),
        (-3.1,-2.9),
        (1.9,2.1),
        (2.,2.),
        (1.9,1.9),
    ];*/
    let mut data = Vec::new();
    for _ in 0..2000 {
        data.push((rng.gen_range(0., 5.) as f64,rng.gen_range(0., 5.) as f64));
        data.push((rng.gen_range(-5., -0.) as f64,rng.gen_range(-5., 0.) as f64));
        data.push((rng.gen_range(-5., -2.5) as f64,rng.gen_range(2.5, 5.) as f64));
        //data.push((rng.gen_range(-5., -0.) as f64,rng.gen_range(-5., 0.) as f64));

    }

    let mut centroids: Vec<_> = Vec::new();
    //centroids.push((-2.0,-2.0));
    //centroids.push((1.5,1.5));
    let num_centroids = 3;
    for i in 0..num_centroids {
        centroids.push((rng.gen_range(-5., 5.) as f64,rng.gen_range(-5., 5.) as f64));
        //println!("{:?}",centroids[i]);
    }


    // Build the kmeans_list and push a unique point to each of them
    let mut kmeans_list: Vec<KMeanPoint> = Vec::new();
    // For every point in the data,
    for point in &data {
        let mut kmeans_point: KMeanPoint = KMeanPoint {point: point.clone(), centroid: centroids.choose(&mut rand::thread_rng()).unwrap().clone(),distance: 0.0 };
        kmeans_point.distance = ((point.0 - kmeans_point.centroid.0).abs().powf(2.0) + (point.1 - kmeans_point.centroid.1).abs().powf(2.0)).sqrt();
        //println!("{:?}",kmeans_point);
        kmeans_list.push(kmeans_point);
    }

    /*for point in &kmeans_list {
        println!("{:?}",point);
    }*/

    for point in &mut kmeans_list {
        // Calculate the distance to each centroid in the list. If the distance is smaller than the one that exists, replace
        let mut dist = 0.0;
        for mut centroid in &centroids {
            // Calculating the Euclidean distance between the point and the centroid
            dist = ((point.point.0 - centroid.0).abs().powf(2.0) + (point.point.1 - centroid.1).abs().powf(2.0)).sqrt();
            // Calculates the distance to each centroid and pushes it to a vector
            //println!("Distance of {:?} to centroid {:?} = {}",point.point,point.centroid,dist);
            if dist < point.distance {
                //println!("Switching {:?}'s assigned centroid from {:?} to {:?}",point,point.centroid,centroid);

                point.centroid = *centroid;
            }
            else if dist > point.distance {
                //println!("Since {} > {} , keeping centroid at {:?} ",dist,point.distance,point.centroid);
            }
        }
    }

    /*println!(" ");
    for point in &kmeans_list {
        println!("{:?}",point);
    }*/

    // Doing the data visualization here
    {
        // We create our scatter plot from the data
        let s1: Scatter = Scatter::from_slice(&data).style(
            PointStyle::new()
                .marker(PointMarker::Square) // setting the marker to be a square
                .colour("black"),
        ); // and a custom colour

        //let mut data3: Vec<(f64,f64)> = vec![(-1.6, -2.7),(2.0,1.0)];
        let c0: Scatter = Scatter {data: vec![centroids[0].clone()], style: PointStyle::new() }.style(PointStyle::new().colour("green"));
        let c1: Scatter = Scatter {data: vec![centroids[1].clone()], style: PointStyle::new() }.style(PointStyle::new().colour("blue"));

        // The 'view' describes what set of data is drawn
        let v = ContinuousView::new()
            .add(s1)
            .add(c0)
            .add(c1)
            .x_range(-5., 5.)
            .y_range(-5., 5.)
            .x_label("x-axis")
            .y_label("y-axis");

        // A page with a single view is then saved to an SVG file
        Page::single(&v).save(svg_path).unwrap();
        Command::new("cairosvg").arg(svg_path).arg("-o").arg(png_path).output().expect("failed to convert .svg file to .png file");

        // Sleeps the thread for visualization
        std::thread::sleep(std::time::Duration::from_millis(1000));
        println!("\n");
    }

    for iteration in 0..10 {

        println!("**** ITERATION #{}",iteration);
        /*for centroid in &centroids {
            println!("Centroid is located at: {:?}",centroid);
        }*/

        for point in &mut kmeans_list {
            // Calculate the distance to each centroid in the list. If the distance is smaller than the one that exists, replace
            let mut dist = 0.0;
            for mut centroid in &centroids {
                // Calculating the Euclidean distance between the point and the centroid
                dist = ((point.point.0 - centroid.0).abs().powf(2.0) + (point.point.1 - centroid.1).abs().powf(2.0)).sqrt();
                // Calculates the distance to each centroid and pushes it to a vector
                //println!("Distance of {:?} to centroid {:?} = {}",point.point,point.centroid,dist);
                if dist < point.distance {
                    //println!("Switching {:?}'s assigned centroid from {:?} to {:?}",point,point.centroid,centroid);
                    point.centroid = *centroid;
                }
                else {
                    //println!("Since {} < {}, keeping centroid at {:?} ",point.distance,dist,point.centroid);
                }
            }
        }

        /*for point in &kmeans_list {
            println!("{:?}",point);
        }*/

        for d in &centroids {
            let ct = format!("({:.2},{:.2})",d.0,d.1);
            println!("{}",ct);
        }

        /*for point in &kmeans_list {
            let ct = format!("({:.2},{:.2})",point.centroid.0,point.centroid.1);
            println!("{} should be {} or {}",ct,ct0,ct1);
        }*/

        println!("{}",centroids.len());

        // Doing the data visualization here
        let mut vis_vec: Vec<Vec<(f64,f64)>> =  Vec::new();
        for a in & centroids {
            vis_vec.push(Vec::new());
        }
        for point in &kmeans_list {
            for i in 0..centroids.len() {
                if point.centroid == centroids[i] {
                    vis_vec[i].push(point.point);
                }
            }
        }
        //dbg!(&vis_vec[0]);

        // We now have a kmeans_list of points with their nearest centroid and distance to that centroid
        // Now we need to move the centroids to the center of the cluster of all points assigned to them
        for i in 0..centroids.len() {
            let mut x_sum: f64 = 0.0;
            let mut y_sum: f64 = 0.0;
            let mut counter: f64 = 0.0;

            //let mut point_list = Vec::new();
            for j in 0..kmeans_list.len() {
                if kmeans_list[j].centroid == centroids[i] {
                    // println!("{:?} == {:?}",kmeans_list[j].centroid, centroids[i]);
                    counter = counter + 1.0;
                    //println!("For the {:.0}th point assigned to centroid #{}, we're adding {} to x_sum",counter,i,kmeans_list[j].point.0);
                    //println!("{} = {} + {}",x_sum + kmeans_list[j].point.0, x_sum, kmeans_list[j].point.0 );
                    x_sum = x_sum + kmeans_list[j].point.0;
                    //println!("For the {:.0}th point assigned to centroid #{}, we're adding {} to y_sum",counter,i,kmeans_list[j].point.1);
                    y_sum = y_sum + kmeans_list[j].point.1;
                    //point_list.push(kmeans_list[j]);
                }
            }
            //println!("There were {:.0}/{} points for centroid #{}",counter,data.len(),i);
            //println!("For centroid #{}, x_sum: {:.2}, y_sum: {:.2}, counter: {:.2}",i,x_sum,y_sum,counter);
            centroids[i] = (x_sum / counter, y_sum / counter);
            //println!("New location for centroid #{} is now: {:?}",i,centroids[i]);
        }

        {
            // We create our scatter plot from the data
            let s1: Scatter = Scatter::from_slice(&vis_vec[0]).style(
                PointStyle::new()
                    .marker(PointMarker::Square) // setting the marker to be a square
                    .colour("green"),
            ); // and a custom colour

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
            let c1: Scatter = Scatter {data: vec![centroids[0].clone()], style: PointStyle::new() }.style(PointStyle::new().colour("green"));
            let c2: Scatter = Scatter {data: vec![centroids[1].clone()], style: PointStyle::new() }.style(PointStyle::new().colour("blue"));
            let c3: Scatter = Scatter {data: vec![centroids[2].clone()], style: PointStyle::new() }.style(PointStyle::new().colour("red"));


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
            Command::new("cairosvg").arg(svg_path).arg("-o").arg(png_path).output().expect("failed to convert .svg file to .png file");

            // Sleeps the thread for visualization
            std::thread::sleep(std::time::Duration::from_millis(100));
            println!("\n");
        }
    }
}

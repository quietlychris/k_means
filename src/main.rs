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
    for _ in 0..100 {
        data.push((rng.gen_range(0., 5.) as f64,rng.gen_range(0., 5.) as f64));
        data.push((rng.gen_range(-5., -0.) as f64,rng.gen_range(-5., 0.) as f64));
        data.push((rng.gen_range(-5., -2.5) as f64,rng.gen_range(2.5, 5.) as f64));
        //data.push((rng.gen_range(-5., -0.) as f64,rng.gen_range(-5., 0.) as f64));

    }

    let mut centroids: Vec<_> = Vec::new();
    //centroids.push((-2.0,-2.0));
    //centroids.push((1.5,1.5));
    //centroids.push((-3.2,3.5));
    let num_centroids = 3;
    for i in 0..num_centroids {
        centroids.push((rng.gen_range(-5., 5.) as f64,rng.gen_range(-5., 5.) as f64));
        //println!("{:?}",centroids[i]);
    }


    // Build the kmeans_list and push a unique point to each of them
    let mut kmeans_list: Vec<KMeanPoint> = Vec::new();
    for point in &data {
        // Had started by randomly assigning points to cluster. Don't do this!
        //let mut kmeans_point: KMeanPoint = KMeanPoint {point: point.clone(), centroid: centroids.choose(&mut rand::thread_rng()).unwrap().clone(),distance: 0.0 };
        let mut assigned_centroid = centroids[0];
        for centroid in &centroids {
            if get_distance(point.clone(),centroid.clone()) < get_distance(point.clone(),assigned_centroid.clone()) {
                assigned_centroid = centroid.clone();
            }
        }

        let kmeans_point : KMeanPoint = KMeanPoint {point: point.clone(), centroid:assigned_centroid, distance: get_distance(point.clone(),assigned_centroid) };
        //println!("{:?}",kmeans_point);
        kmeans_list.push(kmeans_point);
    }

    println!("The length of the kmeans_list is: {}",kmeans_list.len());
    /*for point in &kmeans_list {
        println!("{:?}",point);
    }*/

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
        let c1: Scatter = Scatter {data: vec![centroids[0].clone()], style: PointStyle::new() }.style(PointStyle::new().colour("green"));
        let c2: Scatter = Scatter {data: vec![centroids[1].clone()], style: PointStyle::new() }.style(PointStyle::new().colour("blue"));
        let c3: Scatter = Scatter {data: vec![centroids[2].clone()], style: PointStyle::new() }.style(PointStyle::new().colour("red"));


        // The 'view' describes what set of data is drawn
        let v = ContinuousView::new()
            .add(s1)
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
        std::thread::sleep(std::time::Duration::from_millis(1000));
        println!("\n");
    }

    for iteration in 0..5 {

        println!("**** ITERATION #{}",iteration);
        /*for centroid in &centroids {
            println!("Centroid is located at: {:?}",centroid);
        }*/

        for point in &mut kmeans_list {
            // Calculate the distance to each centroid in the list. If the distance is smaller than the one that exists, replace
            for centroid in &centroids {
                if get_distance(point.point,centroid.clone()) < point.distance {
                    point.centroid = centroid.clone();
                    point.distance = get_distance(point.point,point.centroid);
                }
            }
        }

        // Doing the data visualization here
        let mut vis_vec: Vec<Vec<(f64,f64)>> =  Vec::new();
        for _ in &centroids {
            vis_vec.push(Vec::new());
        }
        for point in &kmeans_list {
            let mut check = 0;
            for i in 0..centroids.len() {
                if get_distance(point.centroid,centroids[i].clone()) < 0.01 {
                    vis_vec[i].push(point.point);
                }
                else {
                    check = 1;
                }
            }
            if check != 0 {
                println!("{:?} != {:?}",point.centroid,centroids);
            }
        }
        let mut element_sum = 0;
        for j in 0..vis_vec.len() {
            element_sum = element_sum + vis_vec[j].len();
        }
        println!("The total number of elements in vis_vec is: {}",element_sum);

        //dbg!(&vis_vec[0]);

        /*for centroid in &centroids {
            println!("Centroid {:?}: ",centroid);
            for point in &kmeans_list {
                if point.centroid == centroid.clone() {
                    println!("{:?}",point);
                }
            }
        }*/

        println!("The length of the kmeans_list is: {}",kmeans_list.len());
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

        // Doing the data visualization here
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
            std::thread::sleep(std::time::Duration::from_millis(1000));
            println!("\n");
        }
    }
}

fn get_distance(a: (f64,f64), b: (f64,f64)) -> f64 {
    let dist = ((a.0 - b.0).abs().powf(2.0) + (a.1 - b.1).abs().powf(2.0)).sqrt();
    dist
}

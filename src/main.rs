use rand::Rng;

mod kmeans;
mod visualization_lib;
use kmeans::*;
use visualization_lib::*;

fn main() {
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
    let mut kmeans_list: Vec<KMeansPoint> = Vec::new();
    for point in &data {
        let mut assigned_centroid = centroids[0];
        for centroid in &centroids {
            if get_distance(*point, *centroid) < get_distance(*point, assigned_centroid) {
                assigned_centroid = centroid.clone();
            }
        }
        let index = centroids
            .iter()
            .position(|&value| value == assigned_centroid)
            .unwrap();
        println!("index = {}", index);
        let kmeans_point: KMeansPoint = KMeansPoint {
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
        kmeans_plot(
            kmeans_list.clone(),
            centroids.clone(),
            "scatter".to_string(),
        );
    }
}

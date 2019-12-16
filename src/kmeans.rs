#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct KMeansPoint {
    pub point: (f64, f64),
    pub cluster: usize,
}

impl KMeansPoint {
    pub fn print(&self) {
        print!(
            "Point: ({:.2},{:.2}), cluster: {}",
            self.point.0, self.point.1, self.cluster
        );
    }
}

pub fn get_distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    let dist = ((a.0 - b.0).abs().powf(2.0) + (a.1 - b.1).abs().powf(2.0)).sqrt();
    dist
}

pub fn print_point(a: (f64, f64)) {
    print!("({:.2},{:.2})", a.0, a.1);
}

mod tests {

    #[test]
    fn check_gd() {
        let a = (0.0, 0.0);
        let b = (3.0, -4.0);
        let dist = crate::get_distance(a, b);
        println!("{:?} to {:?} = {}", a, b, dist);
        assert_eq!(5.0, dist);
    }
}

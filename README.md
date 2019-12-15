### k_means

A pure Rust implementation of the k_means algorithm on a vector of (f64,f64) data with visualization. The conversion from `.svg` to `.png` uses the `cairo` backend, which can be installed with:
```
$ pip install cairosvg
```

The following is the cluster selection for data with 100 point clusters placed in Quadrant I, IV, and from `(-2.5,2.5)` to `(-5.0,5.0)`, as well as 100 random points across the range of `(-5.0,-5.0)` to `(5.0,5.0)`, for a total of 400 points. Circles are the cluster centroids.

<p align="center"><img src="/scatter.png" width="600" height="500" /></p>

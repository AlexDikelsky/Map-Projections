use core::f64::consts::PI;

pub fn sphere_coords(num_lines: usize, num_points: usize) -> Vec<(f64, f64)> {
    merids(num_lines, num_points).iter().copied().chain(
        pars(num_lines, num_points)).collect()
}

pub fn polar_to_cartesian(point: (f64, f64)) -> (f64, f64) {
    let (rho, theta) = point;
    (rho*theta.sin(), -rho*theta.cos())
}
    

fn merids(num_merids: usize, num_points: usize) -> Vec<(f64, f64)> {
    points_between(0.0, PI/2.0, num_points)
        .iter().copied()
        .map(|x| meridian(x, num_merids)).flatten().collect()
}

fn pars(num_pars: usize, num_points: usize) -> Vec<(f64, f64)> {
    merids(num_pars, num_points).iter().map(|(x, y)| (*y, *x)).collect()
}

// takes radian degree between 0 and π/2
// integral num points
// returns meridian with num points on both "sides"
fn meridian(deg: f64, num: usize) -> Vec<(f64, f64)> {
    points_between(-(PI/2.0), PI/2.0, num).iter().copied()
        .map(|x| vec![(x, deg), (x, deg-(PI/2.0))])
    .flatten().collect()
}

//num+1 points between min and max, inclusive
// works with min>max, but goes backward
fn points_between(min: f64, max: f64, num: usize) -> Vec<f64> {
    let inc = (max - min) / (num as f64);
    (0..=num).map(|x| min + ((x as f64) * inc))
        .collect()
}

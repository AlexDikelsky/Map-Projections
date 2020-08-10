use core::f64::consts::FRAC_PI_2;

#[derive(Copy,Clone)]
pub struct LatLonPoint {
    pub lambda: f64,
    pub phi: f64,
}

#[derive(Copy,Clone)]
pub struct PolarPoint {
    pub rho: f64,
    pub theta: f64,
}

#[derive(Copy,Clone)]
pub struct CartPoint {
    pub x: f64,
    pub y: f64,
}

impl PolarPoint {
    pub fn to_cart(&self) -> CartPoint {
        CartPoint {
            x: self.rho*self.theta.sin(),
            y: -self.rho*self.theta.cos()
        }
    }
}

impl CartPoint {
    pub fn to_tuple(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

impl LatLonPoint {
    pub fn dist_to(&self, other: &LatLonPoint) -> f64 {
        great_circle_dist(*self, *other) 
    }
}



pub fn sphere_coords(num_lines: usize, num_points: usize) -> Vec<LatLonPoint> {
    merids(num_lines, num_points).iter().copied().chain(
        pars(num_lines, num_points)).collect()
}

pub fn great_circle_dist(start: LatLonPoint, end: LatLonPoint) -> f64 {
    let (lambda_1, phi_1) = (start.lambda, start.phi);
    let (lambda_2, phi_2) = (end.lambda, end.phi);

    let delta_lambda = (lambda_1 - lambda_2).abs();

    (phi_1.sin() * phi_2.sin() + 
    phi_1.cos() * phi_2.cos() * delta_lambda.cos()
    ).acos()
}

//num points between min and max, exclusive
// works with min>max, but goes backward
pub fn points_between_exclusive(min: f64, max: f64, num: usize) -> Vec<f64> {
    let inc = (max - min) / (num as f64);
    (0..num).map(|x| min + ((x as f64) * inc))
        .collect()
}

//num points between min and max, inclusive
pub fn points_between_inclusive(min: f64, max: f64, num: usize) -> Vec<f64> {
    let inc = (max - min) / ((num-1) as f64);
    (0..num).map(|x| min + ((x as f64) * inc))
        .collect()
}

fn merids(num_merids: usize, num_points: usize) -> Vec<LatLonPoint> {
    points_between_inclusive(0.0, FRAC_PI_2, num_points)
        .iter().copied()
        .map(|x| meridian(x, num_merids)).flatten().collect()
}

fn pars(num_pars: usize, num_points: usize) -> Vec<LatLonPoint> {
    merids(num_pars, num_points).iter().map(|point| 
                                            // Swap lambda and phi for merids
                                            LatLonPoint { lambda: point.phi, 
                                                phi: point.lambda }).collect()
}

// takes radian degree between 0 and π/2
// integral num points
// returns meridian with num points on both "sides"
fn meridian(deg: f64, num: usize) -> Vec<LatLonPoint> {
    points_between_inclusive(-(FRAC_PI_2), FRAC_PI_2, num).iter().copied()
        .map(|x| vec![
             LatLonPoint { lambda: x, phi: deg },
             LatLonPoint { lambda: x, phi: deg-(FRAC_PI_2) }])
    .flatten().collect()
}

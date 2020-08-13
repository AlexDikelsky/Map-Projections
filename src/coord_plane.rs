use core::f64::consts::FRAC_PI_2;
use std::fmt;

#[derive(Copy,Clone,Debug)]
pub struct LatLonPoint {
    pub lambda: f64,
    pub phi: f64,
}

#[derive(Copy,Clone)]
pub struct PolarPoint {
    pub rho: f64,
    pub theta: f64,
}

#[derive(Copy,Clone,Debug)]
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
    pub fn to_latlon_raw(&self) -> LatLonPoint {
        LatLonPoint {
            lambda: self.x,
            phi: self.y,
        }
    }
}

impl LatLonPoint {
    pub fn dist_to(&self, other: &LatLonPoint) -> f64 {
        great_circle_dist(*self, *other) 
    }
}

impl fmt::Display for LatLonPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lat: {0:.5}, Lon: {1:.5}", self.phi, self.lambda)
    }
}



pub fn map_one_point<'a>(mapping_fn: &'a Box<dyn Fn(Vec<LatLonPoint>) -> Vec<CartPoint>>, point: LatLonPoint) -> CartPoint {
    mapping_fn(vec![point])[0]
}



pub fn great_circle_dist(start: LatLonPoint, end: LatLonPoint) -> f64 {
    let (lambda_1, phi_1) = (start.lambda, start.phi);
    let (lambda_2, phi_2) = (end.lambda, end.phi);

    let delta_lambda = (lambda_1 - lambda_2).abs();

    (phi_1.sin() * phi_2.sin() + 
    phi_1.cos() * phi_2.cos() * delta_lambda.cos()
    ).acos()
}




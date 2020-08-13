use core::f64::consts::*;
use crate::coord_plane::LatLonPoint;
use crate::coord_plane::CartPoint;
#[allow(dead_code)]
pub fn mercator(points: Vec<LatLonPoint>) -> Vec<CartPoint> {
    points.iter().map(
        |llpoint| CartPoint {
            x: llpoint.lambda, 
            y: (FRAC_PI_4 + llpoint.phi/2.0).tan().ln()}
            ).collect()
}

#[allow(dead_code)]
pub fn equirectangular(points: Vec<LatLonPoint>) -> Vec<CartPoint> {
    points.iter().map(
        |llpoint| CartPoint {
            x: llpoint.lambda,
            y: llpoint.phi,
        }
        ).collect()
}


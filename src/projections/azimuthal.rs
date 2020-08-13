use core::f64::consts::*;
use crate::coord_plane::LatLonPoint;
use crate::coord_plane::CartPoint;
use crate::coord_plane::PolarPoint;

//#[allow(dead_code)]
//pub fn lambert_azimuthal_equal_area(points: Vec<LatLonPoint>, center_lat: f64) -> Vec<CartPoint> {
//    let phi_1 = center_lat;
//
//    points.iter().map(
//        |llpoint| {
//            let sin_phi = llpoint.phi.sin();
//            let sin_phi_1 = phi_1.sin();
//            let sin_lambda = llpoint.lambda.sin();
//            let cos_phi = llpoint.phi.cos();
//            let cos_phi_1 = phi_1.cos();
//            let cos_lambda = llpoint.lambda.cos();
//
//            let z = (sin_phi_1 * sin_phi + 
//                cos_phi_1 * cos_phi * cos_lambda).acos();
//
//            //Check vs sin_theta
//            let cos_theta =
//                (cos_phi_1 * sin_phi - sin_phi_1 * cos_phi * cos_lambda)
//                / (z.sin());
//
//            let sin_theta = 
//                cos_phi * sin_lambda / z.sin();


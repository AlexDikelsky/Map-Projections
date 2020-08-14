//use core::f64::consts::*;
use crate::coord_plane::LatLonPoint;
use crate::coord_plane::CartPoint;
use crate::coord_plane::PolarPoint;

#[allow(dead_code)]
pub fn bonne(points: Vec<LatLonPoint>, central_meridian: f64) -> Vec<CartPoint> {
    //Technically around for a long time before Boone
    //Based off Werner's cordiform projections, which was based off
    //of Ptolemey's second projection
    //
    //Equal area and correct scale along central meridan and parallels

    let phi_1: f64 = central_meridian;
    let cot_phi_1 = 1.0/(phi_1.tan());
    points.iter().map(
        |llpoint| {
            let rho = cot_phi_1 + phi_1 - llpoint.phi;
            let theta = llpoint.lambda * (llpoint.phi.cos() / rho);
            PolarPoint { rho: rho, theta: theta }.to_cart()
        }).collect()
}

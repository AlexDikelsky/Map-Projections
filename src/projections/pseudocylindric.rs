use core::f64::consts::*;
use crate::coord_plane::LatLonPoint;
use crate::coord_plane::CartPoint;
#[allow(dead_code)]
pub fn sinusoidal(points: Vec<LatLonPoint>) -> Vec<CartPoint> {
    points.iter().map(|llpoint|
                      CartPoint { 
                          x: llpoint.lambda * llpoint.phi.cos(), 
                          y: llpoint.phi
                      }).collect()
}

#[allow(dead_code)]
pub fn loximuthal(points: Vec<LatLonPoint>, central_lat: f64) -> Vec<CartPoint> {
    let phi_1 = central_lat;

    points.iter().map(|llpoint| {
        CartPoint { 
            x: if llpoint.phi == phi_1 {
                //Degenerate case
                llpoint.lambda * llpoint.phi.cos()

            } else {
                llpoint.lambda * (llpoint.phi-phi_1) / 
                (
                    (FRAC_PI_4 + llpoint.phi/2.0).tan().ln() -
                    (FRAC_PI_4 + phi_1 / 2.0).tan().ln()
                )
            },
            y: llpoint.phi - phi_1,
        }
    }).collect()
}

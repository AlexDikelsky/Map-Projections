use core::f64::consts::*;
use crate::coord_plane::CartPoint;
use crate::coord_plane::PolarPoint;
use crate::projections::projection_types::ProjectionParams;


pub fn simple_equidistant_conic(params: ProjectionParams) -> Vec<CartPoint> {
    match params {
        ProjectionParams::PointsTwoStandardPar(points, phi_1, phi_2) => {
            let temp = (phi_2 * phi_1.cos() - 
                        phi_1 * phi_2.cos())
                / (phi_1.cos() - phi_2.cos());

            let n = (phi_1.cos() - phi_2.cos())/(phi_2 - phi_1);

            points.iter().map(
                |llpoint| {
                    let rho = temp - llpoint.phi;
                    let theta = n * llpoint.lambda;
                    PolarPoint { rho: rho, theta: theta }.to_cart()
                }).collect()
        },
        _ => panic!("Invalid parameters {:?} passed in"),
    }
}

pub fn lambert_conformal_conic(params: ProjectionParams) -> Vec<CartPoint> {
    match params {
        ProjectionParams::PointsTwoStandardPar(points, phi_1, phi_2) => {
            let n = (
                    (phi_1.cos().ln() -
                    phi_2.cos().ln()) 
                ) / (
                    (FRAC_PI_4 + phi_2).tan().ln() -
                    (FRAC_PI_4 + phi_1).tan().ln());

            points.iter().map(
                |llpoint| {
                    let theta = n * llpoint.lambda;
                    let rho = 
                        phi_1.cos() *
                        (FRAC_PI_4 + phi_1/2.0).tan().powf(n)
                        /
                        (n * (FRAC_PI_4 + llpoint.phi/2.0).tan().powf(n));
                    PolarPoint { rho: rho, theta: theta }.to_cart()
                }).collect()
        },
        _ => panic!("Invalid parameters {:?} passed in"),
    }
}

use core::f64::consts::*;
use crate::coord_plane::LatLonPoint;
use crate::coord_plane::CartPoint;
use crate::coord_plane::PolarPoint;
//use crate::projections;
//super::ProjectionParams;

use crate::map_bounds::MapBounds;
use crate::projections::projection_types::ProjectionType;
use crate::projections::projection_types::ProjectionParams;


#[allow(dead_code)]
pub fn simple_equidistant_conic(params: ProjectionParams) -> Vec<CartPoint> {
    //points: Vec<LatLonPoint>, standard_par1: f64, standard_par2: f64) -> Vec<CartPoint> {

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
        _ => panic!("Invalid parameters {:?} passed in to simple_equidistant_conic"),
    }
}

#[allow(dead_code)]
pub fn lambert_conformal_conic(points: Vec<LatLonPoint>, 
                                standard_par1: f64, standard_par2: f64) -> Vec<CartPoint> {
    let phi_1 = standard_par1;
    let phi_2 = standard_par2;

    let n = (
            (phi_1.cos().ln() -
            phi_2.cos().ln()) 
        ) / (
            (FRAC_PI_4 + phi_2).tan().ln() -
            (FRAC_PI_4 + phi_1));

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
}

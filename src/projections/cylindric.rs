use core::f64::consts::*;
use crate::coord_plane::CartPoint;
use crate::projections::projection_types::ProjectionParams;

pub fn mercator(params: ProjectionParams) -> Vec<CartPoint> {
    match params {
        ProjectionParams::PointsOnly(points) => {
            points.iter().map(
            |llpoint| CartPoint {
                x: llpoint.lambda, 
                y: (FRAC_PI_4 + llpoint.phi/2.0).tan().ln()
            }).collect()
        },
        _ => panic!("Invalid parameters {:?} passed in"),
    }
}

pub fn equirectangular(params: ProjectionParams) -> Vec<CartPoint> {
    match params {
        ProjectionParams::PointsOnly(points) => {
            points.iter().map(
                |llpoint| CartPoint {
                    x: llpoint.lambda,
                    y: llpoint.phi,
                }
                ).collect()
        },
        _ => panic!("Invalid parameters {:?} passed in"),
    }
}


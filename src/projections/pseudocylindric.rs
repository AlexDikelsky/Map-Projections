use core::f64::consts::*;
use crate::coord_plane::CartPoint;
use crate::projections::projection_types::ProjectionParams;

pub fn sinusoidal(params: ProjectionParams) -> Vec<CartPoint> {
    match params {
        ProjectionParams::PointsOnly(points) => {
            points.iter().map(|llpoint|
                      CartPoint { 
                          x: llpoint.lambda * llpoint.phi.cos(), 
                          y: llpoint.phi
                      }).collect()
        },
        _ => panic!("Invalid parameters {:?} passed in"),
    }
}

pub fn loximuthal(params: ProjectionParams) -> Vec<CartPoint> {
    match params {
        ProjectionParams::PointsStandardPar(points, phi_1) => {
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
        _ => panic!("Invalid parameters {:?} passed in"),
    }
}

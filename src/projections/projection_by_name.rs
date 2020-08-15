use crate::projections::conic;
use crate::projections::pseudoconic;
use crate::projections::cylindric;
use crate::projections::pseudocylindric;
use crate::projections::projection_types::Projection;
use crate::projections::projection_types::ProjectionType;
use crate::chart_and_js_exports::JSProjectionParams;

fn list_projections() -> Vec<Projection> {
    vec![
        Projection {
            name: "Simple Equidistant Conic".to_string(),
            projection_function: Box::new(conic::simple_equidistant_conic),
            projection_type: ProjectionType::Conic,
            params: JSProjectionParams::JSPointsTwoStandardPar,
        },
        Projection {
            name: "Lambert Conformal Conic".to_string(),
            projection_function: Box::new(conic::lambert_conformal_conic),
            projection_type: ProjectionType::Conic,
            params: JSProjectionParams::JSPointsTwoStandardPar,
        },
        Projection {
            name: "Sinusoidal".to_string(),
            projection_function: Box::new(pseudocylindric::sinusoidal),
            projection_type: ProjectionType::PseudoCylindric,
            params: JSProjectionParams::JSPointsOnly,
        },
        Projection {
            name: "Loximuthal".to_string(),
            projection_function: Box::new(pseudocylindric::loximuthal),
            projection_type: ProjectionType::PseudoCylindric,
            params: JSProjectionParams::JSPointsStandardPar,
        },
        Projection {
            name: "Mercator".to_string(),
            projection_function: Box::new(cylindric::mercator),
            projection_type: ProjectionType::Cylindric,
            params: JSProjectionParams::JSPointsOnly,
        },
        Projection {
            name: "Equirectangular".to_string(),
            projection_function: Box::new(cylindric::equirectangular),
            projection_type: ProjectionType::Cylindric,
            params: JSProjectionParams::JSPointsOnly,
        },
        Projection {
            name: "Bonne".to_string(),
            projection_function: Box::new(pseudoconic::bonne),
            projection_type: ProjectionType::PseudoConic,
            params: JSProjectionParams::JSPointsStandardPar,
        },
    ]
}


pub fn use_projection(projection_name: String) -> Option<Projection> {
    list_projections().into_iter().find(
        |proj| proj.name == projection_name
        )
}



use crate::projections::conic;
//use crate::projections::pseudoconic::*;
//use crate::projections::cylindirc::*;
//use crate::projections::pseudocylindirc::*;
use crate::projections::projection_types::Projection;
use crate::projections::projection_types::ProjectionType;
use crate::projections::projection_types::ProjectionParams;

fn list_projections() -> Vec<Projection> {
    vec![
        Projection {
            name: "Simple Equidistant Conic".to_string(),
            projection_function: Box::new(conic::simple_equidistant_conic),
            projection_type: ProjectionType::Conic,
        },
    ]
}


pub fn use_projection(projection_name: String) -> Option<Projection> {
    list_projections().into_iter().find(
        |proj| proj.name == projection_name
        )
}



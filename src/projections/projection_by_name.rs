use crate::projections::conic::*;
use crate::projections::pseudoconic::*;
use crate::projections::cylindirc::*;
use crate::projections::pseudocylindirc::*;
use crate::projections::Projection;
use crate::projections::ProjectionType;
use crate::projections::ProjectionParams;

fn list_projections() -> Vec<Projection> {
    vec![
        Projection {
            conic::simple_equidistant_conic(



//pub fn use_projection(projection_name: String) -> Projection {


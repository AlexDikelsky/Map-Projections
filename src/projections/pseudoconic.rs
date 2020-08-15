use crate::coord_plane::CartPoint;
use crate::projections::projection_types::ProjectionParams;

pub fn bonne(params: ProjectionParams) -> Vec<CartPoint> {
    //Technically around for a long time before Boone
    //Based off Werner's cordiform projections, which was based off
    //of Ptolemey's second projection
    //
    //Equal area and correct scale along central meridan and parallels

    match params {
        ProjectionParams::PointsStandardPar(points, phi_1) => {
            points.iter().map(
                |llpoint| {
                    let rho = phi_1.tan().recip() + phi_1 - llpoint.phi;
                    let theta = (llpoint.lambda * llpoint.phi.cos()) / rho;
                    CartPoint {
                        x: rho * theta.sin(),
                        y: phi_1.tan().recip() - rho * theta.cos(),
                    }
                }).collect()
        },
        _ => panic!("Invalid parameters {:?} passed in"),
    }
}

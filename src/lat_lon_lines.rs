use core::f64::consts::FRAC_PI_2;
use crate::coord_plane::LatLonPoint;
use crate::one_dim_lines::points_between_exclusive_both_ends;
use crate::one_dim_lines::points_between_inclusive;

pub fn sphere_coords(num_lines: usize, num_points: usize) -> Vec<LatLonPoint> {
    merids(num_lines, num_points).iter().copied().chain(
        pars(num_lines, num_points)).collect()
}

fn merids(num_merids: usize, num_points: usize) -> Vec<LatLonPoint> {
    points_between_inclusive(0.0, FRAC_PI_2, num_points)
        .iter().copied()
        .map(|x| meridian(x, num_merids)).flatten().collect()
}

fn pars(num_pars: usize, num_points: usize) -> Vec<LatLonPoint> {
    merids(num_pars, num_points).iter().map(
        |point| LatLonPoint {
            lambda: point.phi,
            phi: point.lambda,
        }).collect()
}

// takes radian degree between 0 and π/2
// integral num points
// returns meridian with num points on both "sides"
fn meridian(deg: f64, num: usize) -> Vec<LatLonPoint> {
    points_between_inclusive(-(FRAC_PI_2), FRAC_PI_2, num).iter().copied()
        .map(|x| vec![
             LatLonPoint { lambda: x, phi: deg },
             LatLonPoint { lambda: x, phi: deg-(FRAC_PI_2) }])
    .flatten().collect()
}

#[allow(dead_code)]
pub fn intersections_of_pars_and_merids(num_lines: usize) -> Vec<LatLonPoint> {
    points_between_exclusive_both_ends(-FRAC_PI_2, FRAC_PI_2, num_lines).iter()
        .map(|x| points_between_exclusive_both_ends(-FRAC_PI_2, FRAC_PI_2, num_lines).iter()
             .map(|y| 
                  LatLonPoint { lambda: *x, phi: *y } 
                  ).collect::<Vec<LatLonPoint>>())
        .flatten().collect()
}

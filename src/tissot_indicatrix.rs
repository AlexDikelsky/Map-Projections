use core::f64::consts::FRAC_PI_2;
use super::coord_plane::points_between_inclusive;
use super::shapes::Circle;

pub fn gen_indicatrices(num_lines: usize, num_points: usize) -> Vec<(f64, f64)> {
    intersections_of_pars_and_merids(num_lines, num_points).iter()
        .map(|p| 
             Circle { 
                 center: *p,
                 radius: super::INDICATRICE_SIZE,
                 num_points: num_points,
             }.expand_to_indicatrix(80.0)
             ).flatten().collect()
}

pub fn circles(num_lines: usize, num_points: usize) -> Vec<(f64, f64)> {
    intersections_of_pars_and_merids(num_lines, num_points).iter()
        .map(|p| 
             Circle { 
                 center: *p,
                 radius: super::INDICATRICE_SIZE,
                 num_points: num_points,
             }.make_circular_points()
             ).flatten().collect()
}

fn intersections_of_pars_and_merids(num_lines: usize, num_points: usize) -> Vec<(f64, f64)> {
    points_between_inclusive(-FRAC_PI_2, FRAC_PI_2, num_lines).iter()
        .map(|x: &f64| points_between_inclusive(-FRAC_PI_2, FRAC_PI_2, num_lines).iter()
             .map(|y: &f64| (*x, *y)).collect::<Vec<(f64, f64)>>())
        .flatten().collect()
}


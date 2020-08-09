use core::f64::consts::FRAC_PI_2;
use super::coord_plane::points_between_inclusive;
use super::shapes::Circle;

pub fn gen_indicatrices(mapping_function: Box<dyn Fn(Vec<(f64, f64)>) -> Vec<(f64,f64)>>, num_lines: usize, num_points: usize) -> Vec<(f64, f64)> {
    let points_and_center: Vec<(Vec<(f64, f64)>, (f64, f64))> =
        intersections_of_pars_and_merids(num_lines, num_points).iter()
        .map(|p| 
             Circle { 
                 center: *p,
                 radius: super::INDICATRIX_SIZE,
                 num_points: num_points,
             }.make_circular_points()).collect();

    points_and_center.iter().map(
    |x| expand_to_indicatrix(project((x.0.clone(), x.1), &mapping_function), 50.0)
    ).flatten().collect()

             //.project(mapping_function)
             //.expand_to_indicatrix(mapping_function, 80.0)
             //).flatten().collect()
}

//pub fn circles(num_lines: usize, num_points: usize) -> Vec<(f64, f64)> {
//    intersections_of_pars_and_merids(num_lines, num_points).iter()
//        .map(|p| 
//             Circle { 
//                 center: *p,
//                 radius: super::INDICATRIX_SIZE,
//                 num_points: num_points,
//             }.make_circular_points()
//             ).flatten().collect()
//}

fn intersections_of_pars_and_merids(num_lines: usize, num_points: usize) -> Vec<(f64, f64)> {
    points_between_inclusive(-FRAC_PI_2, FRAC_PI_2, num_lines).iter()
        .map(|x: &f64| points_between_inclusive(-FRAC_PI_2, FRAC_PI_2, num_lines).iter()
             .map(|y: &f64| (*x, *y)).collect::<Vec<(f64, f64)>>())
        .flatten().collect()
}

// This function pushes points further away from the radius
// This is needed for drawing the tissot indicatrices, because the circle is supposed
// to be infinitismal, then expanded for viewing rather than just a large circle
// in the first place
pub fn expand_to_indicatrix(points: (Vec<(f64, f64)>, (f64,f64)), increase_ratio: f64) -> Vec<(f64, f64)> {
    points.0.iter().map(
        |end| extend_line(points.1, *end, increase_ratio)
        ).collect()
}

fn extend_line(start: (f64, f64), end: (f64, f64), increase_ratio: f64) -> (f64, f64) {
    let x_diff = end.0 - start.0;
    let y_diff = end.1 - start.1;
    (start.0 + x_diff * increase_ratio,
     start.1 + y_diff * increase_ratio,
     )
}
//
// This function pushes points further away from the radius
// This is needed for drawing the tissot indicatrices, because the circle is supposed
// to be infinitismal, then expanded for viewing rather than just a large circle
// in the first place
pub fn project(points: (Vec<(f64, f64)>, (f64,f64)), mapping_function: &Box<dyn Fn(Vec<(f64, f64)>) -> Vec<(f64,f64)>>) -> (Vec<(f64, f64)>, (f64, f64)) {
    // Project the center point, then take the only value
    let center: (f64, f64) = mapping_function(vec![points.1])[0]; 

    (mapping_function(points.0), center)
}

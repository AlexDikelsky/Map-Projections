use super::coord_plane::points_between_exclusive;

//pub fn polygon(corners: Vec<(f64, f64)>, num_points: usize) -> Vec<(f64, f64)> {
//}

pub fn line_exclusive(a: (f64, f64), b: (f64, f64), num_points: usize) -> Vec<(f64, f64)> {
    let (a_x, a_y) = a;
    let (b_x, b_y) = b;

    points_between_exclusive(a_x, b_x, num_points).iter()
        .zip(points_between_exclusive(a_y, b_y, num_points).iter())
        .map(|(i, j)| (*i, *j)).collect()
}

     

use super::coord_plane::points_between_exclusive;
use super::coord_plane::great_circle_dist;
use core::f64::consts::PI;
use core::f64::consts::FRAC_PI_2;
use core::f64::consts::FRAC_PI_8;


//Takes list of pairs, and draws lines from the first pair to the second, and so on
//The last pair connects back to the first pair
#[allow(dead_code)]
pub fn connect_lines(corners: Vec<(f64, f64)>, num_points: usize) -> Vec<(f64, f64)> {

    //Normal order
    corners.iter() 

        //Same order, but one "ahead"
        .zip(corners.iter().cycle().skip(1)) 

        .map(|(start, end)| 

             //Connect the lines from n to n+1 in the list, starting over from 0
             //if it goes out of bounds
             line_exclusive(*start, *end, num_points)).flatten()
        .collect()
}

#[allow(dead_code)]
fn line_exclusive(a: (f64, f64), b: (f64, f64), num_points: usize) -> Vec<(f64, f64)> {
    let (a_x, a_y) = a;
    let (b_x, b_y) = b;

    points_between_exclusive(a_x, b_x, num_points).iter()
        .zip(points_between_exclusive(a_y, b_y, num_points).iter())
        .map(|(i, j)| (*i, *j)).collect()
}

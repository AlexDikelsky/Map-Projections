use super::coord_plane::points_between_exclusive;
use core::f64::consts::PI;

pub struct Circle {
    pub center: (f64, f64),
    pub radius: f64,
    pub num_points: usize,
}

impl Circle {
    fn make_circular_points(&self) -> Vec<(f64, f64)> {
        points_between_exclusive(0.0, 2.0 * PI, self.num_points).iter()
            .map(|x|
                (self.center.0 + (self.radius * x.cos()),
                 self.center.1 + (self.radius * x.sin())))
                .collect()
    }


    fn project(&self, projection_fn: Box<Fn(&Vec<(f64, f64)>) -> Vec<(f64, f64)>>)
            -> Vec<(f64, f64)> {
        projection_fn(&self.make_circular_points())
    }

    // This function pushes points further away from the radius
    // This is needed for drawing the tissot indicatrices, because the circle is supposed
    // to be infinitismal, then expanded for viewing rather than just a large circle
    // in the first place
    pub fn expand_to_indicatrix(&self, projection_fn: Box<Fn(&Vec<(f64, f64)>) -> Vec<(f64, f64)>>,
            increase_ratio: f64) -> Vec<(f64, f64)> {
        self.project(projection_fn).iter().map(
            |p| self.extend_line(*p, increase_ratio)).collect()
    }
            
    fn extend_line(&self, radius_point: (f64, f64), increase_ratio: f64) -> (f64, f64) {
        let x_diff = radius_point.0 - self.center.0;
        let y_diff = radius_point.1 - self.center.1;
        (self.center.0 + x_diff * increase_ratio,
         self.center.1 + y_diff * increase_ratio,
         )
    }

}


//Takes list of pairs, and draws lines from the first pair to the second, and so on
//The last pair connects back to the first pair
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


fn line_exclusive(a: (f64, f64), b: (f64, f64), num_points: usize) -> Vec<(f64, f64)> {
    let (a_x, a_y) = a;
    let (b_x, b_y) = b;

    points_between_exclusive(a_x, b_x, num_points).iter()
        .zip(points_between_exclusive(a_y, b_y, num_points).iter())
        .map(|(i, j)| (*i, *j)).collect()
}


//// Takes rectangular point, and returns num_points that are radius r away
//pub fn circle(point: (f64, f64), r: f64, num_points: usize) -> Vec<(f64, f64)> {
//    points_between_exclusive(0.0, 2.0 * PI, num_points).iter()
//        .map(|x|
//            (point.0 + (r * x.cos()),
//             point.1 + (r * x.sin()))
//            ).collect()
//}

use super::coord_plane::points_between_exclusive;
use core::f64::consts::PI;
use core::f64::consts::FRAC_PI_2;
const spots_to_check: usize = 100;

pub struct Circle {
    pub center: (f64, f64),
    pub radius: f64,
    pub num_points: usize,
}

impl Circle {


    //This function creates an "indicatrix" that would be correct if distances worked like
    //they do on a plane. Doesn't actually create an indicatrix.
    pub fn to_circlular_points(&self) -> super::tissot_indicatrix::Indicatrix {
        super::tissot_indicatrix::Indicatrix {points: points_between_exclusive(0.0, 2.0 * PI, self.num_points).iter()
            .map(|x|
                (self.center.0 + (self.radius * x.cos()),
                 self.center.1 + (self.radius * x.sin())))
                .collect(), 
            center: self.center,
        }
    }

    pub fn to_indicatrix(&self) -> super::tissot_indicatrix::Indicatrix {
        super::tissot_indicatrix::Indicatrix {
            points: {
                //These points all have the same great circle distance from the center

                // angle from center of sphere =
                //    acos(sin(φ1)sin(φ2) + cos(φ1)cos(φ2)cos(|λ1-λ2|)

                let sigma = self.radius;
                let lambda_2 = self.center.0;
                let phi_2 = self.center.1;

                //Idea is this:
                //if (rand_point statisfies 
                //    angle == acos(sin(φ1)sin(φ2) + cos(φ1)cos(φ2)cos(|λ1-λ2|))) {
                //    add point
                //}

                let max_dist = 1.0;

                let lat_bound = (lambda_2 - max_dist, lambda_2 + max_dist);
                let lon_bound = (phi_2 - max_dist, phi_2 + max_dist);

                let guess_points:
                    Vec<(f64, f64)> = 
                        points_between_exclusive(lat_bound.0, lat_bound.1, spots_to_check).iter().map(
                        |x| {
                            points_between_exclusive(lon_bound.0, lon_bound.1, spots_to_check).iter().map(
                            |y| (*x, *y) // tuple
                            ).collect::<Vec<(f64,f64)>>()  // finish first iterator
                        }
                    ).flatten().collect();

                guess_points.iter().filter(
                    |(lambda_1, phi_1)| {
                    
                        //dbg!(sigma, 
                        //        (phi_1.sin() * phi_2.sin() + 
                        //    phi_1.cos() * phi_2.cos() * (**lambda_1 - lambda_2).abs().cos()).acos());

                        f64_in_tolerance(
                            sigma, (
                            phi_1.sin() * phi_2.sin() + 
                            phi_1.cos() * phi_2.cos() * (*lambda_1 - lambda_2).abs().cos()
                            ).acos(),
                            10.0)
                    })
                    .map(|(x,y)| (*x,*y)).collect()
            },
            center: self.center,
        }

    }

}

fn f64_in_tolerance(num1: f64, num2: f64, tolerance: f64) -> bool {
    (num1-num2).abs() < tolerance
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

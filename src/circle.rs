use super::coord_plane::points_between_exclusive;
use super::coord_plane::great_circle_dist;
use core::f64::consts::PI;
use core::f64::consts::FRAC_PI_2;
use core::f64::consts::FRAC_PI_8;

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

    // Generates list of points that closer than a certain threshold of distance
    // in spherical distance from a radius point
    // num_points_to_check = 1000
    // area_from_center = pi/8
    // diff... = 0.02
    pub fn to_indicatrix(&self, num_points_to_check: usize,
                         area_from_center: f64, diff_between_angle_and_calculated: f64
                         ) -> super::tissot_indicatrix::Indicatrix {
        super::tissot_indicatrix::Indicatrix {
            points: {
                //These points all have the same great circle distance from the center

                // angle from center of sphere =
                //    acos(sin(φ1)sin(φ2) + cos(φ1)cos(φ2)cos(|λ1-λ2|)

                let sigma = self.radius;

                //Idea is this:
                //if (rand_point statisfies 
                //    angle == acos(sin(φ1)sin(φ2) + cos(φ1)cos(φ2)cos(|λ1-λ2|))) {
                //    add point
                //}
                gen_guess_points(self.center, num_points_to_check, area_from_center).iter().filter(
                |test_point| {
                    f64_in_tolerance(
                        sigma, 
                        great_circle_dist(**test_point, self.center),
                        diff_between_angle_and_calculated)
                })
                .map(|(x,y)| (*x,*y)).collect()

            },
            center: self.center,
        }

    }

}

fn gen_guess_points(start_point: (f64, f64), points_to_check: usize, tolerance: f64) -> Vec<(f64, f64)> {
    let lat_bound = (start_point.1 - tolerance, start_point.1 + tolerance);
    let lon_bound = (start_point.0 - tolerance, start_point.0 + tolerance);

    //println!("{:?}, {:?}", lat_bound, lon_bound);

    points_between_exclusive(lat_bound.0, lat_bound.1, points_to_check).iter().map(
        |x| {
            points_between_exclusive(lon_bound.0, lon_bound.1, points_to_check).iter().map(
            |y| (*y, *x)
            ).collect::<Vec<(f64,f64)>>()
        }
    ).flatten().collect()
}


fn f64_in_tolerance(num1: f64, num2: f64, tolerance: f64) -> bool {
    (num1-num2).abs() < tolerance
}

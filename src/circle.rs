use super::coord_plane::points_between_exclusive;
use super::coord_plane::great_circle_dist;
use super::coord_plane::LatLonPoint;
use core::f64::consts::PI;
use core::f64::consts::FRAC_PI_2;
use core::f64::consts::FRAC_PI_8;

pub struct Circle {
    pub center: LatLonPoint,
    pub radius: f64,
    pub num_points: usize,
}

impl Circle {
    //This function creates an "indicatrix" that would be correct if distances worked like
    //they do on a plane. Doesn't actually create an indicatrix.
    //pub fn to_circlular_points(&self) -> super::tissot_indicatrix::Indicatrix {
    //    super::tissot_indicatrix::Indicatrix {points: points_between_exclusive(0.0, 2.0 * PI, self.num_points).iter()
    //        .map(|x|
    //            (self.center.0 + (self.radius * x.cos()),
    //             self.center.1 + (self.radius * x.sin())))
    //            .collect(), 
    //        center: self.center,
    //    }
    //}

    // Generates list of points that closer than a certain threshold of distance
    // in spherical distance from a radius point
    // num_points_to_check = 1000
    // area_from_center = pi/8
    // diff... = 0.02
    pub fn to_indicatrix(&self, num_points_to_check: usize,
                         area_from_center: f64) -> super::tissot_indicatrix::Indicatrix {
        super::tissot_indicatrix::Indicatrix {
            points: {
                //These points all have the same great circle distance from the center

                // angle from center of sphere =
                //    acos(sin(φ1)sin(φ2) + cos(φ1)cos(φ2)cos(|λ1-λ2|)
                //
                // This means you can add all points where the angle is less than sigma,
                // and get an approximation of the "right" values

                let sigma = self.radius;

                dbg!(self.center);

                //Idea is this:
                //if (rand_point statisfies 
                //    angle > acos(sin(φ1)sin(φ2) + cos(φ1)cos(φ2)cos(|λ1-λ2|))) {
                //    add point
                //}
                gen_guess_points(self.center, num_points_to_check, area_from_center).iter().filter(
                |test_point| great_circle_dist(**test_point, self.center) < sigma).map(|x| *x)
                .collect()

            },
            center: self.center,
        }

    }

}

fn gen_guess_points(start_point: LatLonPoint, points_to_check: usize, tolerance: f64) -> Vec<LatLonPoint> {
    let lat_bound = (start_point.phi - tolerance, start_point.phi + tolerance);
    let lon_bound = (start_point.lambda - tolerance, start_point.lambda + tolerance);

    //println!("{:?}, {:?}", lat_bound, lon_bound);

    points_between_exclusive(lat_bound.0, lat_bound.1, points_to_check).iter().map(
        |phi| {
            points_between_exclusive(lon_bound.0, lon_bound.1, points_to_check).iter().map(
            |lambda| LatLonPoint { 
                lambda: *lambda, 
                phi: *phi 
            }).collect::<Vec<LatLonPoint>>()
        }
    ).flatten().collect()
}



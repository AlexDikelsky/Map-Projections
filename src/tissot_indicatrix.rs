use core::f64::consts::FRAC_PI_2;
use crate::draw_projection::draw;
use crate::circle::Circle;
use crate::coord_plane::LatLonPoint;
use crate::coord_plane::CartPoint;

pub struct Indicatrix {
    pub center: LatLonPoint,
    pub points: Vec<LatLonPoint>,
}

struct CartPointsWithCenter {
    pub center: CartPoint,
    pub points: Vec<CartPoint>,
}

impl CartPointsWithCenter {
    // This function pushes points further away from the radius
    pub fn expand(self, increase_ratio: f64) -> Vec<CartPoint> {
        self.points.iter().map(
            |end| self.extend_line(*end, increase_ratio)
            ).collect()
    }

    pub fn extend_line(&self, end: CartPoint, increase_ratio: f64) -> CartPoint {
        let x_diff = end.x - self.center.x;
        let y_diff = end.y - self.center.y;
        CartPoint {
            x: self.center.x + x_diff * increase_ratio,
            y: self.center.y + y_diff * increase_ratio,
        }
    }
}


impl Indicatrix {

    // Transform according to projection
    fn project<'a>(self, mapping_function: &'a Box<dyn Fn(Vec<LatLonPoint>) -> Vec<CartPoint>>) -> CartPointsWithCenter {
        CartPointsWithCenter {
            points: mapping_function(self.points),
            center: mapping_function(vec![self.center])[0],
        }
    }

}


pub fn gen_indicatrices(mapping_function: Box<dyn Fn(Vec<LatLonPoint>) -> Vec<CartPoint>>, num_lines: usize, 
                        num_points: usize, area_to_check: f64, radius: f64) -> Vec<CartPoint> {
    super::lat_lon_lines::intersections_of_pars_and_merids(num_lines).iter()
        .map(|p| 
             Circle { 
                 center: *p,
                 radius: radius,
                 num_points: num_points,
             }.to_indicatrix(num_points, area_to_check).project(&mapping_function)
             .expand(75.0))
    .flatten().collect()
}

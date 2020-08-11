use core::f64::consts::FRAC_PI_2;
use super::coord_plane::points_between_exclusive_both_ends;
use super::circle::Circle;
use super::coord_plane::LatLonPoint;
use super::coord_plane::CartPoint;

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
            |end| extend_line(self.center, *end, increase_ratio)
            ).collect()
    }
}


impl Indicatrix {

    // Transform according to projection
    fn project(self, mapping_function: &Box<dyn Fn(Vec<LatLonPoint>) -> Vec<CartPoint>>) -> CartPointsWithCenter {
        CartPointsWithCenter {
            points: mapping_function(self.points),
            center: mapping_function(vec![self.center])[0],
        }
    }

}


pub fn gen_indicatrices(mapping_function: Box<dyn Fn(Vec<LatLonPoint>) -> Vec<CartPoint>>, num_lines: usize, num_points: usize, area_to_check: f64, threshold: f64) -> Vec<CartPoint> {
    intersections_of_pars_and_merids(num_lines).iter()
        .map(|p| 
             Circle { 
                 center: *p,
                 radius: super::INDICATRIX_SIZE,
                 num_points: num_points,
             }.to_indicatrix(num_points, area_to_check, threshold).project(&mapping_function)
             .expand(5.0))
    .flatten().collect()
}

fn intersections_of_pars_and_merids(num_lines: usize) -> Vec<LatLonPoint> {
    points_between_exclusive_both_ends(-FRAC_PI_2, FRAC_PI_2, num_lines-2).iter()
        .map(|x: &f64| points_between_exclusive_both_ends(-FRAC_PI_2, FRAC_PI_2, num_lines-2).iter()
             .map(|y: &f64| 
                  LatLonPoint { lambda: *x, phi: *y } 
                  ).collect::<Vec<LatLonPoint>>())
        .flatten().collect()
}


fn extend_line(start: CartPoint, end: CartPoint, increase_ratio: f64) -> CartPoint {
    let x_diff = end.x - start.x;
    let y_diff = end.y - start.y;
    CartPoint {
        x: start.x + x_diff * increase_ratio,
        y: start.y + y_diff * increase_ratio,
    }
}

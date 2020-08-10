use core::f64::consts::FRAC_PI_2;
use super::coord_plane::points_between_inclusive;
use super::shapes::Circle;

pub struct Indicatrix {
    pub center: (f64, f64),
    pub points: Vec<(f64, f64)>,
}

impl Indicatrix {

    // Transform according to projection
    pub fn project(self, mapping_function: &Box<dyn Fn(Vec<(f64, f64)>) -> Vec<(f64,f64)>>) -> Self {
        Indicatrix {
            points: mapping_function(self.points),
            center: mapping_function(vec![self.center])[0],
        }
    }

    // This function pushes points further away from the radius
    pub fn expand(self, increase_ratio: f64) -> Vec<(f64, f64)> {
        self.points.iter().map(
            |end| extend_line(self.center, *end, increase_ratio)
            ).collect()
    }
}


pub fn gen_indicatrices(mapping_function: Box<dyn Fn(Vec<(f64, f64)>) -> Vec<(f64,f64)>>, num_lines: usize, num_points: usize) -> Vec<(f64, f64)> {
    intersections_of_pars_and_merids(num_lines).iter()
        .map(|p| 
             Circle { 
                 center: *p,
                 radius: super::INDICATRIX_SIZE,
                 num_points: num_points,
             }.to_indicatrix().project(&mapping_function)
             .expand(5.0))
    .flatten().collect()
}

fn intersections_of_pars_and_merids(num_lines: usize) -> Vec<(f64, f64)> {
    points_between_inclusive(-FRAC_PI_2, FRAC_PI_2, num_lines).iter()
        .map(|x: &f64| points_between_inclusive(-FRAC_PI_2, FRAC_PI_2, num_lines).iter()
             .map(|y: &f64| (*x, *y)).collect::<Vec<(f64, f64)>>())
        .flatten().collect()
}


fn extend_line(start: (f64, f64), end: (f64, f64), increase_ratio: f64) -> (f64, f64) {
    let x_diff = end.0 - start.0;
    let y_diff = end.1 - start.1;
    (start.0 + x_diff * increase_ratio,
     start.1 + y_diff * increase_ratio,
     )
}

use core::f64::consts::PI;
use core::f64::consts::FRAC_PI_2;
use std::num::FpCategory;

use crate::coord_plane::CartPoint;
use crate::coord_plane::map_one_point;
use crate::coord_plane::LatLonPoint;

#[derive(Debug)]
pub struct MapBounds {
    pub upper_x: f64,
    pub lower_x: f64,
    pub upper_y: f64,
    pub lower_y: f64,
}

pub enum BoundLocation {
    LowerLeftUpperRight,
    UpperLeftLowerRight,
    MaxXandY,
    Zeros,
}

impl MapBounds {

    #[allow(dead_code)]
    pub fn new(mapping_function: &Box<dyn Fn(Vec<LatLonPoint>) -> Vec<CartPoint>>, 
           bound_loc: BoundLocation) -> Self 
    {
        match bound_loc {
            BoundLocation::LowerLeftUpperRight => {
                let lower_left =
                    map_one_point(mapping_function, LatLonPoint {
                        lambda: -FRAC_PI_2, phi: -FRAC_PI_2
                    });
                
                let upper_right = 
                    map_one_point(mapping_function, LatLonPoint {
                        lambda: FRAC_PI_2, phi: FRAC_PI_2
                    });

                MapBounds {
                    upper_x: upper_right.x,
                    upper_y: upper_right.y,
                    lower_x: lower_left.x,
                    lower_y: lower_left.y,
                }
            },
            BoundLocation::UpperLeftLowerRight => {
                let upper_left = 
                    map_one_point(mapping_function, LatLonPoint {
                        lambda: -FRAC_PI_2, phi: FRAC_PI_2
                    });
                let lower_right = 
                    map_one_point(mapping_function, LatLonPoint {
                        lambda: FRAC_PI_2, phi: -FRAC_PI_2
                    });

                MapBounds {
                    upper_x: lower_right.x,
                    upper_y: upper_left.y,
                    lower_x: upper_left.x,
                    lower_y: lower_right.y,
                }
            },
            BoundLocation::MaxXandY => MapBounds {
                 upper_y: map_one_point(&mapping_function,
                     LatLonPoint { lambda: 0.0, phi: FRAC_PI_2 }).y, 
                 lower_y: map_one_point(&mapping_function,
                     LatLonPoint { lambda: 0.0, phi: -FRAC_PI_2 }).y, 
                 upper_x: map_one_point(&mapping_function,
                     LatLonPoint { lambda: FRAC_PI_2, phi: 0.0 }).x, 
                 lower_x: map_one_point(&mapping_function,
                     LatLonPoint { lambda: -FRAC_PI_2, phi: 0.0 }).x,
            },
            BoundLocation::Zeros => MapBounds {
                upper_x: 0.0,
                upper_y: 0.0,
                lower_x: 0.0,
                lower_y: 0.0,
            }
        }
    }

    #[allow(dead_code)]
    pub fn add_size(self, inc: f64) -> Self {
        MapBounds {
            upper_x: self.upper_x + inc,
            upper_y: self.upper_y + inc,
            lower_x: self.lower_x - inc,
            lower_y: self.lower_y - inc,
        }
    }

    #[allow(dead_code)]
    pub fn to_vec(&self) -> Vec<f64> {
        vec![
            self.upper_x, self.upper_y, 
            self.lower_x, self.lower_y
        ]
    }

    #[allow(dead_code)]
    pub fn bounds_from_vec(arr: Vec<f64>) -> MapBounds {
        MapBounds {
            upper_x: arr[0],
            upper_y: arr[1],
            lower_x: arr[2],
            lower_y: arr[3],
        }
    }


    #[allow(dead_code)]
    pub fn to_normal_vals(&self, inf_approx: f64) -> MapBounds {
        MapBounds::bounds_from_vec(
            self.to_vec().iter().map(
                |float| match float.classify() {
                    FpCategory::Normal => {
                        //If value is greater than "infinity", 
                        match float.abs() > inf_approx.abs() {
                            true => inf_approx.copysign(*float),
                            false => *float
                        }
                    }

                    //If you use 0, it can cause the plotting library to hang
                    FpCategory::Zero => 0.00001,

                    FpCategory::Infinite => {
                        match *float > 0.0 {
                            true => inf_approx,
                            false => -inf_approx,
                        }
                    },
                    FpCategory::Nan => panic!("0/0 or inf/inf division attempted"),
                    FpCategory::Subnormal => *float,
                }
                ).collect::<Vec<f64>>()
            )
    }
}

use core::f64::consts::FRAC_PI_4;
use super::coord_plane::PolarPoint;
use super::coord_plane::LatLonPoint;
use super::coord_plane::CartPoint;
use super::coord_plane::great_circle_dist;


pub enum ProjectionType {
    Conic,
    PseudoConic,
    Azimuthal,
    PseudoAzimuthal,
    Cylindric,
    PseudoCylindric,
    Arbitrary,
}


pub struct projection {
    projection_function: Box<dyn Fn(Vec<LatLonPoint>) -> Vec<CartPoint>>,
    name: String,
    projection_type: ProjectionType,
}




pub fn chain_projections<'a>(fns: &'a [&'a Box<dyn Fn(Vec<LatLonPoint>) -> Vec<CartPoint>>]) 
    -> Box<dyn Fn(Vec<LatLonPoint>) -> Vec<CartPoint> + 'a>
{
    match fns {
        [head, tail @ ..] => Box::new(
            move |x| chain_projections(tail)(head(x).iter().map(|point| {
                (*point).to_latlon_raw()}).collect()
                )),
        [] => Box::new(equirectangular),
    }
}

//This function allows the chaining of previous functions
//Doesn't necessarily create a reasonable map projection
//pub fn chain_projections<'a>(mapping_functions: &'a Vec<&'a Box<dyn Fn(Vec<LatLonPoint>) -> Vec<CartPoint>>>)
//    -> &'a Box<dyn Fn(Vec<LatLonPoint>) -> Vec<CartPoint> + 'a>
//{
//    mapping_functions.iter().skip(1).fold(
//        &Box::new(mapping_functions[0]) as &Box<dyn Fn(Vec<LatLonPoint>) -> Vec<CartPoint>>, 
//        |prev, cur| {
//            Box::new(move |points| {
//                cur(
//                    prev(points).iter().map(move |point| { 
//                        point.to_latlon_raw() 
//                    }).collect()
//                )
//            }) 
//        }
//    )
//}

use core::f64::consts::FRAC_PI_4;
use super::coord_plane::PolarPoint;
use super::coord_plane::LatLonPoint;
use super::coord_plane::CartPoint;
use super::coord_plane::great_circle_dist;

#[allow(dead_code)]
pub fn mercator(points: Vec<LatLonPoint>) -> Vec<CartPoint> {
    points.iter().map(
        |llpoint| CartPoint {
            x: llpoint.lambda, 
            y: (FRAC_PI_4 + llpoint.phi/2.0).tan().ln()}
            ).collect()
}

#[allow(dead_code)]
pub fn bonne(points: Vec<LatLonPoint>, central_meridian: f64) -> Vec<CartPoint> {
    //Technically around for a long time before Boone
    //Based off Werner's cordiform projections, which was based off
    //of Ptolemey's second projection
    //
    //Equal area and correct scale along central meridan and parallels

    let phi_1: f64 = central_meridian;
    let cot_phi_1 = 1.0/(phi_1.tan());
    points.iter().map(
        |llpoint| {
            let rho = cot_phi_1 + phi_1 - llpoint.phi;
            let theta = llpoint.lambda * (llpoint.phi.cos() / rho);
            PolarPoint { rho: rho, theta: theta }.to_cart()
        }).collect()
}

#[allow(dead_code)]
pub fn simple_equidistant_conic(points: Vec<LatLonPoint>, 
                                standard_par1: f64, standard_par2: f64) -> Vec<CartPoint> {
    let phi_1: f64 = standard_par1;
    let phi_2: f64 = standard_par2;

    let temp = (phi_2 * phi_1.cos() - 
                phi_1 * phi_2.cos())
        / (phi_1.cos() - phi_2.cos());

    let n = (phi_1.cos() - phi_2.cos())/(phi_2 - phi_1);

    points.iter().map(
        |llpoint| {
            let rho = temp - llpoint.phi;
            let theta = n * llpoint.lambda;
            PolarPoint { rho: rho, theta: theta }.to_cart()
        }).collect()
}

#[allow(dead_code)]
pub fn equirectangular(points: Vec<LatLonPoint>) -> Vec<CartPoint> {
    points.iter().map(
        |llpoint| CartPoint {
            x: llpoint.lambda,
            y: llpoint.phi,
        }
        ).collect()
}

#[allow(dead_code)]
pub fn sinusoidal(points: Vec<LatLonPoint>) -> Vec<CartPoint> {
    points.iter().map(|llpoint|
                      CartPoint { 
                          x: llpoint.lambda * llpoint.phi.cos(), 
                          y: llpoint.phi
                      }).collect()
}

#[allow(dead_code)]
pub fn lambert_conformal_conic(points: Vec<LatLonPoint>, 
                                standard_par1: f64, standard_par2: f64) -> Vec<CartPoint> {
    let phi_1 = standard_par1;
    let phi_2 = standard_par2;

    let n = (
            (phi_1.cos().ln() -
            phi_2.cos().ln()) 
        ) / (
            (FRAC_PI_4 + phi_2).tan().ln() -
            (FRAC_PI_4 + phi_1));

    points.iter().map(
        |llpoint| {
            let theta = n * llpoint.lambda;
            let rho = 
                phi_1.cos() *
                (FRAC_PI_4 + phi_1/2.0).tan().powf(n)
                /
                (n * (FRAC_PI_4 + llpoint.phi/2.0).tan().powf(n));
            PolarPoint { rho: rho, theta: theta }.to_cart()
        }).collect()
}

//#[allow(dead_code)]
//pub fn lambert_azimuthal_equal_area(points: Vec<LatLonPoint>, center_lat: f64) -> Vec<CartPoint> {
//    let phi_1 = center_lat;
//
//    points.iter().map(
//        |llpoint| {
//            let sin_phi = llpoint.phi.sin();
//            let sin_phi_1 = phi_1.sin();
//            let sin_lambda = llpoint.lambda.sin();
//            let cos_phi = llpoint.phi.cos();
//            let cos_phi_1 = phi_1.cos();
//            let cos_lambda = llpoint.lambda.cos();
//
//            let z = (sin_phi_1 * sin_phi + 
//                cos_phi_1 * cos_phi * cos_lambda).acos();
//
//            //Check vs sin_theta
//            let cos_theta =
//                (cos_phi_1 * sin_phi - sin_phi_1 * cos_phi * cos_lambda)
//                / (z.sin());
//
//            let sin_theta = 
//                cos_phi * sin_lambda / z.sin();



    

//Broken
//pub fn stereographic(points: Vec<LatLonPoint>) -> Vec<CartPoint> {
//    points.iter().map(
//        |llpoint| {
//            let rho = (llpoint.lambda / 2.0).tan().recip();
//            let theta = llpoint.phi;
//
//            PolarPoint { rho, theta }.to_cart()
//        }).collect()
//}

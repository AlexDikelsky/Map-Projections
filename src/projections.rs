use core::f64::consts::FRAC_PI_4;
use super::coord_plane::polar_to_cartesian;

#[allow(dead_code)]
pub fn mercator(points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    points.iter().map(
        |(x, y)| (*x, (FRAC_PI_4 + y/2.0).tan().ln())).collect()
}

#[allow(dead_code)]
pub fn bonne(points: Vec<(f64, f64)>, central_meridian: f64) -> Vec<(f64, f64)> {
    //Technically around for a long time before Boone
    //Based off Werner's cordiform projections, which was based off
    //of Ptolemey's second projection
    //
    //Equal area and correct scale along central meridan and parallels

    let phi_1: f64 = central_meridian;
    let cot_phi_1 = 1.0/(phi_1.tan());
    points.iter().map(
        |(lambda, phi)| {
            let rho = cot_phi_1 + phi_1 - phi;
            let theta = lambda * (phi.cos() / rho);
            polar_to_cartesian((rho, theta))
        }).collect()
}

#[allow(dead_code)]
pub fn simple_equidistant_conic(points: Vec<(f64, f64)>, 
                                standard_par1: f64, standard_par2: f64) -> Vec<(f64, f64)> {
    let phi_1: f64 = standard_par1;
    let phi_2: f64 = standard_par2;

    let temp = (phi_2 * phi_1.cos() - 
                phi_1 * phi_2.cos())
        / (phi_1.cos() - phi_2.cos());

    let n = (phi_1.cos() - phi_2.cos())/(phi_2 - phi_1);

    points.iter().map(
        |(lambda, phi)| {
            let rho = temp - phi;
            let theta = n * lambda;
            polar_to_cartesian((rho, theta))
        }).collect()
}

#[allow(dead_code)]
pub fn sinusoidal(points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    points.iter().map(|(lambda, phi)|
                      (lambda * phi.cos(), *phi)).collect()
}

#[allow(dead_code)]
pub fn lambert_conformal_conic(points: Vec<(f64, f64)>, 
                                standard_par1: f64, standard_par2: f64) -> Vec<(f64, f64)> {
    let phi_1 = standard_par1;
    let phi_2 = standard_par2;

    let n = (
            (phi_1.cos().ln() -
            phi_2.cos().ln()) 
        ) / (
            (FRAC_PI_4 + phi_2).tan().ln() -
            (FRAC_PI_4 + phi_1));

    points.iter().map(
        |(lambda, phi)| {
            let theta = n * lambda;
            let rho = 
                phi_1.cos() *
                (FRAC_PI_4 + phi_1/2.0).tan().powf(n)
                /
                (n * (FRAC_PI_4 + phi/2.0).tan().powf(n));
            polar_to_cartesian((rho, theta))
        }).collect()
}


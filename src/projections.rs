use core::f64::consts::PI;

#[allow(dead_code)]
pub fn mercator(points: &Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    points.iter().map(
        |(x, y)| (*x, (PI/4.0 + y/2.0).tan().ln())).collect()
}

#[allow(dead_code)]
pub fn bonne(points: &Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    let phi_1: f64 = PI/2.0;
    let rho_0: f64 = 0.0;
    let cot_phi_1 = 1.0/(phi_1.tan());
    points.iter().map(
        |(lambda, phi)| {
            let rho = cot_phi_1 + phi_1 - phi;
            let theta = lambda * (phi.cos() / rho);
            let x = rho * theta.sin();
            let y = rho_0 - rho * theta.cos();
            (x, y)
        }).collect()
}

#[allow(dead_code)]
pub fn equidistant_conic(points: &Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    let rho_0: f64 = 0.0;
    let phi_1: f64 = 0.0;
    let phi_2: f64 = PI/4.0;

    let temp = (phi_2 * phi_1.cos() - 
                phi_1 * phi_2.cos())
        / (phi_1.cos() - phi_2.cos());

    let n = (phi_1.cos() - phi_2.cos())/(phi_2 - phi_1);

    points.iter().map(
        |(lambda, phi)| {
            let rho = temp - phi;
            let theta = n * lambda;

            let x = rho * theta.sin();
            let y = rho_0 - rho * theta.cos();
            (x, y)
        }).collect()
}


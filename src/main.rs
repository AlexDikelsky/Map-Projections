use plotters::prelude::*;
use std::iter;

const PI: f64 = 3.1415926;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("map.png", (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    //Modify number of lines here
    //You will always see one more line than you put here because both -π/2 and
    //π/2 are plotted for symmetry
    let num_lines = 4;
    let num_points = 100;
    let bound = 2.0*PI/2.0;

    //Add parallels and meridians
    let mut points: Vec<(f64, f64)> = merids(num_lines, num_points);
    points.extend(pars(num_lines, num_points));

    //Change the function called here to remap
    points = equidistant_conic(&points);

    let mut scatter_ctx = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_ranged(-bound..bound, -bound..bound)?;
    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;
    scatter_ctx.draw_series(
        points
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 2, BLACK.filled())),
    )?;

    Ok(())
}

fn mercator(points: &Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    points.iter().map(
        |(x, y)| (*x, (PI/4.0 + y/2.0).tan().ln())).collect()
}

fn bonne(points: &Vec<(f64, f64)>) -> Vec<(f64, f64)> {
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

fn equidistant_conic(points: &Vec<(f64, f64)>) -> Vec<(f64, f64)> {
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

        




fn merids(num_merids: usize, num_points: usize) -> Vec<(f64, f64)> {
    points_between(0.0, PI/2.0, num_points)
        .iter().copied()
        .map(|x| meridian(x, num_merids)).flatten().collect()
}

fn pars(num_pars: usize, num_points: usize) -> Vec<(f64, f64)> {
    merids(num_pars, num_points).iter().map(|(x, y)| (*y, *x)).collect()
}

// takes radian degree between 0 and π/2
// integral num points
// returns meridian with num points on both "sides"
fn meridian(deg: f64, num: usize) -> Vec<(f64, f64)> {
    let lat_vals : Vec<f64> = points_between(-(PI/2.0), PI/2.0, num);
    lat_vals.iter().map(|x| {
        vec![(*x, deg), (*x, deg-(PI/2.0))]
    }).flatten().collect()
    //(0..num).map(|x| 
    //             ((this_side[x], deg), (other_side[x], deg)))
    //    .flatten().collect()
}

//num points between min and max, not including max
// works with min>max, but goes backward
fn points_between(min: f64, max: f64, num: usize) -> Vec<f64> {
    let inc = (max - min) / (num as f64);
    (0..num).map(|x| min + ((x as f64) * inc))
        .chain(iter::once(max))
        .collect()
}

use plotters::prelude::*;
use core::f64::consts::PI;
use core::f64::consts::FRAC_PI_2;

mod projections;
mod coord_plane;
mod shapes;
mod tissot_indicatrix;

const INDICATRIX_SIZE: f64 = 0.001;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let root = BitMapBackend::new("map.png", (1024, 1024)).into_drawing_area();

    root.fill(&WHITE)?;

    //num_lines changes number of parallels and meridians
    let num_lines = 8;
    //num_points changes how many points are plotted on each meridian
    let num_points = 1000;
    //Size of the graph. For Mercator projection, should be larger
    let bound = 2.0;

    //Add parallels and meridians
    let mut points: Vec<(f64, f64)> = coord_plane::sphere_coords(num_lines, num_points);

    //****************************
    //Set mapping function here
    //****************************
    // If the function takes more than one argument, define them before this line
    let modified_map = |vals| 
        projections::simple_equidistant_conic(vals, FRAC_PI_2 * (3.0/4.0), FRAC_PI_2 * (1.0/4.0));
        //projections::bonne(vals, FRAC_PI_2);

    let mapping_function = modified_map;


    //***
    //Add shapes here to see distortion
    //***
    //
    //**** Square
    //points.extend(shapes::connect_lines(
    //        vec![
    //            (0.0, 0.0),
    //            (1.0, 0.0),
    //            (1.0, 1.0),
    //            (0.0, 1.0),
    //        ], 500));
    //**** Circle
    //points.extend(shapes::circle((-1.0, 1.0), 2.0, 100));


    points = mapping_function(&points);
    let indic = tissot_indicatrix::circles(num_lines, num_points);

    //points = projections::mercator(&indic);

    //points.extend(projections::mercator(&indic));

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

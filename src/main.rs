use plotters::prelude::*;
use core::f64::consts::PI;
use core::f64::consts::FRAC_PI_2;

mod projections;
mod coord_plane;
mod shapes;

fn main() -> Result<(), Box<dyn std::error::Error>> {



    let root = BitMapBackend::new("map.png", (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    //num_lines changes number of parallels and meridians
    let num_lines = 8;
    //num_points changes how many points are plotted on each meridian
    let num_points = 1000;
    //Size of the graph. For Mercator projection, should be larger
    let bound = PI*1.5;

    //Add parallels and meridians
    let mut points: Vec<(f64, f64)> = coord_plane::sphere_coords(num_lines, num_points);

    //***
    //Add shapes here to see distortion
    //***
    //points.extend(shapes::connect_lines(
    //        vec![
    //            (0.0, 0.0),
    //            (1.0, -1.0),
    //            (1.0, 1.0),
    //            (0.0, 1.0),
    //        ], 500));

    // *******************************************
    // Change the function called here to remap
    // Running one at a time produces that type of projection
    // *******************************************
    //points = projections::mercator(&points);
    //points = projections::bonne(&points, FRAC_PI_2);
    points = projections::simple_equidistant_conic(&points, FRAC_PI_2 * (3.0/4.0), FRAC_PI_2 * (1.0/4.0));


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

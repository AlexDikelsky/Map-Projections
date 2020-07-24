use plotters::prelude::*;
use core::f64::consts::PI;

mod projections;
mod coord_plane;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("map.png", (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    //Modify number of lines here
    //You will always see one more line than you put here because both -π/2 and
    //π/2 are plotted for symmetry
    let num_lines = 4;
    let num_points = 100;
    let bound = PI;

    //Add parallels and meridians
    let mut points: Vec<(f64, f64)> = coord_plane::sphere_coords(num_lines, num_points);

    //Change the function called here to remap
    //points = projections::mercator(&points);
    points = projections::bonne(&points, PI/2.0);

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

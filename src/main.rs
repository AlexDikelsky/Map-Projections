mod projections;
mod coord_plane;
mod shapes;
mod circle;
mod tissot_indicatrix;
mod map_bounds;

use plotters::prelude::*;

use core::f64::consts::PI;
use core::f64::consts::FRAC_PI_2;
use core::f64::consts::FRAC_PI_4;
use coord_plane::CartPoint;
use coord_plane::LatLonPoint;
use map_bounds::MapBounds;
use map_bounds::BoundLocation;

const INDICATRIX_SIZE: f64 = 0.00001;
const GEN_INDIC: bool = true;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let root = BitMapBackend::new("map.png", (1024, 1024)).into_drawing_area();

    root.fill(&WHITE)?;

    //num_lines changes number of parallels and meridians
    let num_lines = 7;
    //num_points changes how many points are plotted on each meridian
    let num_points = 1000;
    //Size of the graph.
    //set guess_bound to true if you want it to be guessed based off the 
    //bounds of the projection with bound as an extenstion
    let guess_bound = true;
    let force_normal = true;
    let bound = 0.5;

    //Add parallels and meridians
    let lat_lon_points: Vec<LatLonPoint> = coord_plane::sphere_coords(num_lines, num_points);

    //****************************
    //Set mapping function here
    //****************************
    // If the function takes more than one argument, define them before this line
    let mapping_function: Box<dyn std::ops::Fn(std::vec::Vec<LatLonPoint>) -> std::vec::Vec<CartPoint>> = 
        //Box::new(|vals| projections::simple_equidistant_conic(vals.to_vec(), FRAC_PI_2 * (3.0/4.0), FRAC_PI_2 * (1.0/4.0)));
        //Box::new(|vals| projections::bonne(vals, FRAC_PI_2));
        //Box::new(projections::mercator);
        //Box::new(projections::equirectangular); 
        //Box::new(projections::sinusoidal);
        //Box::new(|vals| projections::lambert_conformal_conic(vals.to_vec(), FRAC_PI_4, PI * 12.0f64.recip()));
        //Box::new(projections::stereographic);
        //Box::new(|vals| projections::loximuthal(vals.to_vec(), 0.3 * PI));

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
 
    
    let bounds = MapBounds::new(
        &mapping_function, BoundLocation::MaxXandY).add_size(bound)
        .to_normal_vals(10.0);

    let upper_x = bounds.upper_x;
    let lower_x = bounds.lower_x;
    let upper_y = bounds.upper_y;
    let lower_y = bounds.lower_y;

    dbg!(bounds);

    let mut scatter_ctx = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_ranged(lower_x..upper_x, lower_y..upper_y)?;
    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;


    let mapped_points = mapping_function(lat_lon_points);


    scatter_ctx.draw_series(
        mapped_points
            .iter()
            .map(|point| Circle::new(point.to_tuple(), 2, BLACK.filled())),
    )?;


    if GEN_INDIC {
        let area_to_check = 0.4;
        let threshold = 0.01;
        let indic = tissot_indicatrix::gen_indicatrices(Box::new(mapping_function), num_lines, num_points,
            area_to_check, threshold);
        scatter_ctx.draw_series(
            indic.iter()
                .map(|point| Circle::new(point.to_tuple(), 2, RED.filled()))
                )?;
    }

    Ok(())
}

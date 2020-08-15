mod projections;
mod coord_plane;
mod shapes;
mod circle;
mod tissot_indicatrix;
mod map_bounds;
mod one_dim_lines;
mod lat_lon_lines;
mod chart_and_js_exports;
mod draw_projection;


//fn main() -> Result<(), Box<dyn std::error::Error>> {
//
//    let root = BitMapBackend::new("map.png", (1024, 1024)).into_drawing_area();
//
//    root.fill(&WHITE)?;
//
//    //num_lines changes number of parallels and meridians
//    let num_lines = 7;
//    //num_points changes how many points are plotted on each meridian
//    let num_points = 100;
//    //Size of the graph.
//    //set guess_bound to true if you want it to be guessed based off the 
//    //bounds of the projection with bound as an extenstion
//    //
//    //
//    //Note that conformal projections will have non-circular indicatrices if
//    //the bounds aren't square
//    let bound = 3.0;
//
//    //Add parallels and meridians
//    let lat_lon_points: Vec<LatLonPoint> = lat_lon_lines::sphere_coords(num_lines, num_points);
//
//    //****************************
//    //Set mapping function here
//    //****************************
//    // If the function takes more than one argument, define them before this line
////    let _simple_equidistant_conic = |vals: Vec<LatLonPoint>| projections::simple_equidistant_conic(vals.to_vec(), FRAC_PI_1 * (3.0/4.0), FRAC_PI_2 * (1.0/4.0));
//    let _bonne = Box::new(|vals: Vec<LatLonPoint>| projections::pseudoconic::bonne(vals, FRAC_PI_4)) as Box<dyn std::ops::Fn(std::vec::Vec<coord_plane::LatLonPoint>) -> std::vec::Vec<coord_plane::CartPoint>>;
////    let _mercator = projections::mercator;
////    let _equirectangular = projections::equirectangular;
////    let _sinusoidal = projections::sinusoidal;
////    let _lambert_conformal_conic = |vals: Vec<LatLonPoint>| projections::lambert_conformal_conic(vals.to_vec(), FRAC_PI_3, PI * 12.0f64.recip());
////    let _stereographic = projections::stereographic;
////    let _loximuthal = |vals: Vec<LatLonPoint>| projections::loximuthal(vals.to_vec(), -1.3 * PI);
//
//    //let fns = [_sinusoidal, _bonne];
//
//    let mapping_function = _bonne;
//        
//       // Box::new(move |x|
//       // _loximuthal(_loximuthal(x).iter().map(|point| point.to_latlon_raw()).collect()))
//       // as Box<dyn std::ops::Fn(std::vec::Vec<coord_plane::LatLonPoint>) -> std::vec::Vec<coord_plane::CartPoint>>;
//
//    //***
//    //Add shapes here to see distortion
//    //***
//    //
//    //**** Square
//    //points.extend(shapes::connect_lines(
//    //        vec![
//    //            (0.0, 0.0),
//    //            (1.0, 0.0),
//    //            (1.0, 1.0),
//    //            (0.0, 1.0),
//    //        ], 500));
// 
//    
//    let bounds = MapBounds::new(
//        &mapping_function, BoundLocation::Zeros).add_size(bound)
//        .to_normal_vals(10.0);
//
//    let upper_x = bounds.upper_x;
//    let lower_x = bounds.lower_x;
//    let upper_y = bounds.upper_y;
//    let lower_y = bounds.lower_y;
//
//    dbg!(bounds);
//
//    let mut scatter_ctx = ChartBuilder::on(&root)
//        .x_label_area_size(40)
//        .y_label_area_size(40)
//        .build_ranged(lower_x..upper_x, lower_y..upper_y)?;
//    scatter_ctx
//        .configure_mesh()
//        .disable_x_mesh()
//        .disable_y_mesh()
//        .draw()?;
//
//
//    let mapped_points = mapping_function(lat_lon_points);
//
//
//    scatter_ctx.draw_series(
//        mapped_points
//            .iter()
//            .map(|point| Circle::new(point.to_tuple(), 2, BLACK.filled())),
//    )?;
//
//    if GEN_INDIC {
//        let area_to_check = 0.02;
//        let indic_radius = 0.1 / 180.0 * PI;  // One tenth of a deg
//        let indic = tissot_indicatrix::gen_indicatrices(Box::new(mapping_function), num_lines, num_points, area_to_check, indic_radius);
//        scatter_ctx.draw_series(
//            indic.iter()
//                .map(|point| Circle::new(point.to_tuple(), 2, RED.filled()))
//                )?;
//    }
//
//    Ok(())
//}

//pub fn power(canvas_id: &str, power: i32) -> Result<Chart, JsValue> {
//    let map_coord = func_plot::draw(canvas_id, power).map_err(|err| err.to_string())?;
//    Ok(Chart {
//        convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
//    })
//}
//

use core::f64::consts::PI;

use crate::chart_and_js_exports::DrawResult;
use plotters::prelude::*;
use crate::map_bounds::MapBounds;

use crate::coord_plane::LatLonPoint;
use crate::lat_lon_lines::sphere_coords;

use crate::projections::projection_by_name;
use crate::projections::projection_types::ProjectionParams;
use crate::projections::projection_types::Projection;


pub fn draw(canvas_id: &str, map_projection_name: String, num_lat_lon: usize, 
            tissot: bool, bounds: MapBounds) 
    -> DrawResult<impl Fn((i32, i32)) -> Option<(f64, f64)>>
{
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;
    
    let upper_x = bounds.upper_x;
    let lower_x = bounds.lower_x;
    let upper_y = bounds.upper_y;
    let lower_y = bounds.lower_y;

    let mut map_ctx = ChartBuilder::on(&root)
        .caption(format!("{}", map_projection_name), font)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(lower_x..upper_x, lower_y..upper_y)?;
    map_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    let points: Vec<LatLonPoint> = sphere_coords(num_lat_lon, 1000);

    let projection: Projection = projection_by_name::use_projection(
        "Simple Equidistant Conic".to_string())
        .expect("Projection not found");

    let mapped_points = (projection.projection_function)(
        ProjectionParams::PointsTwoStandardPar(points, PI * (3.0/4.0), PI * (1.0/4.0)));
        
    //map_ctx.draw_series(LineSeries::new(
    //    (-50..=50)
    //        .map(|x| x as f32 / 50.0)
    //        .map(|x| (x, -x.powf(8 as f32))),
    //    &RED,
    //))?;

    map_ctx.draw_series(
        mapped_points
        .iter()
        .map(|point| Circle::new(point.to_tuple(), 2, BLACK.filled())),
    )?;

    root.present()?;
    return Ok(map_ctx.into_coord_trans());
}

//pub fn power(canvas_id: &str, power: i32) -> Result<Chart, JsValue> {
//    let map_coord = func_plot::draw(canvas_id, power).map_err(|err| err.to_string())?;
//    Ok(Chart {
//        convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
//    })
//}
//

use crate::llib::DrawResult;
use plotters::prelude::*;

use crate::coord_plane::LatLonPoint;
use crate::lat_lon_lines::sphere_coords;


//Also should take:
//  tissot (bool)
//  function to map
//  bounds to use


///These comments may be shown 6:05 CDT
//#[wasm_bindgen]
pub fn draw(canvas_id: &str, num_lat_lon: usize) 
    -> DrawResult<impl Fn((i32, i32)) -> Option<(f64, f64)>>
{
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;

    let mut map_ctx = ChartBuilder::on(&root)
        .caption(format!("y=x^{}", 999), font)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(-1f64..1f64, -1.2f64..1.2f64)?;

    map_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    let points: Vec<LatLonPoint> = sphere_coords(num_lat_lon, 1000);

    let mapped_points = points;
        

    map_ctx.draw_series(
        mapped_points.iter()
        .map(|point| Circle::new((point.lambda, point.phi), 2, BLACK.filled())),
    )?;

    root.present()?;
    return Ok(map_ctx.into_coord_trans());
}

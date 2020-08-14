use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use super::draw_projection::draw;
use crate::map_bounds::MapBounds;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Type alias for the result of a drawing function.
pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Type used on the JS side to convert screen coordinates to chart
/// coordinates.
#[wasm_bindgen]
pub struct Chart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
}

/// Result of screen to chart coordinates conversion.
#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Chart {
    /// Draw selected map projection
    pub fn project(canvas_id: &str, map_projection_name: String, num_lat_lon: usize,
                   tissot: bool, bounds: Vec<f64>)
            -> Result<Chart, JsValue> {

        let map_coord = draw(canvas_id, map_projection_name, num_lat_lon, tissot, 
                             MapBounds::bounds_from_vec(bounds))
            .map_err(|err| err.to_string())?;

        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    /// This function can be used to convert screen coordinates to
    /// chart coordinates.
    pub fn coord(&self, x: i32, y: i32) -> Option<Point> {
        (self.convert)((x, y)).map(|(x, y)| Point { x, y })
    }
}

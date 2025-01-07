use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn draw_circle(context: &web_sys::CanvasRenderingContext2d, x: f64, y: f64, radius: f64) {
    context.begin_path();
    context
        .arc(x, y, radius, 0.0, std::f64::consts::PI * 2.0)
        .unwrap();
    context.fill();
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Object {
    position: (f64, f64),
    velocity: (f64, f64),
    mass: f64,
}

#[wasm_bindgen]
impl Object {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, vx: f64, vy: f64, mass: f64) -> Object {
        Object {
            position: (x, y),
            velocity: (vx, vy),
            mass,
        }
    }

    pub fn update_position(&mut self) {
        self.position.0 += 5.0;
    }

    pub fn x(&self) -> f64 {
        self.position.0
    }

    pub fn y(&self) -> f64 {
        self.position.1
    }
}

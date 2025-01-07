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

    pub fn x(&self) -> f64 {
        self.position.0
    }

    pub fn y(&self) -> f64 {
        self.position.1
    }
}

#[wasm_bindgen]
pub fn update_position(obj1: &mut Object, obj2: &Object) {
    const DELTA_TIME: f64 = 1.0;

    let dx = obj1.x() - obj2.x();
    let dy = obj1.y() - obj2.y();
    let distance = (dx.powi(2) + dy.powi(2)).sqrt();
    let force = (obj1.mass * obj2.mass) / distance.powi(2);

    let acceleration_x = force * dx / distance / obj1.mass;
    let acceleration_y = force * dy / distance / obj1.mass;

    obj1.velocity.0 += acceleration_x * DELTA_TIME;
    obj1.velocity.1 += acceleration_y * DELTA_TIME;

    obj1.position.0 += obj1.velocity.0 * DELTA_TIME;
    obj1.position.1 -= obj1.velocity.1 * DELTA_TIME;
}

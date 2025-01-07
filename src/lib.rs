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
    const delta_time: f64 = 1.0;

    let distance_x = obj1.x() - obj2.x();
    let distance_y = obj1.y() - obj2.y();
    let distance = (distance_x.powi(2) + distance_y.powi(2)).sqrt();
    let force = (obj1.mass * obj2.mass) / distance.powi(2);

    let acceleration_x = force * distance_x / distance / obj1.mass;
    let acceleration_y = force * distance_y / distance / obj2.mass;

    obj1.velocity.0 += acceleration_x * delta_time;
    obj1.velocity.1 += acceleration_y * delta_time;

    obj1.position.0 += obj1.velocity.0 * delta_time;
    obj1.position.1 -= obj1.velocity.1 * delta_time;
}

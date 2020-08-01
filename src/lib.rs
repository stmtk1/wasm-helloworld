mod rand;
mod boid;
mod vector;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use rand::Rand;
use boid::Boid;

#[wasm_bindgen]
pub fn get_new_boid(size: f64, seed: f64) -> JsValue {
    let mut rnd_gen = Rand::new(seed);
    let num = size as u64;

    JsValue::from_serde(&(0..num).map(|_| Boid::new(&mut rnd_gen)).collect::<Vec<Boid>>()).unwrap()
}

#[wasm_bindgen]
pub fn get_next_state(js_boids: &JsValue) -> JsValue {
    let boids: Vec<Boid> = js_boids.into_serde().unwrap();
    JsValue::from_serde(&(&boids).into_iter().map(|boid| boid.next_state(&boids)).collect::<Vec<Boid>>()).unwrap()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

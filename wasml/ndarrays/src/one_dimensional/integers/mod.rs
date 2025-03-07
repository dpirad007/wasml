pub mod basic;
pub mod custom_serde;
pub mod math;
pub mod wasm;

use ndarray::{arr1, Array1};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Integers1d {
    #[wasm_bindgen(skip)]
    pub data: Array1<i32>,
}

// All the rust only functions
impl Integers1d {
    /// Create a new Integers1d
    pub fn new(array: Vec<i32>) -> Integers1d {
        Integers1d { data: arr1(&array) }
    }
}

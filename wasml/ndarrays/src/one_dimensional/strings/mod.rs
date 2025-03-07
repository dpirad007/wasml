pub mod basic;
pub mod custom_serde;
pub mod wasm;

use ndarray::{arr1, Array1};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Strings1d {
    #[wasm_bindgen(skip)]
    pub data: Array1<String>,
}

// All the rust only functions
impl Strings1d {
    /// Create a new Strings1d
    pub fn new(array: Vec<String>) -> Strings1d {
        Strings1d { data: arr1(&array) }
    }
}

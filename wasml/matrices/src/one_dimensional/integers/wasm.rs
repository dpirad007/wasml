use super::Integers1d;
use ndarray::arr1;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Integers1d {
    /// Create a new Integers1d from javascript
    #[wasm_bindgen(constructor)]
    pub fn new_with_js(js_array: JsValue) -> Integers1d {
        let vector: Vec<i32> = js_array.into_serde().unwrap();
        Integers1d {
            data: arr1(&vector),
        }
    }

    /// Gives the value contained in the ndarray as a javascript array
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> JsValue {
        JsValue::from_serde(&self.data.to_vec()).unwrap()
    }

    /// Get the string representation of the underlying ndarray
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{:#?}", self.data)
    }
}

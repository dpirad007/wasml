use super::SeriesI32;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl SeriesI32 {
    pub fn sum(&self) -> i32 {
        self.data.sum()
    }

    pub fn product(&self) -> i32 {
        self.data.product()
    }

    pub fn mean(&self) -> f64 {
        let mut c = 0;
        let mut sum = 0;
        let total;
        self.data.data.iter().for_each(|x| {
            c += 1;
            sum += x;
        });

        total = sum as f64 / c as f64;
        total
    }

    pub fn median(&self) -> f64 {
        self.data.median()
    }

    pub fn max(&self) -> i32 {
        self.data.max()
    }

    pub fn min(&self) -> i32 {
        self.data.min()
    }
}

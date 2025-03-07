use super::Strings1d;
use ndarray::{s, Array1, Axis};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Strings1d {
    /// Get the length of the array
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Get the shape of the array as a javascript array
    #[wasm_bindgen(js_name = shape)]
    pub fn shape_to_js(&self) -> JsValue {
        JsValue::from_serde(self.data.shape()).unwrap()
    }

    /// Get the value at the specified index
    pub fn get(&self, index: usize) -> String {
        self.data[index].clone()
    }

    /// Set the value at the specified index
    pub fn set(&mut self, index: usize, value: String) {
        self.data[index] = value;
    }

    /// Swap the values at the specified indices
    pub fn swap(&mut self, index1: usize, index2: usize) {
        self.data.swap(index1, index2);
    }

    /// Reverse the ndarray
    pub fn reverse(&mut self) {
        self.data = self.data.slice(s![..;-1]).to_owned();
    }

    /// Return the array reversed
    pub fn reversed(&self) -> Strings1d {
        Strings1d {
            data: self.data.slice(s![..;-1]).to_owned(),
        }
    }

    /// Appending an element to the array
    pub fn append(&mut self, element: String) {
        let new = Array1::from_iter(
            self.data
                .clone()
                .into_iter()
                .map(|x| x.clone())
                .chain(std::iter::once(element)),
        );

        self.data = new;
    }

    /// Return the result of appending an element to the array
    pub fn appended(&mut self, element: String) -> Strings1d {
        let new = Array1::from_iter(
            self.data
                .clone()
                .into_iter()
                .map(|x| x.clone())
                .chain(std::iter::once(element)),
        );

        Strings1d { data: new }
    }

    /// Extend the array with another
    pub fn extend(&mut self, other: Strings1d) {
        let new = Array1::from_iter(
            self.data
                .clone()
                .into_iter()
                .map(|x| x.clone())
                .chain(other.data.clone().into_iter().map(|x| x.clone())),
        );

        self.data = new;
    }

    /// Return the result of extending the array with another
    pub fn extended(&self, other: Strings1d) -> Strings1d {
        let new = Array1::from_iter(
            self.data
                .clone()
                .into_iter()
                .map(|x| x.clone())
                .chain(other.data.clone().into_iter().map(|x| x.clone())),
        );

        Strings1d { data: new }
    }

    /// Insert a value at the specified index
    pub fn insert(&mut self, index: usize, value: String) {
        let (first, second) = self.data.view().split_at(Axis(0), index);
        let new = Array1::from_iter(
            first
                .iter()
                .map(|x| x.clone())
                .chain(std::iter::once(value))
                .chain(second.iter().map(|x| x.clone())),
        );

        self.data = new;
    }

    /// Return the array with the value inseted at the specified index
    pub fn inserted(&mut self, index: usize, value: String) -> Strings1d {
        let (first, second) = self.data.view().split_at(Axis(0), index);
        let new = Array1::from_iter(
            first
                .iter()
                .map(|x| x.clone())
                .chain(std::iter::once(value))
                .chain(second.iter().map(|x| x.clone())),
        );

        Strings1d { data: new }
    }

    /// Remove the value at the specified index and return the result
    pub fn splice(&mut self, index: usize) -> String {
        let val = self.data[index].clone();
        let (first, second) = self.data.multi_slice_mut((s![..index], s![index + 1..]));
        let new = Array1::from_iter(
            first
                .iter()
                .map(|x| x.clone())
                .chain(second.iter().map(|x| x.clone())),
        );

        self.data = new;

        val
    }

    /// Return the array after removing an element at the specified index
    /// and the element
    pub fn spliced(&mut self, index: usize) -> js_sys::Array {
        let val = self.data[index].clone();
        let (first, second) = self.data.multi_slice_mut((s![..index], s![index + 1..]));
        let new = Array1::from_iter(
            first
                .iter()
                .map(|x| x.clone())
                .chain(second.iter().map(|x| x.clone())),
        );

        let spliced = Strings1d { data: new };

        js_sys::Array::of2(&JsValue::from(spliced), &JsValue::from(val))
    }
}

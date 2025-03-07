use super::Integers1d;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Integers1d {
    /// Add two Integers1d's and return the result
    pub fn add(&self, other: &Integers1d) -> Integers1d {
        Integers1d {
            data: &self.data + &other.data,
        }
    }

    /// Substract two Integers1d's and return the result
    pub fn sub(&self, other: &Integers1d) -> Integers1d {
        Integers1d {
            data: &self.data - &other.data,
        }
    }

    /// Multiply two Integers1d's and return the result
    pub fn mul(&self, other: &Integers1d) -> Integers1d {
        Integers1d {
            data: &self.data * &other.data,
        }
    }

    /// Divide two Integers1d's and return the result
    pub fn div(&self, other: &Integers1d) -> Integers1d {
        Integers1d {
            data: &self.data / &other.data,
        }
    }

    /// Efficiently performs self += alpha * rhs
    pub fn scaled_add(&mut self, alpha: i32, other: Integers1d) {
        self.data.scaled_add(alpha, &other.data);
    }

    /// Get the sum of all the elements in the array
    pub fn sum(&self) -> i32 {
        self.data.sum()
    }

    /// Get the product of all the elements in the array
    pub fn product(&self) -> i32 {
        self.data.product()
    }

    /// Get minimun element
    pub fn min(&self) -> i32 {
        let mut min: i32 = self.data[0];
        self.data.iter().for_each(|elem| {
            if min > *elem {
                min = *elem;
            }
        });
        min
    }

    /// Get maximun element
    pub fn max(&self) -> i32 {
        let mut max: i32 = self.data[0];
        self.data.iter().for_each(|elem| {
            if max < *elem {
                max = *elem;
            }
        });
        max
    }

    /// Get the mean of all the elements in the array
    pub fn mean(&self) -> f64 {
        let mut sum: i32 = 0;
        for x in &self.data {
            sum += x;
        }

        sum as f64 / self.len() as f64
    }

    /// Get median of all elements
    pub fn median(&self) -> f64 {
        let mut d = self.data.to_vec();
        d.sort_by(|a, b| {
            a.partial_cmp(b).unwrap()
        });
        
        let mid = d.len() / 2;
        if d.len() % 2 == 0 {
            (d[mid-1] + d[mid]) as f64 / 2.0
        } else {
            d[mid] as f64
        }
    }

    /// Get variance of all elements in array
    pub fn variance(&self, degree_of_freedom: f64) -> f64 {
        let mean = self.mean();
        let mut sqr_diff: f64 = 0.0;
        
        for x in &self.data {
            sqr_diff += (*x as f64 - mean) * (*x as f64 - mean);
        }

        return sqr_diff as f64 / (self.len() as f64 - degree_of_freedom);
    }

    /// Get standard_deviation of all elements in array
    pub fn standard_deviation(&self, degree_of_freedom: f64) -> f64 {
        self.variance(degree_of_freedom).sqrt()
    }
}

use super::ColumnType;
use super::DataFrame;
use super::Series;
use crate::series::floats::SeriesF64;
use crate::series::integers::SeriesI32;
use crate::series::strings::SeriesSTR;
use ndarrays::one_dimensional::floats::Floats1d;
use wasm_bindgen::prelude::*;

// macro_rules! MMM {
//     ($fn_name: ident) => {
//         pub fn $fn_name(&self) -> Floats1d {
//             let mut res: Vec<f64> = Vec::new();
//             self.data.iter().for_each(|ser| match &ser {
//                 Series::Integers(value) => {
//                     res.push(value.mean());
//                 }
//                 Series::Floats(value) => {
//                     res.push(value.mean());
//                 }
//             });

//             Floats1d::new(res)
//         }
//     }
// }

#[wasm_bindgen]
impl DataFrame {
    #[wasm_bindgen(constructor)]
    pub fn new(vec_series: Vec<JsValue>) -> DataFrame {
        //Get first Series data size
        let mut series_size = 0;
        let first_series_int: Result<SeriesI32, serde_wasm_bindgen::Error> =
            serde_wasm_bindgen::from_value(vec_series[0].clone());
        if let Ok(series_int) = first_series_int {
            series_size = series_int.len()
        }

        let first_series_float: Result<SeriesF64, serde_wasm_bindgen::Error> =
            serde_wasm_bindgen::from_value(vec_series[0].clone());
        if let Ok(series_float) = first_series_float {
            series_size = series_float.len()
        }

        let first_series_str: Result<SeriesSTR, serde_wasm_bindgen::Error> =
            serde_wasm_bindgen::from_value(vec_series[0].clone());
        if let Ok(series_str) = first_series_str {
            series_size = series_str.len()
        }

        let series_data = vec_series
            .iter()
            .map(|series| {
                let as_int: Result<SeriesI32, serde_wasm_bindgen::Error> =
                    serde_wasm_bindgen::from_value(series.clone());

                if let Ok(x) = as_int {
                    if x.len() == series_size {
                        return Series::Integers(x);
                    }
                }

                let as_float: Result<SeriesF64, serde_wasm_bindgen::Error> =
                    serde_wasm_bindgen::from_value(series.clone());

                if let Ok(x) = as_float {
                    if x.len() == series_size {
                        return Series::Floats(x);
                    }
                }

                let as_str: Result<SeriesSTR, serde_wasm_bindgen::Error> =
                    serde_wasm_bindgen::from_value(series.clone());

                if let Ok(x) = as_str {
                    if x.len() == series_size {
                        return Series::Strings(x);
                    }
                }

                panic!("Type couldn't be matched");
            })
            .collect();

        DataFrame { data: series_data }
    }

    #[wasm_bindgen(js_name = columns)]
    pub fn show_columns(&self) -> JsValue {
        let mut res: Vec<String> = Vec::new();
        self.data.iter().for_each(|ser| {
            match ser {
                Series::Integers(x) => res.push(x.name()),
                Series::Floats(x) => res.push(x.name()),
                Series::Strings(x) => res.push(x.name()),
            };
        });

        serde_wasm_bindgen::to_value(&res).unwrap()
    }

    #[wasm_bindgen(getter, js_name = dTypes)]
    pub fn show_datatypes(&self) -> JsValue {
        let mut res: Vec<ColumnType> = Vec::new();
        self.data.iter().for_each(|ser| {
            match ser {
                Series::Integers(x) => res.push(x.dtype()),
                Series::Floats(x) => res.push(x.dtype()),
                Series::Strings(x) => res.push(x.dtype()),
            };
        });

        serde_wasm_bindgen::to_value(&res).unwrap()
    }

    #[wasm_bindgen(js_name = append)]
    pub fn add_column(&mut self, datatype: ColumnType, series: JsValue) {
        // let dt: ColumnType = serde_wasm_bindgen::from_value(datatype).unwrap();
        match datatype {
            ColumnType::FLOAT => {
                let ser = serde_wasm_bindgen::from_value(series).unwrap();
                self.data.push(Series::Floats(ser));
            }
            ColumnType::INTEGER => {
                let ser = serde_wasm_bindgen::from_value(series).unwrap();
                self.data.push(Series::Integers(ser));
            }
            ColumnType::STR => {
                let ser = serde_wasm_bindgen::from_value(series).unwrap();
                self.data.push(Series::Strings(ser));
            }
        }
    }

    #[wasm_bindgen(js_name = size)]
    pub fn dataframe_size(&self) -> usize {
        self.data.iter().count()
    }

    pub fn loc(&self, column_name: String) -> String {
        let mut res = String::from("");
        self.data.iter().for_each(|ser| {
            match ser {
                Series::Integers(x) => {
                    if x.name() == column_name {
                        res = x.show();
                    }
                }
                Series::Floats(x) => {
                    if x.name() == column_name {
                        res = x.show();
                    }
                }
                Series::Strings(x) => {
                    if x.name() == column_name {
                        res = x.show();
                    }
                }
            };
        });

        res
    }

    pub fn ilocr(&self, row: usize) -> js_sys::Array {
        let array = js_sys::Array::new();
        self.data.iter().for_each(|ser| {
            match ser {
                Series::Integers(x) => {
                    let val = serde_wasm_bindgen::to_value(&x.get(row)).unwrap();
                    array.push(&val);
                }
                Series::Floats(x) => {
                    let val = serde_wasm_bindgen::to_value(&x.get(row)).unwrap();
                    array.push(&val);
                }
                Series::Strings(x) => {
                    let val = serde_wasm_bindgen::to_value(&x.get(row)).unwrap();
                    array.push(&val);
                }
            };
        });

        array
    }

    pub fn ilocc(&self, col: usize) -> JsValue {
        let val: JsValue;
        let ser = &self.data[col];
        match ser {
            Series::Integers(x) => {
                val = x.data();
            }
            Series::Floats(x) => {
                val = x.data();
            }
            Series::Strings(x) => {
                val = x.data();
            }
        };
        val
    }

    #[wasm_bindgen(getter,js_name = display)]
    pub fn show(&self) -> String {
        let mut res: String = String::from("");
        self.data.iter().for_each(|ser| match &ser {
            Series::Integers(value) => {
                res.push_str(&value.show());
            }
            Series::Floats(value) => {
                res.push_str(&value.show());
            }
            Series::Strings(value) => {
                res.push_str(&value.show());
            }
        });
        res
    }

    pub fn min(&self) -> Floats1d {
        let mut res: Vec<f64> = Vec::new();
        self.data.iter().for_each(|ser| match &ser {
            Series::Integers(value) => {
                res.push(value.min() as f64);
            }
            Series::Floats(value) => {
                res.push(value.min());
            }
            _ => panic!(),
        });

        Floats1d::new(res)
    }

    pub fn max(&self) -> Floats1d {
        let mut res: Vec<f64> = Vec::new();
        self.data.iter().for_each(|ser| match &ser {
            Series::Integers(value) => {
                res.push(value.max() as f64);
            }
            Series::Floats(value) => {
                res.push(value.max());
            }
            _ => panic!(),
        });

        Floats1d::new(res)
    }

    // MMM!(mean);
    pub fn mean(&self) -> Floats1d {
        let mut res: Vec<f64> = Vec::new();
        self.data.iter().for_each(|ser| match &ser {
            Series::Integers(value) => {
                res.push(value.mean());
            }
            Series::Floats(value) => {
                res.push(value.mean());
            }
            _ => panic!(),
        });

        Floats1d::new(res)
    }

    pub fn median(&self) -> Floats1d {
        let mut res: Vec<f64> = Vec::new();
        self.data.iter().for_each(|ser| match &ser {
            Series::Integers(value) => {
                res.push(value.median());
            }
            Series::Floats(value) => {
                res.push(value.median());
            }
            _ => panic!(),
        });

        Floats1d::new(res)
    }
}

pub mod models;
mod utils;

extern crate nalgebra as na;
use models::js_complex::JSComplex;
use models::js_matrix::JSMatrix;
use wasm_bindgen::prelude::*;

#[allow(unused_unsafe)]
#[wasm_bindgen(start)]
pub fn run() {
    utils::set_panic_hook();
    if let Some(window) = web_sys::window() {
        if let Some(_) = window.document() {
            unsafe { log(&"WASM says: It's Ok") };
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn solve(a: &JsValue, b: &JsValue) -> JsValue {
    match a.into_serde::<JSMatrix<f32>>() {
        Ok(a_js_matrix) => match b.into_serde::<JSMatrix<f32>>() {
            Ok(b_js_matrix) => match a_js_matrix.solve(b_js_matrix) {
                Ok(x_js_matrix) => match JsValue::from_serde(&x_js_matrix) {
                    Ok(value) => value,
                    _ => JsValue::from_str(&"Internal Error"),
                },
                _ => JsValue::from_str(&"Error on solve Ax=b"),
            },
            _ => JsValue::from_str(&"Error on parse b matrix"),
        },
        _ => JsValue::from_str(&"Error on parse A matrix"),
    }
}
#[wasm_bindgen]
pub fn complex_solve(a: &JsValue, b: &JsValue) -> JsValue {
    match a.into_serde::<JSMatrix<JSComplex<f32>>>() {
        Ok(a_js_matrix) => match b.into_serde::<JSMatrix<JSComplex<f32>>>() {
            Ok(b_js_matrix) => match a_js_matrix.complex_solve(b_js_matrix) {
                Ok(x_js_matrix) => match JsValue::from_serde(&x_js_matrix) {
                    Ok(value) => value,
                    _ => JsValue::from_str(&"Internal Error"),
                },
                _ => JsValue::from_str(&"Error on solve Ax=b"),
            },
            _ => JsValue::from_str(&"Error on parse b matrix"),
        },
        _ => JsValue::from_str(&"Error on parse A matrix"),
    }
}

use axb::models::js_complex::JSComplex;
use axb::models::js_matrix::{JSMatrix, JSMatrixElement};
use axb::{complex_solve, solve};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_solve_eye() {
    let mut a_eye = JSMatrix::<f32>::default();
    a_eye.ncols = 3;
    a_eye.nrows = 3;
    a_eye.elements.push(JSMatrixElement {
        row: 0,
        col: 0,
        value: 1.,
    });
    a_eye.elements.push(JSMatrixElement {
        row: 1,
        col: 1,
        value: 1.,
    });
    a_eye.elements.push(JSMatrixElement {
        row: 2,
        col: 2,
        value: 1.,
    });

    let mut b = JSMatrix::<f32>::default();
    b.ncols = 1;
    b.nrows = 3;
    b.elements.push(JSMatrixElement {
        row: 0,
        col: 0,
        value: 3.,
    });
    b.elements.push(JSMatrixElement {
        row: 1,
        col: 0,
        value: 2.,
    });
    b.elements.push(JSMatrixElement {
        row: 2,
        col: 0,
        value: 1.,
    });

    let a_jsvalue = JsValue::from_serde(&a_eye).unwrap();
    let b_jsvalue = JsValue::from_serde(&b).unwrap();
    let result: JSMatrix<f32> = solve(&a_jsvalue, &b_jsvalue).into_serde().unwrap();

    assert_eq!(result, b);
}

#[wasm_bindgen_test]
fn test_solve_complex() {
    let value = JSComplex { re: 1., im: 1. };
    let mut a_complex = JSMatrix::<JSComplex<f32>>::default();
    a_complex.ncols = 2;
    a_complex.nrows = 2;
    a_complex.elements.push(JSMatrixElement {
        row: 0,
        col: 0,
        value: value,
    });
    a_complex.elements.push(JSMatrixElement {
        row: 1,
        col: 1,
        value: value,
    });

    let mut b = JSMatrix::<JSComplex<f32>>::default();
    b.ncols = 1;
    b.nrows = 2;
    b.elements.push(JSMatrixElement {
        row: 0,
        col: 0,
        value: JSComplex { re: 1., im: 0. },
    });
    b.elements.push(JSMatrixElement {
        row: 1,
        col: 0,
        value: JSComplex { re: 0., im: 1. },
    });

    let mut expected = JSMatrix::<JSComplex<f32>>::default();
    expected.ncols = 1;
    expected.nrows = 2;
    expected.elements.push(JSMatrixElement {
        row: 0,
        col: 0,
        value: JSComplex { re: 0.5, im: -0.5 },
    });
    expected.elements.push(JSMatrixElement {
        row: 1,
        col: 0,
        value: JSComplex { re: 0.5, im: 0.5 },
    });

    let a_jsvalue = JsValue::from_serde(&a_complex).unwrap();
    let b_jsvalue = JsValue::from_serde(&b).unwrap();
    let result: JSMatrix<JSComplex<f32>> = complex_solve(&a_jsvalue, &b_jsvalue).into_serde().unwrap();

    assert_eq!(result, expected);
}

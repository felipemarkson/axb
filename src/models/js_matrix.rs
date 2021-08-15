use super::errors::MatrixError;
use super::{js_complex::JSComplex, rs_matrix::RSMatrix};
use num::Complex;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct JSMatrixElement<T> {
    pub row: usize,
    pub col: usize,
    pub value: T,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct JSMatrix<T> {
    pub nrows: usize,
    pub ncols: usize,
    pub elements: Vec<JSMatrixElement<T>>,
}

impl<T> From<RSMatrix<T>> for JSMatrix<T>
where
    T: Default + na::ComplexField + Copy,
{
    fn from(matrix: RSMatrix<T>) -> Self {
        let mut data = JSMatrix::<T>::default();
        data.nrows = matrix.nrows();
        data.ncols = matrix.ncols();

        for (i, row) in matrix.row_iter().enumerate() {
            for (m, &value) in row.into_iter().enumerate() {
                if !value.is_zero() {
                    let element = JSMatrixElement {
                        row: i,
                        col: m,
                        value: value,
                    };
                    data.elements.push(element);
                }
            }
        }
        data
    }
}

impl<T> From<JSMatrix<JSComplex<T>>> for JSMatrix<Complex<T>>
where
    T: Default + na::RealField,
{
    fn from(val: JSMatrix<JSComplex<T>>) -> Self {
        let mut data = JSMatrix::default();
        data.ncols = val.ncols;
        data.nrows = val.nrows;

        for element in val.elements.into_iter() {
            let new_element = JSMatrixElement {
                row: element.row,
                col: element.col,
                value: element.value.into(),
            };
            data.elements.push(new_element)
        }

        data
    }
}

impl<T> From<JSMatrix<Complex<T>>> for JSMatrix<JSComplex<T>>
where
    T: Default + na::RealField,
{
    fn from(val: JSMatrix<Complex<T>>) -> Self {
        let mut data = JSMatrix::default();
        data.ncols = val.ncols;
        data.nrows = val.nrows;

        for element in val.elements.into_iter() {
            let new_element = JSMatrixElement {
                row: element.row,
                col: element.col,
                value: element.value.into(),
            };
            data.elements.push(new_element)
        }

        data
    }
}

impl<T> JSMatrix<T>
where
    T: Default + na::RealField + Copy,
{
    pub fn solve(self, b: Self) -> Result<Self, MatrixError> {
        let a_rs: RSMatrix<T> = self.into();
        let b_rs = b.into();
        match a_rs.solve(b_rs) {
            Ok(x_rs) => Ok(Self::from(x_rs)),
            Err(e) => Err(e),
        }
    }
}

impl<T> JSMatrix<JSComplex<T>>
where
    T: Default + na::RealField + Copy,
{
    pub fn complex_solve(self, b: Self) -> Result<Self, MatrixError> {
        let a_complex: RSMatrix<Complex<T>> = self.into();
        let b_complex = b.into();

        match a_complex.solve(b_complex) {
            Ok(value) => Ok(JSMatrix::from(value).into()),
            Err(e) => Err(e),
        }
    }
}

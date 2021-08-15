use super::errors::MatrixError;
use super::js_complex::JSComplex;
use super::js_matrix::JSMatrix;
use na::iter::RowIter;
use na::{DMatrix, Dynamic, VecStorage};
use nalgebra::Complex;

pub struct RSMatrix<T>(pub DMatrix<T>);

impl<T> RSMatrix<T>
where
    T: Default + na::ComplexField,
{
    pub fn zeros(nrows: usize, ncols: usize) -> Self {
        RSMatrix {
            0: DMatrix::from_element(nrows, ncols, T::zero()),
        }
    }

    pub fn nrows(&self) -> usize {
        self.0.nrows()
    }
    pub fn ncols(&self) -> usize {
        self.0.ncols()
    }
    pub fn row_iter(&self) -> RowIter<T, Dynamic, Dynamic, VecStorage<T, Dynamic, Dynamic>> {
        self.0.row_iter()
    }

    pub fn solve(self, mut b: Self) -> Result<Self, MatrixError> {
        match self.0.lu().solve_mut(&mut b.0) {
            true => Ok(b),
            false => Err(MatrixError::SolveError),
        }
    }
}

impl<T> From<JSMatrix<T>> for RSMatrix<T>
where
    T: Default + na::ComplexField,
{
    fn from(matrix: JSMatrix<T>) -> Self {
        let mut data = RSMatrix::zeros(matrix.nrows, matrix.ncols);
        for element in matrix.elements.into_iter() {
            data.0[(element.row, element.col)] = element.value
        }
        data
    }
}

impl<T> From<JSMatrix<JSComplex<T>>> for RSMatrix<Complex<T>>
where
    T: Default + na::RealField,
{
    fn from(a: JSMatrix<JSComplex<T>>) -> Self {
        let b: JSMatrix<Complex<T>> = JSMatrix::from(a);
        RSMatrix::from(b)
    }
}

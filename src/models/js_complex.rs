use na::Complex;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Clone, Copy, Debug, PartialEq)]
pub struct JSComplex<T> {
    pub re: T,
    pub im: T,
}

impl<T> From<JSComplex<T>> for Complex<T>
where
    T: Default + na::RealField,
{
    fn from(value: JSComplex<T>) -> Complex<T> {
        Complex {
            re: value.re,
            im: value.im,
        }
    }
}

impl<T> From<Complex<T>> for JSComplex<T>
where
    T: Default + na::RealField,
{
    fn from(value: Complex<T>) -> Self {
        JSComplex {
            re: value.re,
            im: value.im,
        }
    }
}

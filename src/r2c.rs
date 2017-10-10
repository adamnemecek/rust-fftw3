use super::FLAG;
use super::aligned_vec::*;
use super::error::*;
use super::pair::{Pair, ToPair};
use super::plan::R2C;
use super::traits::*;

use ffi;

use ndarray::*;
use ndarray_linalg::Scalar;

/// Setting for 1-dimensional R2C transform
#[derive(Debug, Clone, Copy, new)]
pub struct R2C1D {
    n: usize,
    flag: FLAG,
}

/// Utility function to generage 1-dimensional R2C setting
pub fn r2c_1d(n: usize) -> R2C1D {
    R2C1D {
        n,
        flag: ffi::FFTW_MEASURE,
    }
}

impl<R, C> ToPair<R, C> for R2C1D
where
    (R, C): R2C<Real = R, Complex = C>,
    R: FFTWReal,
    C: FFTWComplex<Real = R::Real>,
{
    type Dim = Ix1;
    fn to_pair(&self) -> Result<Pair<R, C, Ix1>> {
        let mut a = AlignedVec::<R>::new(self.n);
        let mut b = AlignedVec::<C>::new(self.n / 2 + 1);
        let forward = unsafe { <(R, C) as R2C>::r2c_1d(self.n, &mut a, &mut b, self.flag) };
        let backward = unsafe { <(R, C) as R2C>::c2r_1d(self.n, &mut b, &mut a, self.flag) };
        Pair {
            a: (a, self.n.into_shape()),
            b: (b, (self.n / 2 + 1).into_shape()),
            forward,
            backward,
            factor_f: Some(Scalar::from_f64(1.0 / self.n as f64)),
            factor_b: None,
        }.null_checked()
    }
}

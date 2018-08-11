mod linear;
mod poly;
mod rbf;

use simd_aligned::{f32s, RowOptimized, SimdMatrix, SimdVector};

pub use self::{linear::*, poly::*, rbf::*};

/// Base trait for kernels
#[doc(hidden)]
pub trait Kernel
where
    Self: Sync,
{
    fn compute(
        &self,
        vectors: &SimdMatrix<f32s, RowOptimized>,
        feature: &SimdVector<f32s>,
        output: &mut [f64],
    );
}

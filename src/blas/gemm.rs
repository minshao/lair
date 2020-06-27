use crate::Scalar;

#[allow(clippy::too_many_arguments)]
#[allow(clippy::cast_possible_wrap)]
pub unsafe fn gemm<A>(
    nrows: usize,
    ncols: usize,
    k: usize,
    a: *const A,
    b: *const A,
    c: *mut A,
    row_stride: isize,
    col_stride: isize,
) where
    A: Scalar,
{
    for p in 0..nrows as isize {
        for q in 0..ncols as isize {
            *c.offset(p * row_stride + q * col_stride) -= (0..k as isize)
                .map(|r| {
                    *a.offset(p * row_stride + r * col_stride)
                        * *b.offset(r * row_stride + q * col_stride)
                })
                .sum();
        }
    }
}
//! Bitwise magic arithmetic utilities

use num_traits::PrimInt;

/// Arithmetic utilities using bitwise magic
pub trait ArithUtil: PrimInt + Copy {
    /// The number of bits in the type
    const BITS: Self;
    /// The minimum value of the type
    const MIN: Self;
    /// The maximum value of the type
    const MAX: Self;
    /// Calculate `ceil(log2(self))` using bitwise magic
    fn ceil_log2(self) -> Self;
    /// Divide using integer division,
    /// but round up instead of down
    fn divide_round_up(self, divisor: Self) -> Self;
}
macro_rules! impl_prim_int {
    ($($target:ty),*) => {
        $(impl ArithUtil for $target {
            const BITS: $target = <$target>::BITS;
            const MIN: $target = <$target>::MIN;
            const MAX: $target = <$target>::MAX;
            #[inline]
            fn ceil_log2(self) -> $target {
                Self::BITS - (self - 1).leading_zeros()
            }
            #[inline(always)]
            fn divide_round_up(self, divisor: $target) -> $target {
                assert!(divisor != 0, "Division by zero");
                assert!(divisor != Self::MIN, "Division underflow");
                unsafe {
                    (self + divisor.uncheceked_sub(1)).unchecked_div(divisor)
                }
            }
        })*
    };
}
impl_prim_int!(u32, u64, usize);
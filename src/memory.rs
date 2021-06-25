//! The abstract interface to the implementation
//! of memory
//!
//! One of the features of this crate
//! is that the interface to memory
//! is abstract, allowing plugable implementations.
//!
//! This is handy for users who need special
//! FFI compatibility or are writing
//! a garbage collected language implementation.
use std::fmt::Debug;

/// A single word in an arbitrary precision
/// arithmetic.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord)]
pub struct Word(pub u64);
impl Word {
    pub const BITS: u64 = 64;
}

/// A trait for allocation errors
pub trait IAllocError: std::error::Error {
    /// Create an error indicating that capacity arithmetic overflowed
    fn capacity_arithmetic_overflow() -> Self;
}

/// An array of [Words](Word)
pub unsafe trait WordArray: AsRef<[Word]> + AsMut<[Word]> + Clone + Debug + Default {
    const EMPTY: Self;
    /// An error indicating that allocation failed
    type AllocErr: IAllocError;
    /// Pre-allocate the array with the specified capacity
    fn with_capacity(capacity: usize) -> Result<Self, Self::AllocErr>;

    /// The length of the array
    fn len(&self) -> usize;

    /// Empty the array, setting the length to zero
    fn clear(&mut self);

    /// The capacity of the array
    fn capacity(&self) -> usize;

    /// Ensure the array's capacity is at least the specified size
    ///
    /// Errors if allocation fails.
    fn reserve(&mut self, additional: usize) -> Result<(), Self::AllocErr>;

    /// Push the specified [Word] onto the array
    ///
    /// Errors if the array needs to re-allocate,
    /// and that allocation fails
    #[inline]
    fn push(&mut self, word: Word) -> Result<(), Self::AllocErr> {
        unsafe {
            self.reserve(1)?;
            self.unchecked_push(word);
            Ok(())
        }
    }

    /// Push the specified [Word] onto the array,
    /// without checking if the [WordArray::capacity] is large enough
    ///
    /// ## Safety
    /// Undefined behavior if the capacity is insufficient
    unsafe fn unchecked_push(&mut self, word: Word);
}

impl IAllocError for ! {
    #[cold]
    fn capacity_arithmetic_overflow() -> Self {
        panic!("Capacity arithmetic overflow")
    }
}
unsafe impl WordArray for Vec<Word> {
    const EMPTY: Self = Vec::new();
    type AllocErr = !;
    #[inline]
    fn with_capacity(capacity: usize) -> Result<Self, !> {
        Ok(Vec::with_capacity(capacity))
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn clear(&mut self) {
        self.clear();
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.capacity()
    }

    #[inline]
    fn reserve(&mut self, capacity: usize) -> Result<(), Self::AllocErr> {
        let () = self.reserve(capacity);
        Ok(())
    }

    #[inline]
    unsafe fn unchecked_push(&mut self, word: Word) {
        debug_assert!(self.len() + 1 <= self.capacity());
        let end = self.as_mut_ptr().add(self.len());
        end.write(word);
        self.set_len(self.len().unchecked_add(1));
    }
}
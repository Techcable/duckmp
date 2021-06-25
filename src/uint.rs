//! Unsigned integers
use num_traits::{Num, Zero, One};

use crate::string::{ParseIntError};
use crate::memory::{WordArray, Word, IAllocError};
use std::ops::Add;


/// An unsigned integer
///
/// Memory is managed via the
/// specified [ArrayType]
pub struct UnsignedInteger<A: WordArray = Vec<Word>> {
    /// The internal array of words
    pub(crate) words: A
}
impl<A: WordArray> Add for UnsignedInteger<A> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(&rhs).unwrap()
    }
}
impl<A: WordArray> Zero for UnsignedInteger<A> {
    #[inline]
    fn zero() -> Self {
        UnsignedInteger::ZERO
    }

    #[inline]
    fn set_zero(&mut self) {
        self.words.clear();
    }

    #[inline]
    fn is_zero(&self) -> bool {
        #[cfg(debug_assertions)] {
            if self.words.len() > 0 {
                debug_assert!(
                    self.words().iter().all(|word| word.0.is_zero())
                )
            }
        }
        self.words.len() == 0
    }
}
impl<A: WordArray> One for UnsignedInteger {
    #[inline]
    fn one() -> Self {
        let mut res = Self::ZER;
        res.set(1).unwrap();
        res
    }

    #[inline]
    fn is_one(&self) -> bool where
        Self: PartialEq, {
        self.words.len() == 1 && self.words[0] == 1
    }
}
impl<A: WordArray> UnsignedInteger<A> {
    /// Zero
    pub const ZERO: Self = UnsignedInteger { words: A::EMPTY };
    /// A slice of words
    #[inline]
    pub fn words(&self) -> &[Word] {
        self.words.as_ref()
    }
    /// A mutable slice of words
    #[inline]
    pub fn words_mut(&mut self) -> &mut [Word] {
        self.words.as_mut()
    }
    /// Set the integer equal to the specified (primitive) value
    #[inline]
    pub fn set(&mut self, val: u64) -> Result<(), A::AllocErr> {
        if val > 0 {
            self.words.reserve(1)?;
        }
        self.words.clear();
        if val > 0 {
            unsafe { self.words.push_unchecked(val) };
        }
        Ok(())
    }
    /// Create an integer from an array of words
    #[inline]
    pub fn from_word_array(words: A) -> Self {
        UnsignedInteger { words }
    }
    /// Get the underlying array of words
    #[inline]
    pub fn as_word_array(&self) -> &A {
        &self.words
    }
    /// Attempt to add the specified integer to this integer
    ///
    /// Errors if allocating space fails
    #[inline]
    pub fn add(&mut self, other: &Self) -> Result<(), A::AllocErr> {
        self.words.reserve(self.len().max(other.len())
            .checked_add(1)
            .ok_or_else(A::AllocErr::capacity_arithmetic_overflow)?)?;
        self.unchecked_add(other);
        Ok(())
    }
    /// Add the specified integer to this integer,
    /// without checking for overflow
    ///
    /// ## Safety
    /// Assumes `self.words.capacity >= max(self.len, other.len) + 1`
    pub fn unchecked_add(&mut self, other: &Self) {
        /*
         * Grade school addition algorithm:
         * For example,
         * 1 2 3
         *   4 5
         * ------
         * 1 6 7
         *
         * Just iterate over each corresponding index,
         * propagating carries as needed.
         */
        let mut carry = false;
        #[inline]
        unsafe fn ensure_iter<A: WordArray>(words: &mut A, target_index: usize) -> &mut Word {
            if target_index >= words.len() {
                /*
                 * Worst case, we should only need to append a single word
                 * to the array. That is because we also did the necessary
                 * bounds check last index as well. For example, if at the start of the call,
                 * `self.len() == 2, other.len() == 4`, when we are at iteration #4 of 'other',
                 * we can rely on the fact that iteration #3 would've already expanded `self.words` up
                 * to length three (so we would only need to add one more word, not two).
                 */
                debug_assert_eq!(
                    target_index + 1,
                    words.len()
                );
                unsafe {
                    words.unchecked_push(Word(0));
                }
            }
            debug_assert!(target_index < words.len());
            &mut words.get_unchecked_mut(target_index).0
        }
        for (addend_index, addend) in other.words().iter().enumerate() {
            let target_word = unsafe { ensure_iter(&mut self.words, addend_index) };
            let (addend, new_carry) = addend.0.overflowing_add(carry as u64);
            carry = new_carry;
            let (res, new_carry) = target_word.0.overflowing_add(addend);
            debug_assert!(!carry, "Double carry");
            carry = new_carry;
            target_word.0 = res;
        }
        {
            // Add final carry
            let target_index = other.words.len();
            let target_word = unsafe { ensure_iter(&mut self.words, target_index) };
            let (res, new_carry) = (*target_word).overflowing_add(carry as u64);
            target_word.0 = res;
            if new_carry {
                debug_assert!(self.words.capacity() >= target_index + 1);
                unsafe { self.words.unchecked_push(Word(1)) };
            }
        }
        debug_assert_ne!(self.words.last(), Some(Word(0)));
    }
    /// Add the specified [u64] to this integer
    #[inline]
    pub fn add_u64(&mut self, val: u64) -> Result<(), A::AllocErr> {
        self.words.reserve(1)?;
        unsafe { self.unchecked_add_u64(val) }
        Ok(())
    }
    /// Add the specified [u64] to this integer,
    /// without checking for the right capacity.
    ///
    /// ## Safety
    /// Assumes `self.words.capacity >= self.words.len + 1`,
    /// as if calling `self.words.reserve(1)`
    pub unsafe fn unchecked_add_u64(&mut self, mut val: u64) {
        if val == 0 { return }
        for target_word in self.words_mut().iter_mut() {
            let (res, carry) = target_word.0.overflowing_add(val);
            target_word.0 = res;
            if carry {
                val = 1;
                continue;
            } else {
                break;
            }
        }
        if val > 0 {
            unsafe {
                self.words.unchecked_push(Word(val));
            }
        }
        debug_assert_ne!(self.words.last(), Some(Word(0)));

    }
}
impl<A: WordArray> Num for UnsignedInteger<A> {
    type FromStrRadixErr = ParseIntError<A::AllocErr>;
    /// Parse a string in the specified radix (base)
    #[inline]
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, ParseIntError<A::AllocErr>> {
        crate::string::parse_unsigned_radix(str, radix)
    }
}
//! Conversions to/from strings
use thiserror::Error;
use crate::memory::{IAllocError, WordArray, Word};
use crate::uint::UnsignedInteger;
use crate::arith_utils::ArithUtil;

/// An error that occurs parsing a string
#[derive(Error, Debug)]
pub enum ParseIntError<A: IAllocError> {
    #[error("Invalid digit {digit} in base {radix}")]
    InvalidDigit {
        digit: char,
        radix: u32
    },
    #[error("Signs are forbidden in unsigned integers")]
    ForbiddenNegative,
    #[error("Empty string")]
    EmptyString,
    #[error("Allocation failed: {cause}")]
    AllocFailed {
        #[from]
        cause: A
    }
}

pub(crate) fn parse_unsigned_radix<A: WordArray>(mut s: &str, radix: u32) -> Result<UnsignedInteger<A>, ParseIntError<A::AllocErr>> {
    assert!((2..=36).contains(&radix), "Invalid radix: {}", radix);
    if s.starts_with("-") {
        return Err(ParseIntError::ForbiddenNegative);
    } else if s.starts_with("+") {
        s = &s[1..];
    }
    if s.is_empty() {
        return Err(ParseIntError::EmptyString)
    }
    /*
     * Allocation will probably be more expensive
     * than a little bit of this math.
     * For 'n' digits in base 10, there are 10**n possibilities,
     * which take up ceil(log2(10**n)) possibilities.
     * By log rules, this reduces too ceil(log2(10)*n),
     * or ceil(log2(10))*n.
     *
     */
    let ceil_log_radix = radix.ceil_log2();
    let max_capacity = match (ceil_log_radix as u64).checked_mul(s.len() as u64) {
        Some(max_bits) => {
            max_bits.divide_round_up(Word::BITS)
        },
        None => return Err(ParseIntError::AllocFailed {
            cause: A::AllocErr::capacity_arithmetic_overflow()
        })
    };
    let mut res = UnsignedInteger::from_word_array(A::with_capacity(max_capacity)?);
    res.set(1);
    for digit in s.chars() {
        let digit_val = match digit {
            '0'..='9' => digit as u8 - b'0',
            'A'..='Z' => digit as u8 - b'A',
            'a'..='z' => digit as u8 - b'a',
            _ => u8::MAX
        };
        if digit_val as u32 >= radix {
            return Err(ParseIntError::InvalidDigit {
                digit, radix
            })
        }
        res *= (digit_val as u32);
    }
    return Ok(res)
}
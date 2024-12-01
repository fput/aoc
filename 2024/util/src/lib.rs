use std::ops::{Index, IndexMut, Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Neg};
use std::str::{Bytes, FromStr};
use std::marker::PhantomData;


pub struct NumberExtractor<'a, T>
where
    T: FromStr + Default + AddAssign + MulAssign + From<u8>,
{
    bytes: Bytes<'a>,
    _marker: PhantomData<T>,
}

impl<'a, T> Iterator for NumberExtractor<'a, T>
where
    T: FromStr + Default + AddAssign + MulAssign + From<u8>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut num = T::default();
        let mut found_digit = false;

        while let Some(byte) = self.bytes.next() {
            let dig = byte.wrapping_sub(b'0');
            if dig < 10 {
                num *= T::from(10); // note: panics on overflow
                num += T::from(dig);
                found_digit = true;
            } else if found_digit {
                return Some(num);
            }
        }

        if found_digit {
            Some(num)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.bytes.size_hint();
        // Estimating that each number could on average be about 3-4 bytes long including separators
        (lower/4, upper.map(|u| u / 3))
    }
}

// Iterator that returns each number in a string one by one
pub fn extract_numbers<T>(input: &str) -> NumberExtractor<'_, T>
where
    T: FromStr + Default + AddAssign + MulAssign + From<u8>,
{
    NumberExtractor { bytes: input.bytes(), _marker: PhantomData }
}

/// A version that can extract signed numbers, allowing a '-' sign in front.
pub struct SignedNumberExtractor<'a, T>
where
    T: FromStr + Default + AddAssign + MulAssign + From<u8> + Neg<Output = T>,
{
    bytes: Bytes<'a>,
    _marker: PhantomData<T>,
}

impl<'a, T> Iterator for SignedNumberExtractor<'a, T>
where
    T: FromStr + Default + AddAssign + MulAssign + From<u8> + Neg<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut num = T::default();
        let mut found_digit = false;
        let mut is_negative = false;

        while let Some(byte) = self.bytes.next() {
            if byte == b'-' {
                if found_digit {
                    break;
                } else {
                    is_negative = true;
                }
            } else {
                let dig = byte.wrapping_sub(b'0');
                if dig < 10 {
                    num *= T::from(10);
                    num += T::from(dig);
                    found_digit = true;
                } else if found_digit {
                    // We've hit a non-digit, return the number
                    break;
                } else if is_negative {
                    is_negative = false;
                }
            }
        }

        if !found_digit {
            return None;
        }

        Some(if is_negative { -num } else { num })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.bytes.size_hint();
        // Rough estimation
        (lower / 4, upper.map(|u| u / 3))
    }
}

/// Iterator that returns each signed number in a string one by one.
/// This supports a leading '-' sign for negative numbers.
pub fn extract_numbers_signed<T>(input: &str) -> SignedNumberExtractor<'_, T>
where
    T: FromStr + Default + AddAssign + MulAssign + From<u8> + Neg<Output = T>,
{
    SignedNumberExtractor { bytes: input.bytes(), _marker: PhantomData }
}


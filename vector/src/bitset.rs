// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::ops::Index;

const BITS_PER_ELEM: usize = 8;
const TRUE: bool = true;
const FALSE: bool = false;

#[derive(Debug, Clone)]
pub struct BitSet {
    bits: Vec<u8>,
}

impl Default for BitSet {
    fn default() -> Self {
        Self::new()
    }
}

impl BitSet {
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self { bits: Vec::new() }
    }

    #[must_use]
    #[inline]
    pub fn with_len(len: usize) -> Self {
        let bits_len = len.div_ceil(BITS_PER_ELEM);
        Self {
            bits: vec![0; bits_len],
        }
    }

    #[must_use]
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            bits: bytes.to_vec(),
        }
    }

    #[must_use]
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.bits.as_ref()
    }

    #[must_use]
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.bits
    }

    fn expand(&mut self, index: usize) {
        let bits_len = (index + 1).div_ceil(BITS_PER_ELEM);
        if self.bits.len() < bits_len {
            // TODO(Shaohua): Adjust resize policy.
            self.bits.resize(bits_len, 0);
        }
    }

    pub fn set(&mut self, index: usize) {
        self.expand(index);
        let word = index / BITS_PER_ELEM;
        let bit = index % BITS_PER_ELEM;
        let flag = 1 << bit;
        self.bits[word] |= flag;
    }

    pub fn unset(&mut self, index: usize) {
        self.expand(index);
        let word = index / BITS_PER_ELEM;
        let bit = index % BITS_PER_ELEM;
        let flag = 1 << bit;
        self.bits[word] &= !flag;
    }

    pub fn flip(&mut self, index: usize) {
        self.expand(index);
        let word = index / BITS_PER_ELEM;
        let bit = index % BITS_PER_ELEM;
        let flag = 1 << bit;
        // FIXME(Shaohua):
        self.bits[word] &= !flag;
    }

    /// Check bit at `index` is set or not.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<bool> {
        let word = index / BITS_PER_ELEM;
        if word >= self.bits.len() {
            return None;
        }
        let bit = index % BITS_PER_ELEM;
        let flag = 1 << bit;
        Some((self.bits[word] & flag) == flag)
    }

    /// Returns the number of bits set to `true`.
    #[must_use]
    pub fn count_ones(&self) -> usize {
        self.bits
            .iter()
            .map(|byte| byte.count_ones() as usize)
            .sum()
    }

    /// Returns the number of bits set to `false`.
    #[must_use]
    pub fn count_zeros(&self) -> usize {
        self.bits
            .iter()
            .map(|byte| byte.count_zeros() as usize)
            .sum()
    }

    #[must_use]
    #[inline]
    pub const fn iter(&self) -> BitSetIter<'_> {
        BitSetIter {
            bit_set: self,
            index: 0,
        }
    }

    /// # Panics
    /// Raise panic if length of two bitset not equal.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        assert_eq!(self.bits.len(), other.bits.len());
        let bits = self
            .bits
            .iter()
            .zip(other.bits.iter())
            .map(|(a, b)| a | b)
            .collect();
        Self { bits }
    }

    /// # Panics
    /// Raise panic if length of two bitset not equal.
    #[must_use]
    pub fn intersect(&self, other: &Self) -> Self {
        assert_eq!(self.bits.len(), other.bits.len());
        let bits = self
            .bits
            .iter()
            .zip(other.bits.iter())
            .map(|(a, b)| a & b)
            .collect();
        Self { bits }
    }

    /// # Panics
    /// Raise panic if length of two bitset not equal.
    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        assert_eq!(self.bits.len(), other.bits.len());
        let bits = self
            .bits
            .iter()
            .zip(other.bits.iter())
            .map(|(a, b)| a & !b)
            .collect();
        Self { bits }
    }
}

impl From<String> for BitSet {
    fn from(value: String) -> Self {
        Self {
            bits: value.into_bytes(),
        }
    }
}

impl From<&str> for BitSet {
    fn from(s: &str) -> Self {
        Self::from_bytes(s.as_bytes())
    }
}

macro_rules! from_number_impl {
    ($($t:ty)*) => {$(
        impl From<$t> for BitSet {
            fn from(value: $t) -> Self {
                Self {
                    bits: value.to_le_bytes().to_vec(),
                }
            }
        }
    )*};
}

from_number_impl! {i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize}

impl Index<usize> for BitSet {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        if self.get(index).expect("index out of range") {
            &TRUE
        } else {
            &FALSE
        }
    }
}

pub struct BitSetIter<'a> {
    bit_set: &'a BitSet,
    index: usize,
}

impl Iterator for BitSetIter<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let is_set = self.bit_set.get(self.index);
        if is_set.is_some() {
            self.index += 1;
        }
        is_set
    }
}

impl<'a> IntoIterator for &'a BitSet {
    type IntoIter = BitSetIter<'a>;
    type Item = bool;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::bitset::BitSet;

    #[test]
    fn test_from_len() {
        let a = BitSet::with_len(9);
        assert_eq!(a.as_bytes(), &[0, 0]);
    }

    #[test]
    fn test_from_str() {
        let s = "hello, world";
        let bits = BitSet::from(s);
        assert_eq!(bits.as_bytes(), s.as_bytes());
    }

    #[test]
    fn test_from_u8() {
        let byte: u8 = 0b1001_0011;
        let bits = BitSet::from(byte);
        assert_eq!(bits.as_bytes(), &[byte]);
    }

    #[test]
    fn test_from_i8() {
        let byte: i8 = 0b0101_0011;
        let bits = BitSet::from(byte);
        assert_eq!(bits.as_bytes(), &[0b0101_0011]);
    }

    #[test]
    fn test_set() {
        let mut bits = BitSet::with_len(7);
        bits.set(5);
        assert_eq!(bits.as_bytes(), &[0b0010_0000]);
    }
}

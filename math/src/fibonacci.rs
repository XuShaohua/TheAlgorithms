// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! The Fibonacci sequence is a sequence in which each number is the sum of the two preceding ones.
//!
//! See [Fibonacci sequence](https://en.wikipedia.org/wiki/Fibonacci_sequence)

#![allow(clippy::module_name_repetitions)]

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FibError {
    IllegalIndex,
}

pub const MAX_INDEX: u8 = 47;

/// # Errors
/// Returns error if `num` is not a possitive integer.
pub fn fib(num: u8) -> Result<i64, FibError> {
    if num == 0 || num > MAX_INDEX {
        return Err(FibError::IllegalIndex);
    }
    if num == 1 {
        return Ok(0);
    }
    if num == 2 {
        return Ok(1);
    }
    let num = num as usize;
    let mut buf: Vec<i64> = vec![0; num + 1];
    buf[0] = 0;
    buf[1] = 0;
    buf[2] = 1;
    for i in 3..=num {
        buf[i] = buf[i - 1] + buf[i - 2];
    }
    Ok(buf[num])
}

#[cfg(test)]
mod tests {
    use super::{fib, FibError};

    #[test]
    fn test_fib() {
        assert_eq!(fib(5), Ok(3));
        assert_eq!(fib(2), Ok(1));
        assert_eq!(fib(9), Ok(21));
        assert_eq!(fib(47), Ok(1836311903));
        assert_eq!(fib(49), Err(FibError::IllegalIndex));
    }
}

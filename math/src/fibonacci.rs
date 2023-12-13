// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! The Fibonacci sequence is a sequence in which each number is the sum of the two preceding ones.
//!
//! See [Fibonacci sequence](https://en.wikipedia.org/wiki/Fibonacci_sequence)

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FibError {
    IllegalIndex,
}

pub const MAX_INDEX: u8 = 93;

/// # Errors
/// Returns error if `num` is not a possitive integer.
pub fn fib(num: u8) -> Result<u64, FibError> {
    let mut a = 0;
    let mut b = 1;
    match num {
        0 => Ok(0),
        num if num > MAX_INDEX => Err(FibError::IllegalIndex),
        _ => {
            for _ in 1..num {
                let c = a + b;
                a = b;
                b = c;
            }
            Ok(b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{fib, FibError};

    #[test]
    fn test_fib() {
        assert_eq!(fib(5), Ok(5));
        assert_eq!(fib(2), Ok(1));
        assert_eq!(fib(9), Ok(34));
        assert_eq!(fib(47), Ok(2971215073));
        assert_eq!(fib(94), Err(FibError::IllegalIndex));
    }
}

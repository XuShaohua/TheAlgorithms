// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Swap two numbers without temporary variable.
///
/// ```rust
/// use bitwise::swap_number::swap_number;
///
/// let mut a = 3;
/// let mut b = 42;
/// swap_number(&mut a, &mut b);
/// assert_eq!(a, 42);
/// assert_eq!(b, 3);
/// ```
pub fn swap_number(a: &mut i32, b: &mut i32) {
    *a ^= *b;
    *b ^= *a;
    *a ^= *b;
}

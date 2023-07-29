// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Calculate NOT of the input values
#[must_use]
pub fn not_gate(input_1: i32) -> i32 {
    i32::from(input_1 == 0)
}

#[cfg(test)]
mod tests {
    use super::not_gate;

    #[test]
    fn test_not_gate() {
        assert_eq!(not_gate(0), 1);
        assert_eq!(not_gate(1), 0);
    }
}

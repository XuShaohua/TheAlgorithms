// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Calculate OR of the input values
#[must_use]
pub fn or_gate(input_1: i32, input_2: i32) -> i32 {
    i32::from(input_1 != 0 || input_2 != 0)
}

#[cfg(test)]
mod tests {
    use super::or_gate;

    #[test]
    fn test_or_gate() {
        assert_eq!(or_gate(0, 0), 0);
        assert_eq!(or_gate(0, 1), 1);
        assert_eq!(or_gate(1, 0), 1);
        assert_eq!(or_gate(1, 1), 1);
    }
}
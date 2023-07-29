// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[must_use]
pub fn xnor_gate(input_1: i32, input_2: i32) -> i32 {
    i32::from(input_1 == input_2)
}

#[cfg(test)]
mod tests {
    use super::xnor_gate;

    #[test]
    fn test_xnor_gate() {
        assert_eq!(xnor_gate(0, 0), 1);
        assert_eq!(xnor_gate(0, 1), 0);
        assert_eq!(xnor_gate(1, 0), 0);
        assert_eq!(xnor_gate(1, 1), 1);
    }
}

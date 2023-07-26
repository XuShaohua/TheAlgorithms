// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Given a string `column_title` that represents the column title in an Excel sheet,
/// return its corresponding column number.
#[must_use]
#[allow(clippy::cast_lossless)]
#[allow(clippy::cast_possible_truncation)]
pub fn excel_title_to_column(column_title: &str) -> u32 {
    debug_assert!(!column_title.is_empty());
    debug_assert!(column_title.is_ascii());

    column_title
        .as_bytes()
        .iter()
        .rev()
        .enumerate()
        .map(|(index, c)| (c - 64) as u32 * 26_u32.pow(index as u32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::excel_title_to_column;

    #[test]
    fn test_excel_title_to_column() {
        const PAIRS: &[(&str, u32)] = &[("A", 1), ("B", 2), ("AB", 28), ("Z", 26)];
        for (key, val) in PAIRS {
            assert_eq!(excel_title_to_column(key), *val);
        }
    }
}

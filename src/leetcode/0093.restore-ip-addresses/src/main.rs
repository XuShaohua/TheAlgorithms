// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Bruteforce
pub fn restore_ip_addresses1(s: String) -> Vec<String> {
    fn is_valid_ip_sector(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        if s.len() > 3 {
            return false;
        }

        // Leading zero
        if s.len() > 1 && s.starts_with('0') {
            return false;
        }
        if let Ok(val) = s.parse::<i32>() {
            (0..=255).contains(&val)
        } else {
            false
        }
    }

    // 保存结果
    let mut out: Vec<String> = Vec::new();
    let max_i: usize = 4.min(s.len());
    for i in 1..max_i {
        let first = &s[0..i];
        if !is_valid_ip_sector(first) {
            continue;
        }

        let max_j: usize = (i + 4).min(s.len());
        for j in (i + 1)..max_j {
            let second = &s[i..j];
            if !is_valid_ip_sector(second) {
                continue;
            }

            let max_k: usize = (j + 4).min(s.len());
            for k in (j + 1)..max_k {
                let third = &s[j..k];
                let forth = &s[k..];
                if is_valid_ip_sector(third) && is_valid_ip_sector(forth) {
                    out.push([first, second, third, forth].join("."));
                }
            }
        }
    }

    out
}

// Backtracking

pub type SolutionFn = fn(String) -> Vec<String>;

fn check_solution(func: SolutionFn) {
    let s = "25525511135".to_owned();
    let expected = vec!["255.255.11.135".to_owned(), "255.255.111.35".to_owned()];
    assert_eq!(func(s), expected);

    let s = "0000".to_owned();
    let expected = vec!["0.0.0.0".to_owned()];
    assert_eq!(func(s), expected);

    let s = "101023".to_owned();
    let expected = vec![
        "1.0.10.23".to_owned(),
        "1.0.102.3".to_owned(),
        "10.1.0.23".to_owned(),
        "10.10.2.3".to_owned(),
        "101.0.2.3".to_owned(),
    ];
    assert_eq!(func(s), expected);
}

fn main() {
    check_solution(restore_ip_addresses1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, restore_ip_addresses1};

    #[test]
    fn test_restore_ip_addresses1() {
        check_solution(restore_ip_addresses1);
    }
}

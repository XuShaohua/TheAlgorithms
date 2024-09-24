// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

fn is_ipv4(query: &str) -> bool {
    // 用 `.` 来分隔各部分
    // 并判断每个部分是有效的数值
    // 数值不带有前缀0

    let parts: Vec<&str> = query.split('.').collect();
    if parts.len() != 4 {
        return false;
    }

    for part in parts {
        if part.len() < 1 || part.len() > 3 {
            return false;
        }

        // 数值不带有前缀0
        if part.len() > 1 && part.starts_with("0") {
            return false;
        }

        // 判断字符的范围, 0-9
        for c in part.chars() {
            if !c.is_ascii_digit() {
                return false;
            }
        }

        if let Ok(val) = part.parse::<i32>() {
            // 数值范围是 0..255
            if val < 0 || val > 255 {
                return false;
            }
        } else {
            // 不是有效的整数
            return false;
        }
    }

    true
}

fn is_ipv6(query: &str) -> bool {
    // 使用 `:` 作为分隔符
    // 每个部分是16进制的整数, 16进制支持大小写, 最多包含4个字符
    // 可以有0作为前缀
    // 不需要考虑缩写

    let parts: Vec<&str> = query.split(':').collect();
    if parts.len() != 8 {
        return false;
    }

    for part in parts {
        // 1-4个字符
        if part.len() < 1 || part.len() > 4 {
            return false;
        }

        for c in part.chars() {
            // 判断字符的范围, 0-9, a-f, A-F
            if !c.is_ascii_hexdigit() {
                return false;
            }
        }
    }

    true
}

pub fn valid_ip_address1(query_ip: String) -> String {
    if is_ipv4(&query_ip) {
        "IPv4".to_owned()
    } else if is_ipv6(&query_ip) {
        "IPv6".to_owned()
    } else {
        "Neither".to_owned()
    }
}

pub type SolutionFn = fn(String) -> String;

fn check_solution(func: SolutionFn) {
    {
        let s1 = "2001:0db8:85a3:0:0:8A2E:0370:7334:".to_owned();
        let expected = "Neither";
        let out = func(s1);
        assert_eq!(out, expected);
    }

    {
        let s1 = "1e1.4.5.6".to_owned();
        let expected = "Neither";
        let out = func(s1);
        assert_eq!(out, expected);
    }
}

fn main() {
    check_solution(valid_ip_address1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, valid_ip_address1};

    #[test]
    fn test_valid_ip_address1() {
        check_solution(valid_ip_address1);
    }
}

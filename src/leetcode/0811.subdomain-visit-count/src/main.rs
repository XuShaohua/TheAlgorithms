// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

// HashTable
// 字典计数
// 字符串分隔
pub fn subdomain_visits1(cpdomains: Vec<String>) -> Vec<String> {
    assert!(!cpdomains.is_empty());
    // 计数用的字典 (domain, count)
    let mut map: HashMap<String, usize> = HashMap::new();

    for cpdomain in &cpdomains {
        if let Some(pos) = cpdomain.find(' ') {
            let count: usize = cpdomain[..pos].parse().unwrap();
            let domain_name: &str = &cpdomain[pos + 1..];
            // 先把当前的域名加到字典里.
            *map.entry(domain_name.to_string()).or_default() += count;

            // 拆分 domain name, 其实就是以 '.' 分隔的字符串.
            for (index, _) in domain_name.match_indices('.') {
                let part_name = &domain_name[(index + 1)..];
                // 再把父域名加到字典里.
                *map.entry(part_name.to_string()).or_default() += count;
            }
        }
    }

    // 最后, 重新组装.
    map.into_iter()
        .map(|(name, count)| format!("{count} {name}"))
        .collect()
}

// HashTable
// 优化字符串处理过程
pub fn subdomain_visits2(cpdomains: Vec<String>) -> Vec<String> {
    assert!(!cpdomains.is_empty());

    // 计数用的字典 (domain, count)
    let mut map: HashMap<String, usize> = HashMap::new();

    for cpdomain in &cpdomains {
        let (count_str, domain_name) = cpdomain.split_once(' ').unwrap();
        let count: usize = count_str.parse().unwrap();
        // 先把当前的域名加到字典里.
        *map.entry(domain_name.to_string()).or_default() += count;

        // 拆分 domain name, 其实就是以 '.' 分隔的字符串.
        for (index, _) in domain_name.match_indices('.') {
            let part_name = &domain_name[(index + 1)..];
            // 再把父域名加到字典里.
            *map.entry(part_name.to_string()).or_default() += count;
        }
    }

    // 最后, 重新组装.
    map.into_iter()
        .map(|(name, count)| format!("{count} {name}"))
        .collect()
}

pub type SolutionFn = fn(Vec<String>) -> Vec<String>;

fn check_solution(func: SolutionFn) {
    fn to_string_vec(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }

    fn check_vec_equal(mut v1: Vec<String>, mut v2: Vec<String>) {
        v1.sort_unstable();
        v2.sort_unstable();
        assert_eq!(v1, v2)
    }

    let cpdomains = &["9001 discuss.leetcode.com"];
    let exp = &["9001 leetcode.com", "9001 discuss.leetcode.com", "9001 com"];
    check_vec_equal(func(to_string_vec(cpdomains)), to_string_vec(exp));

    let cpdomains = &[
        "900 google.mail.com",
        "50 yahoo.com",
        "1 intel.mail.com",
        "5 wiki.org",
    ];
    let exp = &[
        "901 mail.com",
        "50 yahoo.com",
        "900 google.mail.com",
        "5 wiki.org",
        "5 org",
        "1 intel.mail.com",
        "951 com",
    ];
    check_vec_equal(func(to_string_vec(cpdomains)), to_string_vec(exp));
}

fn main() {
    check_solution(subdomain_visits1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, subdomain_visits1, subdomain_visits2};

    #[test]
    fn test_subdomain_visits1() {
        check_solution(subdomain_visits1);
    }

    #[test]
    fn test_subdomain_visits2() {
        check_solution(subdomain_visits2);
    }
}

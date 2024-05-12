// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

// HashTable
// 将每个字符串按升序排序, 作为它的索引.
pub fn group_anagrams1(strs: Vec<String>) -> Vec<Vec<String>> {
    debug_assert!(!strs.is_empty());

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for s in strs.into_iter() {
        // 构造字符串的索引
        let mut chars_vec: Vec<char> = s.chars().collect();
        chars_vec.sort_unstable();
        let index: String = chars_vec.into_iter().collect();
        // 将 (字符串索引, 字符串) 插入到表哈稀表中.
        map.entry(index).or_default().push(s);
    }

    //map.into_values().collect()
    //map.into_iter().map(|(_index, strs)| strs).collect()

    // 为了测试, 我们对结果进行一下排序.
    let mut out: Vec<Vec<String>> = map
        .into_values()
        .map(|mut strs| {
            strs.sort_unstable();
            strs
        })
        .collect();
    out.sort_by_key(|v| v.len());
    out
}

pub type SolutionFn = fn(Vec<String>) -> Vec<Vec<String>>;

fn check_solution(func: SolutionFn) {
    let strs = vec![
        "eat".to_owned(),
        "tea".to_owned(),
        "tan".to_owned(),
        "ate".to_owned(),
        "nat".to_owned(),
        "bat".to_owned(),
    ];
    let exp = vec![
        vec!["bat".to_owned()],
        vec!["nat".to_owned(), "tan".to_owned()],
        vec!["ate".to_owned(), "eat".to_owned(), "tea".to_owned()],
    ];
    assert_eq!(func(strs), exp);

    let strs = vec!["".to_owned()];
    let exp = vec![vec!["".to_owned()]];
    assert_eq!(func(strs), exp);

    let strs = vec!["a".to_owned()];
    let exp = vec![vec!["a".to_owned()]];
    assert_eq!(func(strs), exp);
}

fn main() {
    check_solution(group_anagrams1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, group_anagrams1};

    #[test]
    fn test_group_anagrams1() {
        check_solution(group_anagrams1);
    }
}

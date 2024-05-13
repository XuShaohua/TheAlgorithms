// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;
use std::collections::HashMap;

// HashTable
pub fn find_restaurant1(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    debug_assert!(!list1.is_empty());
    debug_assert!(!list2.is_empty());

    // 用于存储 list1 中的 (单词, 索引)
    let map1: HashMap<String, usize> = list1
        .into_iter()
        .enumerate()
        .map(|(index, s)| (s, index))
        .collect();

    let mut least_index_sum: usize = usize::MAX;
    let mut common_strings = Vec::new();

    // 然后遍历 list2
    for (index2, s2) in list2.into_iter().enumerate() {
        if let Some(index1) = map1.get(&s2) {
            let sum = index1 + index2;
            match sum.cmp(&least_index_sum) {
                Ordering::Less => {
                    common_strings.clear();
                    common_strings.push(s2);
                    least_index_sum = sum;
                }
                Ordering::Equal => common_strings.push(s2),
                Ordering::Greater => (),
            }
        }
    }

    common_strings
}

pub type SolutionFn = fn(Vec<String>, Vec<String>) -> Vec<String>;

fn check_solution(func: SolutionFn) {
    fn to_string_vec(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }

    let list1 = &["Shogun", "Tapioca Express", "Burger King", "KFC"];
    let list2 = &[
        "Piatti",
        "The Grill at Torrey Pines",
        "Hungry Hunter Steakhouse",
        "Shogun",
    ];
    let exp = &["Shogun"];
    assert_eq!(
        func(to_string_vec(list1), to_string_vec(list2)),
        to_string_vec(exp)
    );

    let list1 = &["Shogun", "Tapioca Express", "Burger King", "KFC"];
    let list2 = &["KFC", "Shogun", "Burger King"];
    let exp = &["Shogun"];
    assert_eq!(
        func(to_string_vec(list1), to_string_vec(list2)),
        to_string_vec(exp)
    );

    let list1 = &["happy", "sad", "good"];
    let list2 = &["sad", "happy", "good"];
    let exp = &["sad", "happy"];
    assert_eq!(
        func(to_string_vec(list1), to_string_vec(list2)),
        to_string_vec(exp)
    );
}

fn main() {
    check_solution(find_restaurant1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, find_restaurant1};

    #[test]
    fn test_find_restaurant1() {
        check_solution(find_restaurant1);
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 并行双指针
pub fn is_long_pressed_name1(name: String, typed: String) -> bool {
    assert!(!name.is_empty() && !typed.is_empty());
    let name = name.as_bytes();
    let typed = typed.as_bytes();
    let len1 = name.len();
    let len2 = typed.len();

    // 初始化两个指针
    let mut index1 = 0;
    let mut index2 = 0;

    // 使用双指针遍历两个数组.
    while index1 < len1 && index2 < len2 {
        if name[index1] == typed[index2] {
            // 相等时, 同时向前移动两个指针.
            index1 += 1;
            index2 += 1;
        } else if index1 > 0 && name[index1 - 1] == typed[index2] {
            // 长按了一下
            index2 += 1;
        } else {
            // 按错了
            return false;
        }
    }

    // 如果已经遍历完了 name 数组, 还没遍历完 typed 数组,
    // 接下来遍历 typed 里剩下的元素.
    // 遍历的唯一条件就是 typed 里面剩下的元素都和 name 数组最后一个元素相同,
    // 否则就是按错了.
    while index2 < len2 && name[len1 - 1] == typed[index2] {
        index2 += 1;
    }

    // 检查两个数组是否都遍历完, 如果没有遍历完:
    // - index1 < len1, 少按了
    // - index2 < len2, 按错了
    index1 == len1 && index2 == len2
}

pub type SolutionFn = fn(String, String) -> bool;

fn check_solution(func: SolutionFn) {
    assert!(func("alex".to_owned(), "aaleex".to_owned()));
    assert!(!func("saeed".to_owned(), "ssaaedd".to_owned()));
}

fn main() {
    check_solution(is_long_pressed_name1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, is_long_pressed_name1};

    #[test]
    fn test_is_long_pressed_name1() {
        check_solution(is_long_pressed_name1);
    }
}

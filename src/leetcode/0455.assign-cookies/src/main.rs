// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Greedy
pub fn find_content_children1(g: Vec<i32>, s: Vec<i32>) -> i32 {
    // 先对两个数组进行排序, 方便根据孩子的胃口选择相应的饼干.
    let mut appetite = g;
    appetite.sort_unstable();
    let mut sizes = s;
    sizes.sort_unstable();

    // 两个指针, 分别指向当前要检查的胃口和饼干大小.
    let mut app_index: usize = 0;
    let mut size_index: usize = 0;
    let mut count: i32 = 0;
    while app_index < appetite.len() && size_index < sizes.len() {
        // 当前这块饼干给这个孩子吃.
        if appetite[app_index] <= sizes[size_index] {
            count += 1;
            app_index += 1;
            size_index += 1;
        } else {
            // 忽略当前这块饼干, 去尝试下一块.
            size_index += 1;
        }
    }
    count
}

pub type SolutionFn = fn(Vec<i32>, Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let g = vec![1, 2, 3];
    let s = vec![1, 1];
    assert_eq!(func(g, s), 1);

    let g = vec![1, 2];
    let s = vec![1, 2, 3];
    assert_eq!(func(g, s), 2);
}

fn main() {
    check_solution(find_content_children1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, find_content_children1};

    #[test]
    fn test_find_content_children1() {
        check_solution(find_content_children1);
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Quick sort
pub fn maximum_gap1(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return 0;
    }

    // 先用自带的快速排序方法.
    let mut nums = nums;
    nums.sort_unstable();

    let mut max_gap = 0;
    for i in 1..nums.len() {
        max_gap = max_gap.max(nums[i] - nums[i - 1]);
    }
    max_gap
}

// Radix sort
// 时间复杂度 O(n)
// 空间复杂度 O(n)
pub fn maximum_gap2(nums: Vec<i32>) -> i32 {
    fn radix_sort(nums: &mut Vec<i32>) {
        let max_num: i32 = nums.iter().copied().max().unwrap_or_default();
        let max_digits: usize = max_num.to_string().len();

        // 定义一个长度为10的数组, 每个桶分别代表 0-9 中的一个数
        let mut buckets = vec![vec![]; 10];

        for i in 0..max_digits {
            // 遍历数组, 将当前位的数按顺序放到桶里面.
            for &num in nums.iter() {
                let digit = (num as usize) / 10_usize.pow(i as u32) % 10;
                buckets[digit].push(num);
            }

            // 清空数组, 并将 buckets 中排序好的整数填入.
            nums.clear();
            for bucket in buckets.iter() {
                nums.extend_from_slice(bucket);
            }

            // 最后清空 buckets, 供下次使用.
            for bucket in buckets.iter_mut() {
                bucket.clear();
            }
        }
    }

    if nums.len() < 2 {
        return 0;
    }

    let mut nums = nums;
    radix_sort(&mut nums);

    let mut max_gap = 0;
    for i in 1..nums.len() {
        max_gap = max_gap.max(nums[i] - nums[i - 1]);
    }
    max_gap
}

// Bucket sort
// 时间复杂度: O(n)
// 空间复杂度: O(n)
pub fn maximum_gap3(nums: Vec<i32>) -> i32 {
    fn insertion_sort(nums: &mut [i32]) {
        // 遍历数组
        for i in 1..nums.len() {
            // 从右到左遍历前半部分有序数组
            for j in (1..=i).rev() {
                if nums[j - 1] > nums[j] {
                    nums.swap(j - 1, j);
                } else {
                    break;
                }
            }
        }
    }

    if nums.len() < 2 {
        return 0;
    }

    // 确定最大整数和最小整数
    let mut max_num: i32 = nums[0];
    let mut min_num: i32 = nums[0];
    for &num in nums.iter() {
        max_num = max_num.max(num);
        min_num = min_num.min(num);
    }

    // 处理特殊情况, 所有整数均相等.
    if max_num == min_num {
        return 0;
    }

    // 确定桶的个数
    let gap: usize = 10;
    let bucket_size: usize = (max_num - min_num) as usize / gap + 1;
    let mut buckets = vec![vec![]; bucket_size];

    // 将整数放到桶中
    for &num in nums.iter() {
        let index: usize = (num - min_num) as usize / gap;
        buckets[index].push(num);
    }

    // 对每个桶单独排序
    let mut max_gap: i32 = i32::MIN;
    for mut bucket in buckets {
        insertion_sort(&mut bucket);
        for i in 1..bucket.len() {
            max_gap = max_gap.max(bucket[i] - bucket[i - 1]);
        }
    }

    max_gap
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![3, 6, 9, 1];
    assert_eq!(func(nums), 3);

    let nums = vec![10];
    assert_eq!(func(nums), 0);

    let nums = vec![1, 10000000];
    assert_eq!(func(nums), 9999999);
}

fn main() {
    check_solution(maximum_gap1);
    check_solution(maximum_gap2);
    check_solution(maximum_gap3);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, maximum_gap1, maximum_gap2};

    #[test]
    fn test_maximum_gap1() {
        check_solution(maximum_gap1);
    }

    #[test]
    fn test_maximum_gap2() {
        check_solution(maximum_gap2);
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[allow(clippy::cast_possible_truncation)]
pub fn radix_sort(nums: &mut [u32]) {
    const fn num_digits(mut num: u32) -> usize {
        let mut count: usize = 0;
        while num != 0 {
            count += 1;
            num /= 10;
        }
        count
    }

    if nums.is_empty() {
        return;
    }
    // 获取最大的位数
    let max_digits: usize = nums
        .iter()
        .map(|num| num_digits(*num))
        .max()
        .unwrap_or_default();

    for i in 0..max_digits {
        // bucket 长度为10, 代表了数字 0~9.
        let mut buckets = vec![vec![]; 10];

        for num in nums.iter() {
            // 这个 index 是关键, 它是每个元素在当前位上的数字
            let index: u32 = *num / 10_u32.pow(i as u32) % 10;
            buckets[index as usize].push(*num);
        }

        let mut index = 0;
        for bucket in buckets {
            for num in bucket {
                // 取出对应的元素, 更新到原始数组中
                nums[index] = num;
                index += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::radix_sort;

    #[test]
    fn test_radix_sort() {
        let mut list = [0, 5, 3, 2, 2];
        radix_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [45, 5, 2];
        radix_sort(&mut list);
        assert_eq!(list, [2, 5, 45]);
    }
}

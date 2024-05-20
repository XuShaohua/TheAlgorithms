// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn bucket_sort(nums: &mut [i32]) {
    let bucket_size = nums.len().ilog2() as usize;
    bucket_sort_with_bucket(nums, bucket_size);
}

fn insertion_sort(nums: &mut [i32]) {
    let len = nums.len();

    for i in 1..len {
        for j in (1..=i).rev() {
            if nums[j] < nums[j - 1] {
                nums.swap(j, j - 1);
            }
        }
    }
}

#[allow(clippy::cast_sign_loss)]
fn bucket_sort_with_bucket(nums: &mut [i32], bucket_size: usize) {
    let min_num: i32 = nums.iter().min().copied().unwrap_or_default();
    let max_num: i32 = nums.iter().max().copied().unwrap_or_default();
    let bucket_count: usize = ((max_num - min_num) as usize) / bucket_size + 1;
    let mut buckets: Vec<Vec<i32>> = vec![vec![]; bucket_count];

    // 遍历数组, 将元素分配到每个桶中
    for &num in nums.iter() {
        let bucket_index = ((num - min_num) as usize) / bucket_size;
        buckets[bucket_index].push(num);
    }

    // 对每一个桶单独排序
    let mut index: usize = 0;
    for mut bucket in buckets {
        insertion_sort(&mut bucket);
        nums[index..(index + bucket.len())].copy_from_slice(&bucket);
        index += bucket.len();
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn bucket_sort(arr: &mut [i32]) {
    if !arr.is_empty() {
        let bucket_size = arr.len().ilog2() as usize;
        bucket_sort_with_bucket(arr, bucket_size);
    }
}

fn insertion_sort(arr: &mut [i32]) {
    let len = arr.len();

    for i in 1..len {
        for j in (1..=i).rev() {
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
            }
        }
    }
}

#[allow(clippy::cast_sign_loss)]
fn bucket_sort_with_bucket(arr: &mut [i32], bucket_size: usize) {
    let min_num: i32 = arr.iter().min().copied().unwrap();
    let max_num: i32 = arr.iter().max().copied().unwrap();
    let range: i32 = max_num - min_num;
    let bucket_count: usize = range as usize / bucket_size + 1;
    let mut buckets: Vec<Vec<i32>> = vec![vec![]; bucket_count];

    // 遍历数组, 将元素分配到每个桶中
    for &num in arr.iter() {
        let range: i32 = num - min_num;
        let bucket_index: usize = range as usize / bucket_size;
        buckets[bucket_index].push(num);
    }

    // 对每一个桶单独排序
    let mut index: usize = 0;
    for mut bucket in buckets {
        insertion_sort(&mut bucket);
        arr[index..(index + bucket.len())].copy_from_slice(&bucket);
        index += bucket.len();
    }
}

#[cfg(test)]
mod tests {
    use super::bucket_sort;

    #[test]
    fn test_bucket_sort() {
        let mut list = [0, 5, 3, 2, 2];
        bucket_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        bucket_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        bucket_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );
    }
}

// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    assert!(!nums.is_empty());

    let len = nums.len();
    if len == 1 {
        return 1;
    }

    let first_elem = nums[0];

    let mut i = 0;
    while i < len - 1 {
        let mut duplicates = 0;
        for j in (i + 1)..len {
            if nums[i] == nums[j] {
                nums[j] = first_elem;
                duplicates += 1;
            } else {
                break;
            }
        }
        i += duplicates + 1;
    }

    i = 1;
    while i < nums.len() {
        if nums[i] == first_elem {
            nums.remove(i);
        } else {
            i += 1;
        }
    }
    nums.len() as i32
}

fn main() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let expected_nums = &[0, 1, 2, 3, 4];
    let k = remove_duplicates(&mut nums);
    assert_eq!(&nums, expected_nums);
    assert_eq!(k, 5);

    let mut nums = vec![1, 1, 2];
    let expected_nums = &[1, 2];
    let k = remove_duplicates(&mut nums);
    assert_eq!(&nums, expected_nums);
    assert_eq!(k, 2);
}

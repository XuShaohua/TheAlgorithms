// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub fn get_all<T: Copy>(out: &mut Vec<Vec<T>>, data: &[T], chunk_len: usize) {
    if chunk_len == 0 {
        out.push(Vec::new());
        return;
    }
    if chunk_len == data.len() {
        out.push(data.to_vec());
        return;
    }

    let len = data.len();
    let min = 2_usize.pow(chunk_len as u32) - 1;
    let mut mask = 2_usize.pow(len as u32) - 2_usize.pow((len - chunk_len) as u32);

    fn get_chunk<T: Copy>(mask: usize, data: &[T]) -> Vec<T> {
        let b = format!("{:01$b}", mask, data.len());
        b.chars()
            .enumerate()
            .filter(|&(_, e)| e == '1')
            .map(|(i, _)| data[i])
            .collect()
    }

    while mask >= min {
        if mask.count_ones() as usize == chunk_len {
            let res = get_chunk(mask, data);
            mask -= 1;
            out.push(res);
        } else {
            mask -= 1;
        }
    }
}

// Combinations
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut out = vec![];
    for i in 0..=nums.len() {
        get_all(&mut out, &nums, i);
    }
    out
}

fn main() {
    let nums = vec![1, 2, 3];
    let expected_out = vec![
        vec![],
        vec![1],
        vec![2],
        vec![3],
        vec![1, 2],
        vec![1, 3],
        vec![2, 3],
        vec![1, 2, 3],
    ];
    assert_eq!(subsets(nums), expected_out);

    let nums = vec![0];
    let expected_out = vec![vec![], vec![0]];
    assert_eq!(subsets(nums), expected_out);
}

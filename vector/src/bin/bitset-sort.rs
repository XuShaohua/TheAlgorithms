// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs::File;
use std::io::{self, Read};

use vector::bitset::BitSet;

pub fn random_ints(len: usize) -> Result<Vec<u32>, io::Error> {
    let mut file = File::open("/dev/urandom")?;
    let mut buf = vec![0; len * 4];
    file.read_exact(&mut buf)?;

    let mut nums = vec![0; len];
    for i in 0..len {
        let array: [u8; 4] = [buf[4 * i], buf[4 * i + 1], buf[4 * i + 2], buf[4 * i + 3]];
        let num = u32::from_le_bytes(array);
        nums[i] = num.min(10_000_000);
    }
    Ok(nums)
}

fn main() {
    let mut numbers = random_ints(10_000_000).unwrap();
    let max_number = numbers.iter().max().copied().unwrap_or_default() as usize;
    let mut bit_set = BitSet::with_len(max_number);
    for &num in &numbers {
        bit_set.set(num as usize);
    }

    let sorted: Vec<u32> = bit_set
        .iter()
        .enumerate()
        .filter(|(_index, is_set)| *is_set)
        .map(|(index, _is_set)| index as u32)
        .collect();

    numbers.sort_unstable();
    numbers.dedup();
    assert_eq!(numbers, sorted);
}

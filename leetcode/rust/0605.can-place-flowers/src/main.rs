// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    if n <= 0 {
        return true;
    }

    let mut flowerbed = flowerbed;
    let mut n = n;
    flowerbed.insert(0, 0);
    flowerbed.push(0);
    let mut i = 1;
    while i < flowerbed.len() - 1 {
        if flowerbed[i] == 1 {
            i += 1;
        } else if flowerbed[i - 1] == 0 && flowerbed[i + 1] == 0 {
            flowerbed[i] = 1;
            n -= 1;
            if n == 0 {
                break;
            }
        }

        i += 1;
    }

    n == 0
}

fn check_solution() {
    let flowerbed = vec![1, 0, 0, 0, 1];
    assert!(can_place_flowers(flowerbed, 1));

    let flowerbed = vec![1, 0, 0, 0, 1];
    assert!(!can_place_flowers(flowerbed, 2));

    let flowerbed = vec![1, 0, 0, 0, 0, 1];
    assert!(!can_place_flowers(flowerbed, 2));

    let flowerbed = vec![0, 1, 0, 1, 0, 1, 0, 0];
    assert!(can_place_flowers(flowerbed, 1));

    let flowerbed = vec![1, 0, 0, 1];
    assert!(!can_place_flowers(flowerbed, 1));

    let flowerbed = vec![0, 0, 1, 0, 0];
    assert!(can_place_flowers(flowerbed, 1));
}

fn main() {
    check_solution();
}

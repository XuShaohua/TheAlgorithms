// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
    let mut full_bottles = num_bottles;
    let mut count = 0;
    let mut empty_bottles = 0;

    // 当还有满瓶的水, 或者空瓶子的数量大于最小兑换数时, 游戏就可以进行
    while full_bottles > 0 || empty_bottles >= num_exchange {
        // 喝水
        count += full_bottles;
        // 收集空的瓶子
        empty_bottles += full_bottles;
        // 把空瓶子换成满瓶的水
        full_bottles = empty_bottles / num_exchange;
        // 可能还会剩下一些空瓶子在本次无法兑换
        empty_bottles %= num_exchange;
    }
    count
}

pub type SolutionFn = fn(i32, i32) -> i32;

fn check_solution(func: SolutionFn) {
    struct Record {
        num_bottles: i32,
        num_exchange: i32,
        total: i32,
    }

    impl Record {
        #[must_use]
        #[inline]
        const fn new(num_bottles: i32, num_exchange: i32, total: i32) -> Self {
            Self {
                num_bottles,
                num_exchange,
                total,
            }
        }
    }

    const RECORDS: &[Record] = &[Record::new(9, 3, 13), Record::new(15, 4, 19)];
    for record in RECORDS {
        assert_eq!(func(record.num_bottles, record.num_exchange), record.total);
    }
}

fn main() {
    check_solution(num_water_bottles);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, num_water_bottles};

    #[test]
    fn test_num_water_bottles() {
        check_solution(num_water_bottles);
    }
}

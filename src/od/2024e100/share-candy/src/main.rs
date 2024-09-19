// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;
use std::io::{stdin, BufRead};

fn main() {
    // 缓存 + 递归
    fn get_minimum_times(num: usize, cache: &mut HashMap<usize, usize>) -> usize {
        if let Some(value) = cache.get(&num) {
            return *value;
        }
        if num % 2 == 0 {
            // 如果是偶数个
            // 平均分一次
            let times = 1 + get_minimum_times(num / 2, cache);
            cache.insert(num, times);
            times
        } else {
            // 如果是奇数个, 有两种方式:
            // 取一个
            let times1 = 1 + get_minimum_times(num + 1, cache);
            // 放一个
            let times2 = 1 + get_minimum_times(num - 1, cache);

            // 求它们的最小值
            let min_times = times1.min(times2);
            cache.insert(num, min_times);
            min_times
        }
    }

    let mut cache = HashMap::from([(1, 0)]);
    let mut line = String::new();
    let ret = stdin().lock().read_line(&mut line);
    assert!(ret.is_ok());
    let num_candies: usize = line.trim().parse().unwrap();

    let times = get_minimum_times(num_candies, &mut cache);
    println!("{times}");
}

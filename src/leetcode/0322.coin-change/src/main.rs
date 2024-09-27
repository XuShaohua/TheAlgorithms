// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

// 动态规划 DP
pub fn coin_change1(coins: Vec<i32>, amount: i32) -> i32 {
    fn dp(cache: &mut HashMap<i32, i32>, coins: &[i32], n: i32) -> i32 {
        // 基本常量
        const INITIAL_COUNT: i32 = i32::MAX;
        const INVALID_COUNT: i32 = -1;
        // 初始条件
        if n == 0 {
            // 恰好兑换完
            return 0;
        }
        if n < 0 {
            // 说明没有合适的币值去完成兑换
            return INVALID_COUNT;
        }
        if let Some(&count) = cache.get(&n) {
            return count;
        }

        // 初始的最小兑换次数要足够大
        let mut min_exchange_count = INITIAL_COUNT;

        // 依次遍历每一种币值, 并使用其中的币值换一次
        // 找出最小的兑换次数
        for &coin in coins {
            // 递归调用, 用当前的币值兑换一次
            let mut exchange_count = dp(cache, coins, n - coin);
            cache.insert(n - coin, exchange_count);
            if exchange_count != INVALID_COUNT {
                // 如果是有效兑换, 真正的兑换次数要算上当前这个币.
                exchange_count += 1;
                cache.insert(n, exchange_count);
                min_exchange_count = min_exchange_count.min(exchange_count);
            }
        }

        // 最后返回最小值
        if min_exchange_count == INITIAL_COUNT {
            INVALID_COUNT
        } else {
            min_exchange_count
        }
    }

    let mut cache = HashMap::new();

    // 调用 dp(), 递归获得最小次数
    dp(&mut cache, &coins, amount)
}

// 动态规划 DP
// 取消递归
pub fn coin_change2(coins: Vec<i32>, amount: i32) -> i32 {
    const INVALID_COUNT: i32 = i32::MAX;
    assert!(amount >= 0);
    let amount = amount as usize;
    // 数组用于缓存, 记录每个数值可以用到的最小兑换次数.
    let mut dp = vec![INVALID_COUNT; amount + 1];
    dp[0] = 0;

    for &coin in &coins {
        let coin = coin as usize;
        for remains in coin..=amount {
            if dp[remains - coin] != INVALID_COUNT {
                // 是否使用这个币值进行兑换
                dp[remains] = dp[remains].min(dp[remains - coin] + 1);
            }
        }
    }

    if dp[amount] == INVALID_COUNT {
        -1
    } else {
        dp[amount]
    }
}

pub type SolutionFn = fn(Vec<i32>, i32) -> i32;

fn check_solution(func: SolutionFn) {
    let coins = vec![1, 2, 5];
    let amount = 11;
    assert_eq!(func(coins, amount), 3);

    let coins = vec![1, 2, 5];
    let amount = 100;
    assert_eq!(func(coins, amount), 20);

    let coins = vec![2];
    let amount = 3;
    assert_eq!(func(coins, amount), -1);

    let coins = vec![1];
    let amount = 0;
    assert_eq!(func(coins, amount), 0);
}

fn main() {
    check_solution(coin_change1);
    check_solution(coin_change2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, coin_change1, coin_change2};

    #[test]
    fn test_coin_change1() {
        check_solution(coin_change1);
    }

    #[test]
    fn test_coin_change2() {
        check_solution(coin_change2);
    }
}

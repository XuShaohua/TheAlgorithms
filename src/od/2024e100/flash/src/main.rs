// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::io::{stdin, BufRead};
use std::mem;

fn num_to_card(num: i32) -> &'static str {
    match num {
        // 从3到 A, 2
        // 忽略无效的值
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        10 => "10",
        11 => "J",
        12 => "Q",
        13 => "K",
        14 => "A",
        15 => "2",
        _ => panic!("Invalid card num"),
    }
}

// 将牌转换成整数
fn card_to_num(card: &str) -> i32 {
    match card {
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "10" => 10,
        "J" => 11,
        "Q" => 12,
        "K" => 13,
        "A" => 14,
        "2" => 15,
        _ => panic!("Invalid card"),
    }
}

fn remove_slice<T: PartialEq>(list1: &mut Vec<T>, list2: &[T]) -> usize {
    let mut count = 0;
    for item in list2 {
        if let Some(pos) = list1.iter().position(|x| x == item) {
            list1.remove(pos);
            count += 1;
        }
    }
    count
}

fn main() {
    // 读取所有的牌
    let mut line = String::new();
    let ret = stdin().lock().read_line(&mut line);
    assert!(ret.is_ok());
    let mut cards: Vec<i32> = line.split_ascii_whitespace().map(card_to_num).collect();
    cards.sort_unstable();
    assert_eq!(cards.len(), 13);

    let mut flash_list = Vec::new();
    let last_flash_card = 15;
    let mut first_card = 3;
    // 找出所有的顺子
    while first_card < 10 {
        // 顺子中的第一张牌.
        if !cards.contains(&first_card) {
            first_card += 1;
            continue;
        }

        let mut temp_cards = Vec::new();
        for card in first_card..last_flash_card {
            if cards.contains(&card) {
                temp_cards.push(card);
            } else if temp_cards.len() >= 5 {
                // 有效顺子, 保存顺子
                remove_slice(&mut cards, &temp_cards);
                let mut flash = Vec::new();
                mem::swap(&mut flash, &mut temp_cards);
                flash_list.push(flash);
            } else {
                // 无效顺子
                remove_slice(&mut cards, &temp_cards);
                temp_cards.clear();
            }
        }

        // 检查最后一组顺子
        if temp_cards.len() >= 5 {
            // 保存顺子
            remove_slice(&mut cards, &temp_cards);
            flash_list.push(temp_cards);
        }
    }

    // 给顺子排序
    // 1. 基于顺子中的第一张牌
    // 2. 基于顺子的长度
    flash_list.sort_by_key(|flash| (flash[0], flash.len()));

    // 打印结果
    // 将数字转换成牌
    if !flash_list.is_empty() {
        println!("{}", flash_list.len());
        for flash in flash_list {
            let s = flash
                .into_iter()
                .map(num_to_card)
                .collect::<Vec<_>>()
                .join(" ");
            println!("{s}");
        }
    } else {
        println!("No");
    }
}

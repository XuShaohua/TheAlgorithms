#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import sys

def num_to_card(num: int) -> str:
    mapping = [
        # 忽略无效的值
        "", "", "",
        # 从3到 A, 2
        "3", "4", "5", "6", "7", "8", "9", "10",
        "J", "Q", "K", "A", "2"
    ]
    return mapping[num]

def card_to_num(card: str) -> int:
    mapping = {
        "3": 3,
        "4": 4,
        "5": 5,
        "6": 6,
        "7": 7,
        "8": 8,
        "9": 9,
        "10": 10,
        "J": 11,
        "Q": 12,
        "K": 13,
        "A": 14,
        "2": 15,
    }
    return mapping[card]

def main():
    # 读取所有的牌
    cards = [card_to_num(card) for card in input().split()]
    cards.sort()
    assert len(cards) == 13

    flash_list = []
    last_flash_card = 15

    first_card = 3
    while first_card < 10:
        if first_card not in cards:
            first_card += 1
            continue

        temp_cards = []
        for card in range(first_card, last_flash_card):
            if card in cards:
                temp_cards.append(card)
            elif len(temp_cards) >= 5:
                # 保存顺子
                for card in temp_cards:
                    cards.remove(card)
                flash_list.append(temp_cards)
                temp_cards = []
            else:
                for card in temp_cards:
                    cards.remove(card)
                temp_cards = []

        # 检查最后一组顺子
        if len(temp_cards) >= 5:
            # 保存顺子
            for card in temp_cards:
                cards.remove(card)
            flash_list.append(temp_cards)
            temp_cards = []

    # 给顺子排序
    # 1. 基于顺子中的第一张牌
    # 2. 基于顺子的长度
    flash_list.sort(key = lambda flash: (flash[0], len(flash)))

    # 打印结果
    # 将数字转换成牌
    if flash_list:
        print(len(flash_list))
        for flash in flash_list:
            print(" ".join(num_to_card(num) for num in flash))
    else:
        print("No")

if __name__ == "__main__":
    main()

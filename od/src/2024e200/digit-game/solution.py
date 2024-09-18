#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import sys

def main():
    # 读取输入
    for line in sys.stdin:
        # 读取牌的数量
        n, m = list(map(int, line.split()))
        assert 1 <= n <= 1000
        assert 1 <= m <= 400000
        # 读取n张牌的数字
        card_numbers = list(map(int, input().split()))

        # 使用前缀和, 遍历所有的牌
        prefix_sum = []
        last_sum = 0
        for card in card_numbers:
            last_sum += card
            prefix_sum.append(last_sum)

        found = False

        # 然后计算两张牌之间的所有牌之和
        for i in range(n):
            for j in range(i + 1, n):
                card_sum = prefix_sum[j] - prefix_sum[i]
                if card_sum % m == 0:
                    found = True
                    break
            if found:
                break

        print(1 if found else 0)

if __name__ == "__main__":
    main()

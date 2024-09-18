#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 读取输入
    # 计算所有苹果的异或和, 如果不为0, 则没有办法按照A的方法来分, 直接返回
    # 计算所有苹果的重量之和
    # 找到最小重量的苹果并把它分给A, 剩下的都分给B
    num_apples = int(input())
    apple_weights = list(map(int, input().split()))
    assert len(apple_weights) == num_apples
    assert num_apples <= 20000

    MAX_WEIGHT = 10000
    xor_sum = 0
    total_weight = 0
    min_weight = MAX_WEIGHT
    for weight in apple_weights:
        xor_sum = xor_sum ^ weight
        total_weight += weight
        min_weight = min(min_weight, weight)

    if xor_sum != 0:
        print(-1)
    else:
        # 将 min_weight 那个苹果分给A, 剩下的都给B
        remains = total_weight - min_weight
        print(remains)

if __name__ == "__main__":
    main()

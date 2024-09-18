#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 读取输入
    # 确定有人数 n
    # 确定总共喊了多少声
    # 基于此, 就可以确定喊的顺序
    pass_list = list(map(int, input().split()))
    # 得到人数
    num_people = len(pass_list)
    assert 0 < num_people
    # 喊过的总次数
    total_passes = sum(pass_list)

    # 喊过的条件是:
    # 1. 当前数字是7的倍数
    # 2. 当前数字中包含7

    # 每个人真正喊过的次数
    real_pass_list = [0] * num_people

    # 当前的喊的数字 k, 注意数字是从1开始
    current_num = 1
    # 当前喊数的人在队列中的位置, 从0开始计数
    current_person = 0

    # 一直循环, 直到喊过的次数用完了
    while total_passes > 0:
        if current_num % 7 == 0 or "7" in str(current_num):
            # 这个人要喊过
            real_pass_list[current_person] += 1
            total_passes -= 1
        current_num += 1
        current_person = (current_person + 1) % num_people

    # 打印结果
    print(" ".join(map(str, real_pass_list)))

if __name__ == "__main__":
    main()

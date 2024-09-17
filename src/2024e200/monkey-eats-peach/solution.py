#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import math

def can_finish(peaches, leave_hours, eat_speed):
    ans = 0
    for peach in peaches:
        # 每棵树上花的时间, 不够一个小时, 就算一个小时, 因为猴子要慢吃
        ans += math.ceil(peach / eat_speed)
    # 吃的总时间不大于离开的时间
    return ans <= leave_hours

def main():
    peaches = list(map(int, input().split()))
    hours = int(input())
    n = len(peaches)
    if n == 0 or n >= 1000 or hours <= 0 or hours >= 10000:
        print(0)
        return

    # 二分查找法找出吃的速度的最小值
    left = 1
    right = 10 ** 9
    while left < right:
        middle = left + (right - left) // 2
        if can_finish(peaches, hours, middle):
            right = middle
        else:
            left = middle + 1

    # 如果最快的速度仍然吃不完, 那就无解
    if left == right:
        print(0)
    else:
        print(left)


if __name__ == "__main__":
    main()

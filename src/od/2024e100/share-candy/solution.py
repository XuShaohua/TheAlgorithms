#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import sys

def main():
    cache = {1: 0}

    # 缓存 + 递归
    def get_minimum_times(num: int) -> int:
        if num in cache:
            return cache[num]
        if num % 2 == 0:
            # 如果是偶数个
            # 平均分一次
            times = 1 + get_minimum_times(num // 2)
            cache[num] = times
            return times
        else:
            # 如果是奇数个, 有两种方式:
            # 取一个
            times1 = 1 + get_minimum_times(num + 1)
            # 放一个
            times2 = 1 + get_minimum_times(num - 1)

            # 求它们的最小值
            min_times = min(times1, times2)
            cache[num] = min_times
            return min_times

    sweet = int(input().strip())
    times = get_minimum_times(sweet)
    print(times)

if __name__ == "__main__":
    main()

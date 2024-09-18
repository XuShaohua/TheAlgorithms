#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    try:
        heights = list(map(int, input().split()))
    except ValueError:
        # Invalid input
        print("[]")
        return
    
    i = 0
    j = 1
    while j < len(heights):
        # 交换相邻小朋友的条件:
        # - 相邻的两个小朋友身高不相同
        # - 如果 i 是偶数, 并且 i 位小朋友的身高小于右侧 j 位的身高
        # - 如果 i 是奇数, 并且 i 位小朋友的身高大于右侧 j 位的身高
        if heights[i] != heights[j] and ((heights[i] > heights[j]) != (i % 2 == 0)):
            heights[i], heights[j] = heights[j], heights[i]
        i += 1
        j += 1

    print(" ".join(map(str, heights)))

if __name__ == "__main__":
    main()

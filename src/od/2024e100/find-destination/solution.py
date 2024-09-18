#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    def get_steps(first_step):
        print("first step:", first_step)
        pos = 0 + first_step
        count = 0
        last_pos = num_len - 1
        while pos < last_pos :
            print("  pos:", pos, " +step:", nums[pos])
            pos += nums[pos]
            count += 1
        if pos == last_pos:
            print("  got end pos:", last_pos)
            return count
        else:
            return None

    # 先解析每个位置对应的步数
    nums = list(map(int, input().split()))
    num_len = len(nums)

    if num_len % 2 == 0:
        max_first_step = num_len // 2
    else:
        max_first_step = (num_len - 1) // 2 + 1

    # Brute force
    # 唯一变化的就是第一步的步长, 我们遍历它所有可能的步长
    all_steps = []
    for first_step in range(1, max_first_step):
        num_steps = get_steps(first_step)
        if num_steps:
            all_steps.append(num_steps)

    if not all_steps:
        print(-1)
        return

    # 对所有步长进行排序, 然后找到最小的步数
    all_steps.sort()
    print(all_steps[0])

if __name__ == "__main__":
    main()

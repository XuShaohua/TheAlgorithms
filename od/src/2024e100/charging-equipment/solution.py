#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 读取输入
    # 机器数量
    num_machines = int(input())
    assert 0 < num_machines
    # 每台机器的输出功率
    machine_powers = list(map(int, input().split()))
    assert len(machine_powers) == num_machines
    # 最大输出功率
    max_power = int(input())

    # 背包问题, DP
    # 思路一: 暴力法, 时间复杂度 O(2^n)

    target_power = 0

    def recursive(machine_index, power_sum):
        # 所有机器都访问完了
        if machine_index >= num_machines:
            diff = max_power - power_sum
            nonlocal target_power
            if 0 <= diff < (max_power - target_power):
                target_power = power_sum
            return

        # 递归调用
        recursive(machine_index + 1, power_sum + machine_powers[machine_index])
        recursive(machine_index + 1, power_sum)

    recursive(0, 0)

    # 打印结果
    print(target_power)

if __name__ == "__main__":
    main()

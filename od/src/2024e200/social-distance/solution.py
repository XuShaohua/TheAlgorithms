#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 读取输入
    # 座位的数量
    num_seat = int(input())
    assert 1 <= num_seat <= 500
    # 进出的序列
    line = input()[1:-1]
    # 1 表示进入会议室, 负数表示从该位置离开会议室
    enter_leave_list = list(map(int, line.split(",")))
    assert enter_leave_list

    # 下一个进入的员工可以占用的位置
    new_position = -1
    # 当前在会议室的所有员工占用的位置
    # 这个序列在有员工进出时都要更新
    seat = []

    # 遍历所有进出序列
    for op in enter_leave_list:
        # 有员工离开
        if op < 0:
            # 得到离开员工的位置
            position = -op
            # 并将该位置移除
            seat.remove(position)
            continue
        
        # 有员工进入会议室, 给他安排位置
        assert op == 1

        if not seat:
            # 当会议室为空时, 要坐在位置0, 而且之后这个位置上的员工不再变动
            new_position = 0
        elif len(seat) == num_seat:
            # 当会议室满了后, 新进入的员工没有位置
            new_position = -1
        else:
            # 找出最大空闲位置
            max_dist = 0
            new_position = 0
            # 遍历已有位置序列
            for index, position in enumerate(seat):
                # 当前位置的距离
                distance = 0
                if index + 1 == len(seat):
                    # 最后一个位置到当前位置的距离
                    distance = num_seat - 1 - position
                else:
                    # 相邻两个位置的中间距离
                    distance = (seat[index + 1] - position) // 2
                # 更新最大距离
                if distance > max_dist:
                    max_dist = distance
                    # 更新新进入员工的位置
                    if index + 1 == len(seat):
                        # 最后一个位置
                        new_position = num_seat - 1
                    else:
                        # 相邻两个位置的中间位置
                        new_position = position + distance

        if new_position != -1:
            # 将新进入的员工排好位
            seat.append(new_position)
            seat.sort()

    # 打印最终的结果
    print(new_position)

if __name__ == "__main__":
    main()

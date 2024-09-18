#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 先读取输入
    # 然后构造列表, 将每个app都注册进去
    # 最后查找给定时间点内的app

    num_apps = int(input())
    input_apps = []
    for i in range(num_apps):
        parts = input().split()
        assert len(parts) == 4
        # 将时间点转换成整数
        start_time = int(parts[2].replace(":", ""))
        end_time = int(parts[3].replace(":", ""))
        app = (parts[0], int(parts[1]), start_time, end_time)
        input_apps.append(app)
    # 读取查询的时间点
    searched_time = int(input().replace(":", ""))

    NAME = 0
    PRIORITY = 1
    START_TIME = 2
    END_TIME = 3

    # 注册app
    validated_apps = []
    for app in input_apps:
        skip_app = False

        for i in range(len(validated_apps)):
            valid_app = validated_apps[i]
            # 检查时间段是否有冲突
            if valid_app[START_TIME] >= app[END_TIME] or valid_app[END_TIME] <= app[START_TIME]:
                continue
            # 检查优先级, 如果待注册的app优先级不高于已注册的app, 就不注册它
            if valid_app[PRIORITY] >= app[PRIORITY]:
                skip_app = True
            else:
                # 将已注册的app清除, 因为它优先级更低
                validated_apps.pop(i)
            break
        if not skip_app:
            validated_apps.append(app)

    for app in validated_apps:
        # 查询的时间点在该 app 的服务时间段内
        if app[START_TIME] <= searched_time < app[END_TIME]:
            print(app[0])
            return

    print("NA")

if __name__ == "__main__":
    main()

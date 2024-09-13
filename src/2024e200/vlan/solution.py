#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 读取输入
    # 解析当前所有的 VLAN ID, 并存储到集合或者数组中
    # 然后移除指定的 ID
    # 最后新剩下的 ID 格式化输出
    parts = input().split(",")
    id_set = set()
    for part in parts:
        if "-" in part:
            range_part = part.split("-")
            start_id = int(range_part[0])
            end_id = int(range_part[1])
            for vlan_id in range(start_id, end_id + 1):
                assert 1 <= vlan_id <= 4094
                id_set.add(vlan_id)
        else:
            vlan_id = int(part)
            assert 1 <= vlan_id <= 4094
            id_set.add(vlan_id)
    assert id_set
    removed_id = int(input())

    if removed_id in id_set:
        id_set.remove(removed_id)

    # 格式化输出
    # 先转换成列表, 再排序
    id_list = list(id_set)
    id_list.sort()

    start_id = -1
    last_id = -1
    out = []
    for vlan_id in id_list:
        if last_id + 1 == vlan_id:
            # 连续 ID
            last_id = vlan_id
        else:
            # 重置连续 ID
            if last_id == -1:
                pass
            elif last_id == start_id:
                # 单个值
                out.append(str(last_id))
            else:
                # 范围
                out.append(F"{start_id}-{last_id}")
            start_id = vlan_id
            last_id = vlan_id

    # 处理最后一个元素
    if last_id == start_id:
        # 单个值
        out.append(str(last_id))
    else:
        # 范围
        out.append(F"{start_id}-{last_id}")

    # 打印结果
    print(",".join(out))

if __name__ == "__main__":
    main()

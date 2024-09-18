// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashSet;
use std::io::{stdin, BufRead};

fn solution() {
    // 读取输入
    // 解析当前所有的 VLAN ID, 并存储到集合或者数组中
    // 然后移除指定的 ID
    // 最后新剩下的 ID 格式化输出
    let mut line = String::new();
    let ret = stdin().lock().read_line(&mut line);
    assert!(ret.is_ok());
    let mut id_set = HashSet::<u32>::new();
    for part in line.trim().split(",") {
        if part.contains("-") {
            let mut range_part = part.split("-");
            let start_id: u32 = range_part.next().unwrap().parse().unwrap();
            let end_id: u32 = range_part.next().unwrap().parse().unwrap();
            assert!(range_part.next().is_none());
            for vlan_id in start_id..=end_id {
                assert!(1 <= vlan_id && vlan_id <= 4094);
                id_set.insert(vlan_id);
            }
        } else {
            let vlan_id: u32 = part.parse().unwrap();
            assert!(1 <= vlan_id && vlan_id <= 4094);
            id_set.insert(vlan_id);
        }
    }
    assert!(!id_set.is_empty());
    line.clear();

    let ret = stdin().lock().read_line(&mut line);
    assert!(ret.is_ok());
    let removed_id: u32 = line.trim().parse().unwrap();

    id_set.remove(&removed_id);

    // 格式化输出
    // 先转换成列表, 再排序
    let mut id_list: Vec<u32> = id_set.into_iter().collect();
    id_list.sort_unstable();

    let mut start_id = u32::MAX;
    let mut last_id = u32::MAX;
    let mut out = Vec::new();
    for &vlan_id in &id_list {
        if vlan_id - 1 == last_id {
            // 连续 ID
            last_id = vlan_id;
        } else {
            // 重置连续 ID
            if last_id == u32::MAX {
                // 忽略
            } else if last_id == start_id {
                // 单个值
                out.push(last_id.to_string());
            } else {
                // 范围
                out.push(format!("{start_id}-{last_id}"));
            }
            start_id = vlan_id;
            last_id = vlan_id;
        }
    }

    // 处理最后一个元素
    if last_id == start_id {
        // 单个值
        out.push(last_id.to_string());
    } else {
        // 范围
        out.push(format!("{start_id}-{last_id}"));
    }

    // 打印结果
    println!("{}", out.join(","));
}

fn main() {
    solution();
}

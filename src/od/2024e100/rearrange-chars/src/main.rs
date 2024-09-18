// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::io::{stdin, BufRead};

fn main() {
    // 创建计数数组
    // 读取输入, 并统计所有字母出现的次数
    // 统计出现的最多次数
    // 然后逆向遍历所有的次数, 并打印结果
    let mut chars = [0_usize; 256];
    let mut line = String::new();
    let ret = stdin().lock().read_line(&mut line);
    assert!(ret.is_ok());

    for byte in line.trim().bytes() {
        let index: usize = byte as usize;
        chars[index] += 1;
    }

    let max_count: usize = chars.iter().max().copied().unwrap();
    let mut out = Vec::new();

    for count in (1..=max_count).rev() {
        // 遍历所有的小写字母
        for byte in b'a'..=b'z' {
            let index = byte as usize;
            if chars[index] == count {
                out.push(format!("{}:{count}", char::from_u32(byte as u32).unwrap()));
            }
        }

        // 遍历所有的大写字母
        for byte in b'A'..=b'Z' {
            let index = byte as usize;
            if chars[index] == count {
                out.push(format!("{}:{count}", char::from_u32(byte as u32).unwrap()));
            }
        }
    }

    // 打印结果
    let s = out.join(";");
    println!("{s}");
}

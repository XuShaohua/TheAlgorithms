// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;
use std::io::{stdin, BufRead};

fn solution() {
    // 读取输入, 并将它们转换成正整数
    // 然后创建一个空的栈
    // 遍历所有的正整数, 依次入栈
    // 每次入栈时做以下检查:
    // 1. 如果 n1=n2, 则它们全出栈, 然后自动入栈 2 * n1
    // 2. 如果 n1=sum(n2, n3..), 则它们全出栈, 然后自动入栈 2 * n1
    // 最后打印栈中剩下的整数

    let mut line = String::new();
    let ret = stdin().lock().read_line(&mut line);
    assert!(ret.is_ok());
    let numbers: Vec<i32> = line
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut stack: Vec<i32> = Vec::new();

    for &number in &numbers {
        // 检查规则1
        if stack.last() == Some(&number) {
            stack.pop();
            stack.push(number * 2);
            continue;
        }

        let mut top_sum = 0;
        let mut will_append = true;

        // 从栈顶开始求和
        for i in (0..stack.len()).rev() {
            top_sum += stack[i];
            match top_sum.cmp(&number) {
                Ordering::Greater => {
                    // 不满足
                    break;
                }
                Ordering::Equal => {
                    // 满足规则2
                    stack.resize(i, 0);
                    stack.push(number * 2);
                    will_append = false;
                    break;
                }
                _ => (),
            }
        }

        if will_append {
            // 如果上面的规则不满足, 就把该整数入栈
            stack.push(number);
        }
    }

    // 打印结果
    let out = stack
        .into_iter()
        .rev()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{out}");
}

fn main() {
    solution();
}

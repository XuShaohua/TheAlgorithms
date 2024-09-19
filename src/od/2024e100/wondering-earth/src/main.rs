// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::io::{stdin, BufRead};

fn main() {
    // 读取输入
    let mut line = String::new();
    let ret = stdin().lock().read_line(&mut line);
    assert!(ret.is_ok());
    let mut parts = line.split_ascii_whitespace();
    let num_engines: usize = parts.next().unwrap().parse().unwrap();
    let num_initial_startup: usize = parts.next().unwrap().parse().unwrap();
    assert!((1..=1000).contains(&num_engines));
    assert!((1..=1000).contains(&num_initial_startup));

    // 引擎初始状态
    let mut initials = Vec::with_capacity(num_initial_startup);

    // 哪些引擎是被"手动启动"的
    for _i in 0..num_initial_startup {
        line.clear();
        let ret = stdin().lock().read_line(&mut line);
        assert!(ret.is_ok());
        let mut parts = line.split_ascii_whitespace();
        let tick: usize = parts.next().unwrap().parse().unwrap();
        let pos: usize = parts.next().unwrap().parse().unwrap();
        initials.push((tick, pos));
    }

    // 标记引擎是否点火
    let mut engines = vec![false; num_engines];

    let mut engines_started = 0;

    // 记录本轮中点火的引擎
    let mut started_this_round = Vec::<usize>::new();

    // 模拟每个时间点
    for tick in 0..num_engines {
        // 如果所有引擎都已点火, 就终止循环
        if engines_started == num_engines {
            break;
        }

        started_this_round.clear();

        // 当前时间点中的快照
        let mut snapshot = engines.clone();

        // "关联启动"模式, 启动相邻的引擎
        for (index, engine_started) in engines.iter().enumerate() {
            // 当前引擎已经被启动
            if *engine_started {
                println!("CHECK sibling: {index}");
                let previous_index = (num_engines + index - 1) % num_engines;
                let next_index = (index + 1) % num_engines;
                if !snapshot[previous_index] {
                    snapshot[previous_index] = true;
                    started_this_round.push(previous_index);
                    engines_started += 1;
                    println!("  START previous: {previous_index}");
                }
                if !snapshot[next_index] {
                    snapshot[next_index] = true;
                    started_this_round.push(next_index);
                    engines_started += 1;
                    println!("  START next: {next_index}");
                }
            }
        }

        // 检查"手动启动"的引擎
        for (initial_tick, initial_position) in &initials {
            if *initial_tick == tick && !snapshot[*initial_position] {
                snapshot[*initial_position] = true;
                engines_started += 1;
                started_this_round.push(*initial_position);
                println!("START initial: {initial_position}");
            }
        }

        // 保存快照
        engines.clone_from(&snapshot);
    }

    // 打印结果
    println!("{}", started_this_round.len());
    let s = started_this_round
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{s}");
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;
use std::io::{stdin, BufRead};

fn solution() {
    let mut salary_tree = HashMap::<i32, (i32, i32)>::new();
    let mut hierachy_tree = HashMap::<i32, Vec<i32>>::new();

    // 读取输入
    let mut line = String::new();
    let ret = stdin().lock().read_line(&mut line);
    assert!(ret.is_ok());
    let num_persons: usize = line.trim().parse().unwrap();

    let mut parent_id: i32 = 0;
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        let mut parts = line.split_ascii_whitespace();
        let current_id: i32 = parts.next().unwrap().parse().unwrap();
        parent_id = parts.next().unwrap().parse().unwrap();
        let salary: i32 = parts.next().unwrap().parse().unwrap();
        salary_tree.insert(current_id, (parent_id, salary));
        hierachy_tree.entry(parent_id).or_default().push(current_id);
    }
    assert_eq!(num_persons, salary_tree.len());

    // 先得到 boss 的 ID
    let mut boss_id = parent_id;
    while let Some(entry) = salary_tree.get(&boss_id) {
        boss_id = entry.0;
    }
    //println!("boss id: {boss_id}");

    // 递归计算一个 ID 的收入
    fn get_salary_recursive(
        parent_id: i32,
        salary_tree: &mut HashMap<i32, (i32, i32)>,
        hierachy_tree: &HashMap<i32, Vec<i32>>,
    ) -> i32 {
        if let Some(children) = hierachy_tree.get(&parent_id) {
            let mut total_salary = 0;
            for &child in children {
                let salary = get_salary_recursive(child, salary_tree, hierachy_tree);
                total_salary += (salary / 100) * 15;
            }
            total_salary
        } else {
            // 得到当前用户的收入
            salary_tree[&parent_id].1
        }
    }

    let boss_salary = get_salary_recursive(boss_id, &mut salary_tree, &hierachy_tree);
    println!("{boss_salary}");
}

fn main() {
    solution();
}

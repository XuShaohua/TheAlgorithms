// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

// HashTable
pub fn find_duplicate1(paths: Vec<String>) -> Vec<Vec<String>> {
    debug_assert!(!paths.is_empty());

    // (file-content, paths)
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for path in &paths {
        let mut dir: String = String::new();
        //println!("path: {path}");
        for (index, part) in path.split(' ').enumerate() {
            // 解析目录名
            if index == 0 {
                //println!("dirname: {dir}");
                part.clone_into(&mut dir);
                continue;
            }

            // 解析文件名和文件内容
            if let Some(pos) = part.find('(') {
                let filename = &part[..pos];
                let filepath = [&dir, filename].join("/");
                let content: String = part[(pos + 1)..].to_owned();

                map.entry(content).or_default().push(filepath);
            }
        }
    }
    //println!("map: {map:?}");

    map.into_values().filter(|paths| paths.len() > 1).collect()
}

// HashTable
// 优化字符串处理, 去掉 if 语句
pub fn find_duplicate2(paths: Vec<String>) -> Vec<Vec<String>> {
    debug_assert!(!paths.is_empty());

    // (file-content, paths)
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for path in &paths {
        // 解析目录和文件列表
        let (dir, filenames) = path.split_once(' ').unwrap();
        for file_entry in filenames.split(' ') {
            // 解析文件名和文件内容
            let (filename, content) = file_entry.split_once('(').unwrap();
            let filepath = [dir, filename].join("/");
            map.entry(content.to_owned()).or_default().push(filepath);
        }
    }

    map.into_values().filter(|paths| paths.len() > 1).collect()
}

pub type SolutionFn = fn(Vec<String>) -> Vec<Vec<String>>;

fn check_solution(func: SolutionFn) {
    fn check_equal(mut v1: Vec<Vec<String>>, mut v2: Vec<Vec<String>>) {
        v1.sort_unstable();
        v2.sort_unstable();
        assert_eq!(v1, v2);
    }

    let paths = vec![
        "root/a 1.txt(abcd) 2.txt(efgh)".to_owned(),
        "root/c 3.txt(abcd)".to_owned(),
        "root/c/d 4.txt(efgh)".to_owned(),
        "root 4.txt(efgh)".to_owned(),
    ];
    let out = vec![
        vec![
            "root/a/2.txt".to_owned(),
            "root/c/d/4.txt".to_owned(),
            "root/4.txt".to_owned(),
        ],
        vec!["root/a/1.txt".to_owned(), "root/c/3.txt".to_owned()],
    ];
    check_equal(func(paths), out);

    let paths = vec![
        "root/a 1.txt(abcd) 2.txt(efgh)".to_owned(),
        "root/c 3.txt(abcd)".to_owned(),
        "root/c/d 4.txt(efgh)".to_owned(),
    ];
    let out = vec![
        vec!["root/a/2.txt".to_owned(), "root/c/d/4.txt".to_owned()],
        vec!["root/a/1.txt".to_owned(), "root/c/3.txt".to_owned()],
    ];
    check_equal(func(paths), out);

    let paths = vec![
        "root/a 1.txt(abcd) 2.txt(efsfgh)".to_owned(),
        "root/c 3.txt(abdfcd)".to_owned(),
        "root/c/d 4.txt(efggdfh)".to_owned(),
    ];
    let out = vec![];
    check_equal(func(paths), out);
}

fn main() {
    check_solution(find_duplicate1);
    check_solution(find_duplicate2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, find_duplicate1, find_duplicate2};

    #[test]
    fn test_find_duplicate1() {
        check_solution(find_duplicate1);
    }

    #[test]
    fn test_find_duplicate2() {
        check_solution(find_duplicate2);
    }
}

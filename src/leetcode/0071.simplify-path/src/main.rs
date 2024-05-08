// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Stack
pub fn simplify_path1(path: String) -> String {
    let mut stack: Vec<String> = Vec::new();
    for part in path.split('/') {
        match part {
            // 忽略空白
            "" => (),
            // 忽略当前目录
            "." => (),
            // 返回上个目录
            ".." => {
                let _ = stack.pop();
            }
            part => stack.push(part.to_owned()),
        }
    }
    // 根目录
    let mut result = "/".to_owned();
    result.push_str(&stack.join("/"));
    result
}

// Stack
// 优化字符串操作
pub fn simplify_path2(path: String) -> String {
    let mut stack: Vec<&str> = Vec::with_capacity(path.len());
    for part in path.split('/') {
        match part {
            // 忽略空白
            "" => (),
            // 忽略当前目录
            "." => (),
            // 返回上个目录
            ".." => {
                let _ = stack.pop();
            }
            part => stack.push(part),
        }
    }
    // 目录以 "/" 开头
    format!("/{}", stack.join("/"))
}

pub type SolutionFn = fn(String) -> String;

fn check_solution(func: SolutionFn) {
    let path = "/home/".to_owned();
    assert_eq!(&func(path), "/home");

    let path = "/home//foo/".to_owned();
    assert_eq!(&func(path), "/home/foo");

    let path = "/home/user/Documents/../Pictures".to_owned();
    assert_eq!(&func(path), "/home/user/Pictures");

    let path = "/../".to_owned();
    assert_eq!(&func(path), "/");

    let path = "/.../a/../b/c/../d/./".to_owned();
    assert_eq!(&func(path), "/.../b/d");
}

fn main() {
    check_solution(simplify_path1);
    check_solution(simplify_path2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, simplify_path1, simplify_path2};

    #[test]
    fn test_simplify_path1() {
        check_solution(simplify_path1);
    }

    #[test]
    fn test_simplify_path2() {
        check_solution(simplify_path2);
    }
}

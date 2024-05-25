// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Backtracking
pub fn exist1(board: Vec<Vec<char>>, word: String) -> bool {
    fn backtracking(
        board: &[Vec<char>],
        chars: &[char],
        index: usize,
        visited: &mut Vec<Vec<bool>>,
        path: &mut Vec<(usize, usize)>,
        res: &mut Vec<Vec<(usize, usize)>>,
    ) {
        // 终止条件, chars 都被访问了, 或者当前位置已经无路可走了.
        if index == chars.len() {
            // 存在
            res.push(path.clone());
            return;
        }

        let target: char = chars[index];

        if let Some((last_row, last_col)) = path.last().copied() {
            // 只搜索邻近的几个点
            let mut abid: Vec<(usize, usize)> = Vec::new();
            // [
            //   (last_row - 1, last_col),
            //   (last_row + 1, last_col),
            //   (last_row, last_col - 1),
            //   (last_row, last_col + 1),
            // ];
            if last_row > 0 {
                abid.push((last_row - 1, last_col));
            }
            if last_row + 1 < board.len() {
                abid.push((last_row + 1, last_col));
            }
            if last_col > 0 {
                abid.push((last_row, last_col - 1));
            }
            if last_col + 1 < board[0].len() {
                abid.push((last_row, last_col + 1));
            }

            for (row, col) in abid.into_iter() {
                // 当前元素与目标元素相等, 而且当前元素在之前还没有被访问过.
                if board[row][col] == target && !visited[row][col] {
                    // 访问元素
                    path.push((row, col));
                    visited[row][col] = true;

                    // 递归搜索
                    backtracking(board, chars, index + 1, visited, path, res);

                    // 取消访问
                    path.pop();
                    visited[row][col] = false;
                }
            }
        } else {
            // 搜索第一个字符可能的入口点
            for row in 0..board.len() {
                for col in 0..board[0].len() {
                    if board[row][col] == target {
                        // 访问元素
                        path.push((row, col));
                        visited[row][col] = true;

                        // 递归搜索
                        backtracking(board, chars, index + 1, visited, path, res);

                        // 取消访问
                        path.pop();
                        visited[row][col] = false;
                    }
                }
            }
        }
    }

    debug_assert!((1..=6).contains(&board.len()));
    debug_assert!((1..=15).contains(&word.len()));

    // 将字符串转换成 char array.
    let chars: Vec<char> = word.chars().collect();

    // 记录访问过的所有字符所在的坐标.
    let mut res: Vec<Vec<(usize, usize)>> = Vec::new();
    // 记录当前路径
    let mut path: Vec<(usize, usize)> = Vec::new();
    // 记录字符表上的位置有没有访问过.
    let mut visited: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];

    backtracking(&board, &chars, 0, &mut visited, &mut path, &mut res);
    !res.is_empty()
}

// 对上面的方法做一些优化
pub fn exist2(board: Vec<Vec<char>>, word: String) -> bool {
    fn backtracking(
        board: &[Vec<char>],
        chars: &[char],
        index: usize,
        visited: &mut Vec<Vec<bool>>,
        path: &mut Vec<(usize, usize)>,
        res: &mut Vec<Vec<(usize, usize)>>,
    ) {
        #[allow(clippy::too_many_arguments)]
        fn visit_char(
            board: &[Vec<char>],
            chars: &[char],
            index: usize,
            visited: &mut Vec<Vec<bool>>,
            path: &mut Vec<(usize, usize)>,
            res: &mut Vec<Vec<(usize, usize)>>,
            row: usize,
            col: usize,
            target: char,
        ) {
            // 当前元素与目标元素相等, 而且当前元素在之前还没有被访问过.
            if board[row][col] == target && !visited[row][col] {
                // 访问元素
                path.push((row, col));
                visited[row][col] = true;

                // 递归搜索
                backtracking(board, chars, index + 1, visited, path, res);

                // 取消访问
                path.pop();
                visited[row][col] = false;
            }
        }

        // 终止条件, chars 都被访问了, 或者当前位置已经无路可走了.
        if index == chars.len() {
            // 存在
            res.push(path.clone());
            return;
        }

        let target: char = chars[index];

        if let Some((last_row, last_col)) = path.last().copied() {
            // 只搜索邻近的几个点
            if last_row > 0 {
                visit_char(
                    board,
                    chars,
                    index,
                    visited,
                    path,
                    res,
                    last_row - 1,
                    last_col,
                    target,
                );
            }
            if last_row + 1 < board.len() {
                visit_char(
                    board,
                    chars,
                    index,
                    visited,
                    path,
                    res,
                    last_row + 1,
                    last_col,
                    target,
                );
            }
            if last_col > 0 {
                visit_char(
                    board,
                    chars,
                    index,
                    visited,
                    path,
                    res,
                    last_row,
                    last_col - 1,
                    target,
                );
            }
            if last_col + 1 < board[0].len() {
                visit_char(
                    board,
                    chars,
                    index,
                    visited,
                    path,
                    res,
                    last_row,
                    last_col + 1,
                    target,
                );
            }
        } else {
            // 搜索第一个字符可能的入口点
            for row in 0..board.len() {
                for col in 0..board[0].len() {
                    visit_char(board, chars, index, visited, path, res, row, col, target);
                }
            }
        }
    }

    debug_assert!((1..=6).contains(&board.len()));
    debug_assert!((1..=15).contains(&word.len()));

    // 将字符串转换成 char array.
    let chars: Vec<char> = word.chars().collect();

    // 记录访问过的所有字符所在的坐标.
    let mut res: Vec<Vec<(usize, usize)>> = Vec::new();
    // 记录当前路径
    let mut path: Vec<(usize, usize)> = Vec::new();
    // 记录字符表上的位置有没有访问过.
    let mut visited: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];

    backtracking(&board, &chars, 0, &mut visited, &mut path, &mut res);
    !res.is_empty()
}

pub fn exist3(board: Vec<Vec<char>>, word: String) -> bool {
    fn backtracking(
        board: &[Vec<char>],
        chars: &[char],
        index: usize,
        row: usize,
        col: usize,
        visited: &mut [Vec<bool>],
    ) -> bool {
        // 终止条件是, 访问到了最后一个字符. 要检查它与当前字符相等.
        if index + 1 == chars.len() {
            return chars[index] == board[row][col];
        }

        if board[row][col] == chars[index] {
            // 访问元素
            visited[row][col] = true;

            // 递归搜索
            // 只搜索邻近的几个点
            let mut abid: Vec<(usize, usize)> = Vec::with_capacity(4);
            if row > 0 {
                abid.push((row - 1, col));
            }
            if row + 1 < board.len() {
                abid.push((row + 1, col));
            }
            if col > 0 {
                abid.push((row, col - 1));
            }
            if col + 1 < board[0].len() {
                abid.push((row, col + 1));
            }
            for (new_row, new_col) in abid.into_iter() {
                if !visited[new_row][new_col]
                    && backtracking(board, chars, index + 1, new_row, new_col, visited)
                {
                    return true;
                }
            }

            // 撤销访问
            visited[row][col] = false;
        }

        false
    }

    debug_assert!((1..=15).contains(&word.len()));
    let rows: usize = board.len();
    debug_assert!((1..=6).contains(&rows));
    let cols: usize = board[0].len();
    debug_assert!((1..=6).contains(&cols));

    let chars: Vec<char> = word.chars().collect();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];

    for row in 0..rows {
        for col in 0..cols {
            if backtracking(&board, &chars, 0, row, col, &mut visited) {
                return true;
            }
        }
    }
    false
}

pub type SolutionFn = fn(Vec<Vec<char>>, String) -> bool;

fn check_solution(func: SolutionFn) {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = "ABCCED".to_owned();
    assert!(func(board, word));

    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = "SEE".to_owned();
    assert!(func(board, word));

    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = "ABCB".to_owned();
    assert!(!func(board, word));
}

fn main() {
    //check_solution(exist1);
    //check_solution(exist2);
    check_solution(exist3);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, exist1};

    #[test]
    fn test_exist1() {
        check_solution(exist1);
    }
}

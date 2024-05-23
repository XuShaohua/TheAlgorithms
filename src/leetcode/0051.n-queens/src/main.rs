// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Backtracking
pub fn solve_n_queens1(n: i32) -> Vec<Vec<String>> {
    #[allow(clippy::needless_range_loop)]
    fn is_valid(n: usize, row: usize, col: usize, chessboard: &[Vec<char>]) -> bool {
        // 先判断前面的行
        for i in 0..row {
            if chessboard[i][col] == 'Q' {
                return false;
            }
        }

        // 然后判断左上角对角方向
        if row >= 1 && col >= 1 {
            let mut i = row - 1;
            let mut j = col - 1;
            loop {
                if chessboard[i][j] == 'Q' {
                    return false;
                }
                if i == 0 || j == 0 {
                    break;
                }
                i -= 1;
                j -= 1;
            }
        }

        // 最后判断右上角对角方向
        if row >= 1 && col + 1 < n {
            let mut i = row - 1;
            let mut j = col + 1;
            loop {
                if chessboard[i][j] == 'Q' {
                    return false;
                }
                if i == 0 || j + 1 == n {
                    break;
                }
                i -= 1;
                j += 1;
            }
        }

        true
    }

    fn backtracking(
        n: usize,
        row: usize,
        chessboard: &mut Vec<Vec<char>>,
        res: &mut Vec<Vec<String>>,
    ) {
        // 终止条件, 所有行都摆放了棋子.
        if row >= n {
            let mut map = Vec::new();
            for row_chars in chessboard.iter() {
                let row_str = String::from_iter(row_chars);
                map.push(row_str);
            }

            res.push(map);
            return;
        }

        // 遍历当前行的每个列
        for col in 0..n {
            // 如果该 (row, col) 组合可以放置, 就继续
            if is_valid(n, row, col, chessboard) {
                // 放置皇后
                chessboard[row][col] = 'Q';

                // 递归搜索
                backtracking(n, row + 1, chessboard, res);

                // 撤销放置
                chessboard[row][col] = '.';
            }
        }
    }

    debug_assert!(n > 0);
    let n: usize = n as usize;

    // 存放所有结果
    let mut res: Vec<Vec<String>> = Vec::new();
    // 存放当前棋盘上的状态, n x n 的矩阵.
    let mut chessboard: Vec<Vec<char>> = vec![vec!['.'; n]; n];
    backtracking(n, 0, &mut chessboard, &mut res);
    res
}

pub type SolutionFn = fn(i32) -> Vec<Vec<String>>;

fn check_solution(func: SolutionFn) {
    let n = 4;
    let expected = vec![
        vec![
            ".Q..".to_owned(),
            "...Q".to_owned(),
            "Q...".to_owned(),
            "..Q.".to_owned(),
        ],
        vec![
            "..Q.".to_owned(),
            "Q...".to_owned(),
            "...Q".to_owned(),
            ".Q..".to_owned(),
        ],
    ];
    assert_eq!(func(n), expected);

    let n = 1;
    let expected = vec![vec!["Q"]];
    assert_eq!(func(n), expected);
}

fn main() {
    check_solution(solve_n_queens1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, solve_n_queens1};

    #[test]
    fn test_solve_n_queens1() {
        check_solution(solve_n_queens1);
    }
}

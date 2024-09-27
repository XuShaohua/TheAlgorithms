// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Brute force
// 时间复杂度 O(m * n)
pub fn island_perimeter1(grid: Vec<Vec<i32>>) -> i32 {
    // 一个岛屿有多个方格组成,  一个方格有四个边.
    // 对于任意一个边, 当与组成岛屿的另一个方格相邻时, 在计算岛的周长时,
    // 就不需要计算这个边;
    // 当不与组成岛屿的另一个方格相邻时, 在计算周长时, 就要算上这个边.
    // 一个岛域的周长, 就是这些方格的边的总和.
    // 遍历所有方格, 检查它是不是岛屿的一部分.
    // 如果是, 就检查它的四个边, 是否能作为岛域周长的一部分.

    const IS_ISLAND: i32 = 1;
    const IS_NOT_ISLAND: i32 = 0;
    let rows = grid.len();
    assert!((1..=100).contains(&rows));
    let columns = grid[0].len();
    assert!((1..=100).contains(&columns));
    let rows_i32 = rows as i32;
    let columns_i32 = columns as i32;
    // 可能的四个方向.
    let directions = [(0_i32, -1_i32), (-1, 0), (1, 0), (0, 1)];

    let mut sum = 0;
    for row in 0..rows {
        for column in 0..columns {
            if grid[row][column] == IS_ISLAND {
                // 尝试四个边
                for dir in directions {
                    let row1 = row as i32 + dir.0;
                    let column1 = column as i32 + dir.1;
                    if 0 <= row1 && row1 < rows_i32 && 0 <= column1 && column1 < columns_i32 {
                        // 如果相邻的方格是 grid 内部
                        let row1 = row1 as usize;
                        let column1 = column1 as usize;

                        // 并且它不是岛屿的一部分.
                        if grid[row1][column1] == IS_NOT_ISLAND {
                            sum += 1;
                        }
                    } else {
                        // 如果相邻的方格不是在 grid 内部, 说明这个边可以作为岛屿周长的一部分.
                        sum += 1;
                    }
                }
            }
        }
    }

    // 返回结果
    sum
}

pub type SolutionFn = fn(Vec<Vec<i32>>) -> i32;

fn check_solution(func: SolutionFn) {
    let grid = vec![
        vec![0, 1, 0, 0],
        vec![1, 1, 1, 0],
        vec![0, 1, 0, 0],
        vec![1, 1, 0, 0],
    ];
    assert_eq!(func(grid), 16);

    let grid = vec![vec![1]];
    assert_eq!(func(grid), 4);

    let grid = vec![vec![1, 0]];
    assert_eq!(func(grid), 4);
}

fn main() {
    check_solution(island_perimeter1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, island_perimeter1};

    #[test]
    fn test_island_perimeter1() {
        check_solution(island_perimeter1);
    }
}

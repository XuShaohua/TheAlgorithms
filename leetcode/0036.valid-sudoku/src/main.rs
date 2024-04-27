// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

fn sudoku_to_int(board: Vec<Vec<char>>) -> Vec<Vec<u8>> {
    assert_eq!(board.len(), 9);
    assert_eq!(board[0].len(), 9);

    let mut out = Vec::with_capacity(9);
    for row in board.iter() {
        let mut num_row = Vec::with_capacity(9);
        for item in row.iter() {
            let num = if *item == '.' {
                0
            } else {
                item.to_digit(10).unwrap() as u8
            };
            num_row.push(num);
        }
        out.push(num_row);
    }
    out
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let board = sudoku_to_int(board);
    eprintln!("board: {board:?}");
    false
}

fn main() {
    let board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert!(is_valid_sudoku(board));

    let board = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert!(!is_valid_sudoku(board));
}

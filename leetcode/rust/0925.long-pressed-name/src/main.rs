// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn is_long_pressed_name(name: String, typed: String) -> bool {
    let name = name.as_bytes();
    let typed = typed.as_bytes();
    assert!(!name.is_empty() && !typed.is_empty());

    let m = name.len();
    let n = typed.len();
    let mut i = 0;
    let mut j = 0;

    while i < m && j < n {
        if name[i] == typed[j] {
            i += 1;
            j += 1;
        } else if i > 0 && name[i - 1] == typed[j] {
            j += 1;
        } else {
            return false;
        }
    }

    while j < n && name[m - 1] == typed[j] {
        j += 1;
    }
    i == m && j == n
}

fn check_solution() {
    assert!(is_long_pressed_name("alex".to_owned(), "aaleex".to_owned()));
    assert!(!is_long_pressed_name(
        "saeed".to_owned(),
        "ssaaedd".to_owned()
    ));
}

fn main() {
    check_solution();
}

// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use ch1::quick_find::QuickFind;

fn main() {
    let mut qu = QuickFind::new(10);
    println!("Initial value: {:?}", qu);
    qu.union(4, 3);
    println!("After union(4, 3): {:?}", qu);
    qu.union(3, 8);
    println!("After union(3, 8): {:?}", qu);
    qu.union(6, 5);
    println!("After union(6, 5): {:?}", qu);
    qu.union(9, 4);
    println!("After union(9, 4): {:?}", qu);
    qu.union(2, 1);
    println!("After union(2, 1): {:?}", qu);
    println!("connected(5, 0): {}", qu.is_connected(5, 0));
    qu.union(5, 0);
    println!("After union(5, 0): {:?}", qu);
    qu.union(7, 2);
    println!("After union(7, 2): {:?}", qu);
    qu.union(6, 1);
    println!("After union(6, 1): {:?}", qu);
}

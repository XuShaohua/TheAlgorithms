// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashSet;

use system_design::snow_flake::SnowFlake;

fn main() {
    let mut sf = SnowFlake::new(1, 2).unwrap();
    let mut set = HashSet::new();
    let id_count: usize = 40000;
    for _i in 0..id_count {
        let id = sf.generate_id().unwrap();
        set.insert(id);
    }
    assert_eq!(set.len(), id_count);
    let mut list: Vec<u64> = set.into_iter().collect();
    list.sort_unstable();
    for id in &list {
        println!("id: {id}, 0b{id:0b}");
    }
}

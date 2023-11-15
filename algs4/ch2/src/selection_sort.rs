// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt;
use std::ops;

use crate::export_graph;

fn get_output_file(counter: u32) -> String {
    format!("/tmp/selection-sort-{}.png", counter)
}

pub fn selection_sort(list: &mut Vec<i32>, with_graph: bool) {
    let mut counter = 0;
    if with_graph {
        let filename = get_output_file(counter);
        counter += 1;
        export_graph(list, filename);
    }
    let n = list.len();
    for i in 0..n {
        let mut min = i;
        for j in (i + 1)..n {
            if list[j] < list[min] {
                min = j;
            }
        }
        if i != min {
            crate::exchange(list, i, min);
            if with_graph {
                let filename = get_output_file(counter);
                counter += 1;
                export_graph(list, filename);
            }
        }
    }

    if with_graph {
        let filename = get_output_file(counter);
        counter += 1;
        export_graph(list, filename);
    }
}

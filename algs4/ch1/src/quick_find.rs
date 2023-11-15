// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Defects:
/// - Union too expensive (N Array accesses)
/// - Trees are flat, but too expensive to keep them flat
#[derive(Debug)]
pub struct QuickFind {
    items: Vec<usize>,
}

impl QuickFind {
    pub fn new(n: usize) -> QuickFind {
        let mut items = vec![0; n];
        for i in 0..n {
            items[i] = i;
        }
        QuickFind { items }
    }

    /// Find is fast, O(1).
    pub fn is_connected(&self, p: usize, q: usize) -> bool {
        self.items[p] == self.items[q]
    }

    /// Find too expensive (N array accesses).
    pub fn union(&mut self, p: usize, q: usize) {
        let pid = self.items[p];
        let qid = self.items[q];
        for i in 0..self.items.len() {
            if self.items[i] == pid {
                self.items[i] = qid;
            }
        }
    }
}

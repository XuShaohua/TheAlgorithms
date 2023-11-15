// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::connectivity::Connectivity;
use crate::error::Error;
use crate::graph::Graph;

/// Defects:
/// - Tree can get tall. The worst case is a list.
/// - Find too expensive (could be N array accesses).
#[derive(Debug)]
pub struct QuickUnion {
    items: Vec<usize>,
    graph_idx: usize,
}

impl QuickUnion {
    pub fn new(n: usize) -> QuickUnion {
        let mut items = vec![0; n];
        for i in 0..n {
            items[i] = i;
        }
        QuickUnion {
            items,
            graph_idx: 0,
        }
    }

    fn root(&self, mut i: usize) -> usize {
        while i != self.items[i] {
            i = self.items[i];
        }
        return i;
    }

    fn do_generate_graph(&self) -> Graph {
        let mut graph = Graph::new();
        for (index, item) in self.items.iter().enumerate() {
            graph.add_node(index, *item);
        }
        graph
    }
}

impl Connectivity for QuickUnion {
    fn len(&self) -> usize {
        self.items.len()
    }

    fn is_connected(&self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    fn union(&mut self, p: usize, q: usize) {
        let pid = self.root(p);
        let qid = self.root(q);
        self.items[pid] = qid;
    }

    fn generate_graph(&mut self) -> Result<String, Error> {
        let filepath = format!("/tmp/quick-union_{}", self.graph_idx);
        self.do_generate_graph().quick_output(&filepath)?;
        self.graph_idx += 1;
        Ok(filepath)
    }
}

// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::connectivity::Connectivity;
use crate::error::Error;
use crate::graph::Graph;

/// Depths of any node x is at most `lgN`.
#[derive(Debug)]
pub struct WeightedQuickUnion {
    items: Vec<usize>,
    sizes: Vec<usize>,
    graph_idx: usize,
}

impl WeightedQuickUnion {
    pub fn new(n: usize) -> Self {
        let mut items = vec![0; n];
        let mut sizes = vec![0; n];
        for i in 0..n {
            items[i] = i;
            sizes[i] = 1;
        }
        Self {
            items,
            sizes,
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

impl Connectivity for WeightedQuickUnion {
    fn len(&self) -> usize {
        self.items.len()
    }

    fn is_connected(&self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    fn union(&mut self, p: usize, q: usize) {
        let pid = self.root(p);
        let qid = self.root(q);
        if pid == qid {
            return;
        }

        if self.sizes[pid] < self.sizes[qid] {
            self.items[pid] = qid;
            self.sizes[qid] += self.sizes[pid];
        } else {
            self.items[qid] = pid;
            self.sizes[pid] += self.sizes[qid];
        }
    }

    fn generate_graph(&mut self) -> Result<String, Error> {
        let filepath = format!("/tmp/weighted-quick-union_{}", self.graph_idx);
        self.do_generate_graph().quick_output(&filepath)?;
        self.graph_idx += 1;
        Ok(filepath)
    }
}

// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub struct ArenaList<T> {
    nodes: Vec<NodeInfo<T>>,
    first_free: usize,
    len: usize,
}

struct NodeInfo<T> {
    data: Option<T>,
    prev: Option<usize>,
    next: Option<usize>,
}

pub struct NodeRef<'a, T> {
    index: usize,
    owner: &'a ArenaList<T>,
}

impl<T> ArenaList<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            first_free: 0,
            len: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
            first_free: 0,
            len: 0,
        }
    }

    /// Concates |other| list.
    pub fn append(&mut self, other: &Self) {
        unimplemented!()
    }

    /// Add a new node to haed of list.
    pub fn push(&mut self, value: T) {
        unimplemented!()
    }

    /// Remove the head node from list and returns it.
    pub fn pop(&mut self) -> Option<T> {
        unimplemented!()
    }

    /// Get number of nodes in list.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn to_array(self) -> Vec<T> {
        unimplemented!()
    }
}

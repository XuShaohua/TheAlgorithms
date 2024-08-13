// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use std::ptr::NonNull;
use std::sync::Mutex;

pub struct LockCouplingLinkedList<T> {
    len: usize,
    head: NodePtr<T>,
}

type NodePtr<T> = Mutex<Option<NonNull<Node<T>>>>;

pub struct Node<T> {
    next: NodePtr<T>,
    value: T,
}

impl<T> LockCouplingLinkedList<T> {}

impl<T> Node<T> {
    #[must_use]
    fn new(value: T) -> Self {
        Self {
            next: Mutex::new(None),
            value,
        }
    }
}
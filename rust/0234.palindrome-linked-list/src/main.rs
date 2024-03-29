// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::ops::Deref;

/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[must_use]
    pub const fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    #[must_use]
    pub fn from_slice(slice: &[i32]) -> Option<Box<Self>> {
        let mut list = None;
        for x in slice.iter().rev() {
            list = Some(Box::new(ListNode {
                val: *x,
                next: list,
            }));
        }
        list
    }
}

pub trait Next {
    fn next(self) -> Self;

    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;
}

impl Next for Option<Box<ListNode>> {
    fn next(self) -> Self {
        match self {
            Some(boxed_self) => boxed_self.next,
            None => None,
        }
    }

    fn is_empty(&self) -> bool {
        self.is_none()
    }

    fn len(&self) -> usize {
        match self {
            Some(boxed_self) => boxed_self.deref().next.len() + 1,
            None => 0,
        }
    }
}

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut head = head;
    let len = head.len();
    if len == 0 {
        return false;
    }

    let mut cache = Vec::with_capacity(len);
    let mut index = 0;

    while let Some(head_ref) = head {
        cache.push(head_ref.val);
        head = head_ref.next;
        index += 1;
        if len / 2 == index {
            index -= 1;
            break;
        }
    }

    if len % 2 == 1 {
        head = head.next();
    }

    while let Some(head_ref) = head {
        if head_ref.val != cache[index] {
            return false;
        }
        if index == 0 {
            break;
        }
        index -= 1;
        head = head_ref.next;
    }

    true
}

fn check_solution() {
    let l1 = ListNode::from_slice(&[1, 2, 2, 1]);
    assert!(is_palindrome(l1));

    let l1 = ListNode::from_slice(&[1, 2]);
    assert!(!is_palindrome(l1));
}

fn main() {
    check_solution();
}

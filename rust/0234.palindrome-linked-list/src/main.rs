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

pub trait ListLinkTrait {
    fn next(self) -> Self;

    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;

    fn value(&self) -> Option<i32>;
}

impl ListLinkTrait for Option<Box<ListNode>> {
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

    fn value(&self) -> Option<i32> {
        if let Some(ref head) = self {
            Some(head.val)
        } else {
            None
        }
    }
}

pub type IsPalindromeFunc = fn(Option<Box<ListNode>>) -> bool;

/// Use vector as cache, iterator through linked list and compare values.
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

/// Two pointers
///
/// Reverse left half of list and compare with right part.
pub fn is_palindrome2(head: Option<Box<ListNode>>) -> bool {
    if head.is_none() {
        return false;
    }

    let len = head.len();

    // 1. Reverse left half of list.
    let mut head = head;
    let mut index = 0;
    let mut left_part = None;
    while len / 2 != index {
        left_part = Some(Box::new(ListNode {
            val: head.value().unwrap(),
            next: left_part,
        }));
        head = head.next();
        index += 1;
    }

    if len % 2 == 1 {
        head = head.next();
    }

    // 2. compare from middle
    while head.is_some() {
        if head.value() != left_part.value() {
            return false;
        }
        head = head.next();
        left_part = left_part.next();
    }

    true
}

fn check_solution(func: IsPalindromeFunc) {
    let l1 = ListNode::from_slice(&[1, 2, 2, 1]);
    assert!(func(l1));

    let l1 = ListNode::from_slice(&[1, 2]);
    assert!(!func(l1));

    let l1 = ListNode::from_slice(&[1, 2, 1]);
    assert!(func(l1));
}

fn main() {
    check_solution(is_palindrome);
    check_solution(is_palindrome2);
}

// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

//! Problem: [Add two numbers](https://leetcode.com/problems/add-two-numbers)

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: ListNodePtr,
}

pub type ListNodePtr = Option<Box<ListNode>>;

impl ListNode {
    #[must_use]
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    #[must_use]
    #[inline]
    pub fn new_ptr(val: i32) -> ListNodePtr {
        Some(Box::new(Self::new(val)))
    }

    pub fn push_head(head: ListNodePtr, val: i32) -> ListNodePtr {
        Some(Box::new(Self { val, next: head }))
    }

    pub fn push_tail(tail: &mut ListNodePtr, val: i32) -> ListNodePtr {
        debug_assert!(tail.next.is_none());
        let new_tail = Self::new_ptr(val);
        tail.next = new_tail;
        new_tail
    }

    #[must_use]
    pub fn from_slice(s: &[i32]) -> ListNodePtr {
        assert!(s.len() > 0);
        let mut list = None;
        for val in s.iter().rev() {
            list = Self::push_head(list, *val);
        }
        list
    }
}

fn solution1(l1: ListNodePtr, l2: ListNodePtr) -> ListNodePtr {
    let mut l3 = None;
    let div = 0;
    let rem = 0;
    loop {
        l1 = l1.unwrap();
        l2 = l2.unwrap();
        let val1 = l1.value;
        l1 = l1.next();
        let val2 = l2.value;
        l2 = l2.next();
        (div, rem) = (l1 + l2 + div).div_rem(10);
        l3 = ListNode::push_head(l3, rem);

        if l1.is_none() {
            l2.value += div;
            l3.next = l2;
            break;
        }
        if l2.is_none() {
            l1.value += div;
            l3.next = l1;
            break;
        }
    }
    l3
}

fn main() {
    let l1 = ListNode::from_slice(&[2, 4, 3]);
    let l2 = ListNode::from_slice(&[5, 6, 4]);
    let target = ListNode::from_slice(&[7, 0, 8]);
    assert_eq!(solution1(l1, l2), target);
}

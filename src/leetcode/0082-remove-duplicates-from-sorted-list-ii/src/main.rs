// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[must_use]
    pub const fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    pub fn from_slice(slice: &[i32]) -> Option<Box<Self>> {
        let mut list = None;
        for item in slice.iter().rev() {
            list = Self::cons(list, *item);
        }
        list
    }

    pub fn cons(list: Option<Box<Self>>, val: i32) -> Option<Box<Self>> {
        Some(Box::new(Self { val, next: list }))
    }
}

fn solution1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let Some(mut head) = head {
        let val = head.val;
        while let Some(next_node) = head.next {
            if next.val != val {
                break;
            }
            match next_node.next {
                Some(next_head) => ,
                None => 
            }
        }
        ListNode::cons(Some(head), val)
    } else {
        None
    }
}

fn main() {
    let list = ListNode::from_slice(&[1, 2, 3, 3, 4, 4, 5]);
    let result = solution1(list);
    println!("result: {result:?}");
    let expected_result = ListNode::from_slice(&[1, 2, 5]);
    assert_eq!(result, expected_result);

    let list = ListNode::from_slice(&[1, 1, 1, 2, 3]);
    let result = solution1(list);
    println!("result: {result:?}");
    let expected_result = ListNode::from_slice(&[2, 3]);
    assert_eq!(result, expected_result);
}

#[cfg(test)]
mod tests {
    use super::{solution1, ListNode};

    #[test]
    fn test_solution1() {
        let list = ListNode::from_slice(&[1, 2, 3, 3, 4, 4, 5]);
        let result = solution1(list);
        let expected_result = ListNode::from_slice(&[1, 2, 5]);
        assert_eq!(result, expected_result);

        let list = ListNode::from_slice(&[1, 1, 1, 2, 3]);
        let result = solution1(list);
        let expected_result = ListNode::from_slice(&[2, 3]);
        assert_eq!(result, expected_result);
    }
}
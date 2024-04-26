// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

//! Problem: [Add Two Numbers](https://leetcode.com/problems/add-two-numbers)

type NodeLink = Option<Box<ListNode>>;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ListNode {
    val: i32,
    next: NodeLink,
}

impl ListNode {
    #[must_use]
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    #[must_use]
    pub fn from_slice(slice: &[i32]) -> NodeLink {
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

fn add_two_numbers1(l1: NodeLink, l2: NodeLink) -> NodeLink {
    let mut l3 = Some(Box::new(ListNode::new(0)));
    let mut l1 = &l1;
    let mut l2 = &l2;
    let mut tail = l3.as_mut().unwrap();
    let mut carry = 0;

    loop {
        let mut sum = carry;
        if let Some(l1_box) = l1 {
            sum += l1_box.val;
            l1 = &l1_box.next;
        }
        if let Some(l2_box) = l2 {
            sum += l2_box.val;
            l2 = &l2_box.next;
        }
        carry = sum / 10;
        tail.val = sum % 10;

        if l1.is_none() && l2.is_none() {
            if carry != 0 {
                tail.next = Some(Box::new(ListNode::new(carry)));
            }
            break;
        }

        tail.next = Some(Box::new(ListNode::new(carry)));
        tail = tail.next.as_mut().unwrap();
    }

    l3
}

pub type SolutionFn = fn(NodeLink, NodeLink) -> NodeLink;

fn check_solution(func: SolutionFn) {
    let l1 = ListNode::from_slice(&[2, 4, 3]);
    let l2 = ListNode::from_slice(&[5, 6, 4]);
    assert_eq!(func(l1, l2), ListNode::from_slice(&[7, 0, 8]));

    let l1 = ListNode::from_slice(&[0]);
    let l2 = ListNode::from_slice(&[0]);
    assert_eq!(func(l1, l2), ListNode::from_slice(&[0]));

    let l1 = ListNode::from_slice(&[9, 9, 9, 9, 9, 9, 9]);
    let l2 = ListNode::from_slice(&[9, 9, 9, 9]);
    assert_eq!(
        func(l1, l2),
        ListNode::from_slice(&[8, 9, 9, 9, 0, 0, 0, 1])
    );
}

fn main() {
    check_solution(add_two_numbers1);
}

#[cfg(test)]
mod tests {
    use super::{add_two_numbers1, check_solution};

    #[test]
    fn test_solution() {
        check_solution(add_two_numbers1);
    }
}

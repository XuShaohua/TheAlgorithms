// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

//! Problem: [Add Two Numbers](https://leetcode.com/problems/add-two-numbers)

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

pub type ListNodeRef = Option<Box<ListNode>>;

pub fn unbox<T>(value: Box<T>) -> T {
    *value
}

impl ListNode {
    #[must_use]
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    /// Concate |other| list to this one.
    pub fn append(mut list: ListNodeRef, mut other: ListNodeRef) -> ListNodeRef {
        while let Some(other_box) = other {
            list = ListNode::push(list, other_box.val);
            other = other_box.next;
        }
        list
    }

    pub fn append_with_carrier(
        mut list: ListNodeRef,
        mut other: ListNodeRef,
        mut carrier: i32,
    ) -> ListNodeRef {
        while let Some(other_box) = other {
            let sum = other_box.val + carrier;
            let remainder = sum % 10;
            carrier = sum / 10;
            list = ListNode::push(list, remainder);
            other = other_box.next;
        }
        if carrier > 0 {
            list = ListNode::push(list, carrier);
        }
        list
    }

    /// Add an element to head of list.
    pub fn push(list: ListNodeRef, val: i32) -> ListNodeRef {
        Some(Box::new(ListNode { val, next: list }))
    }

    /// Reverse elements in list.
    pub fn reverse(mut list: ListNodeRef) -> ListNodeRef {
        let mut new_list = None;
        while let Some(list_box) = list {
            new_list = Self::push(new_list, list_box.val);
            list = list_box.next;
        }
        new_list
    }

    #[must_use]
    pub fn from_slice(slice: &[i32]) -> ListNodeRef {
        let mut list = None;
        for x in slice.iter().rev() {
            list = Self::push(list, *x);
        }
        list
    }
}

fn solution1(l1: ListNodeRef, l2: ListNodeRef) -> ListNodeRef {
    let mut result = None;
    let mut carrier = 0;
    let mut l1 = l1;
    let mut l2 = l2;

    loop {
        if l1.is_none() {
            result = ListNode::append_with_carrier(result, l2, carrier);
            break;
        }
        if l2.is_none() {
            result = ListNode::append_with_carrier(result, l1, carrier);
            break;
        }
        let l1_box = l1.unwrap();
        let l2_box = l2.unwrap();
        let sum = l1_box.val + l2_box.val + carrier;
        let remainder = sum % 10;
        carrier = sum / 10;
        result = ListNode::push(result, remainder);

        l1 = l1_box.next;
        l2 = l2_box.next;
    }

    ListNode::reverse(result)
}

fn main() {
    let l1 = ListNode::from_slice(&[2, 4, 3]);
    let l2 = ListNode::from_slice(&[5, 6, 4]);
    assert_eq!(solution1(l1, l2), ListNode::from_slice(&[7, 0, 8]));

    let l1 = ListNode::from_slice(&[0]);
    let l2 = ListNode::from_slice(&[0]);
    assert_eq!(solution1(l1, l2), ListNode::from_slice(&[0]));

    let l1 = ListNode::from_slice(&[9, 9, 9, 9, 9, 9, 9]);
    let l2 = ListNode::from_slice(&[9, 9, 9, 9]);
    assert_eq!(
        solution1(l1, l2),
        ListNode::from_slice(&[8, 9, 9, 9, 0, 0, 0, 1])
    );
}

#[cfg(test)]
mod tests {}

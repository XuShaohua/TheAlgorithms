// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[must_use]
    #[inline]
    pub const fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    #[must_use]
    pub fn from_slice(slice: &[i32]) -> Option<Box<Self>> {
        let mut list = None;
        for num in slice.iter().rev() {
            list = Some(Box::new(Self {
                val: *num,
                next: list,
            }))
        }
        list
    }
}

// 反转链表
pub fn add_two_numbers1(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    #[allow(clippy::question_mark)]
    fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut current = None;
        let mut next = head;
        while next.is_some() {
            let new_next = next.as_mut().unwrap().next.take();
            next.as_mut().unwrap().next = current;
            current = next;
            next = new_next;
        }
        current
    }

    let mut l1 = reverse(l1);
    let mut l2 = reverse(l2);
    let mut result = None;
    let mut carry: i32 = 0;

    while l1.is_some() || l2.is_some() {
        let mut sum: i32 = carry;
        if let Some(l1_ref) = l1 {
            sum += l1_ref.val;
            l1 = l1_ref.next;
        }
        if let Some(l2_ref) = l2 {
            sum += l2_ref.val;
            l2 = l2_ref.next;
        }
        let val: i32 = sum % 10;
        carry = sum / 10;
        result = Some(Box::new(ListNode { val, next: result }));
    }
    if carry != 0 {
        result = Some(Box::new(ListNode {
            val: carry,
            next: result,
        }));
    }

    result
}

pub type SolutionFn = fn(Option<Box<ListNode>>, Option<Box<ListNode>>) -> Option<Box<ListNode>>;

fn check_add_two_numbers(func: SolutionFn) {
    let l1 = ListNode::from_slice(&[7, 2, 4, 3]);
    let l2 = ListNode::from_slice(&[5, 6, 4]);
    let expected = ListNode::from_slice(&[7, 8, 0, 7]);
    assert_eq!(func(l1, l2), expected);

    let l1 = ListNode::from_slice(&[2, 4, 3]);
    let l2 = ListNode::from_slice(&[5, 6, 4]);
    let expected = ListNode::from_slice(&[8, 0, 7]);
    assert_eq!(func(l1, l2), expected);

    let l1 = ListNode::from_slice(&[0]);
    let l2 = ListNode::from_slice(&[0]);
    let expected = ListNode::from_slice(&[0]);
    assert_eq!(func(l1, l2), expected);
}

fn main() {
    check_add_two_numbers(add_two_numbers1);
}

#[cfg(test)]
mod tests {
    use super::{add_two_numbers1, check_add_two_numbers};

    #[test]
    fn test_add_two_numbers1() {
        check_add_two_numbers(add_two_numbers1);
    }
}

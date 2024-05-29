// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn from_slice(slice: &[i32]) -> Option<Box<Self>> {
        let mut list = None;
        for &val in slice.iter().rev() {
            list = Some(Box::new(Self { val, next: list }));
        }
        list
    }

    #[must_use]
    pub fn len(head: &Option<Box<Self>>) -> usize {
        let mut node_ref = head;
        let mut count: usize = 0;

        while node_ref.is_some() {
            node_ref = &node_ref.as_ref().unwrap().next;
            count += 1;
        }

        count
    }

    #[must_use]
    #[inline]
    pub fn is_empty(head: &Option<Box<Self>>) -> bool {
        head.is_none()
    }

    #[must_use]
    pub fn split(head: &mut Option<Box<Self>>, pos: usize) -> Option<Box<Self>> {
        debug_assert!(pos < Self::len(head));
        let mut node_ref = head;
        for _i in 0..pos {
            node_ref = &mut node_ref.as_mut().unwrap().next;
        }
        node_ref.as_mut().unwrap().next.take()
    }

    #[must_use]
    #[allow(clippy::question_mark)]
    pub fn reverse(head: Option<Box<Self>>) -> Option<Box<Self>> {
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

    pub fn cross_merge(mut l1: &mut Option<Box<Self>>, mut l2: Option<Box<Self>>) {
        while l1.is_some() && l2.is_some() {
            let l2_next = l2.as_mut().unwrap().next.take();
            l2.as_mut().unwrap().next = l1.as_mut().unwrap().next.take();
            l1.as_mut().unwrap().next = l2;
            l2 = l2_next;
            l1 = &mut l1.as_mut().unwrap().next;
            l1 = &mut l1.as_mut().unwrap().next;
        }
    }

    #[allow(dead_code)]
    fn debug_print(mut head: &Option<Box<ListNode>>) {
        print!("head: [");
        while head.is_some() {
            let val: i32 = head.as_ref().unwrap().val;
            print!("{val}, ");
            head = &head.as_ref().unwrap().next;
        }
        println!("]");
    }
}

pub fn reorder_list1(head: &mut Option<Box<ListNode>>) {
    if head.is_none() {
        return;
    }

    let len: usize = ListNode::len(head);
    let half = len / 2;
    let tail = ListNode::split(head, half);
    let tail = ListNode::reverse(tail);
    ListNode::cross_merge(head, tail);
}

pub type SolutionFn = fn(&mut Option<Box<ListNode>>);

fn check_reorder_list(func: SolutionFn) {
    let mut head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
    let expected = ListNode::from_slice(&[1, 5, 2, 4, 3]);
    func(&mut head);
    ListNode::debug_print(&head);
    assert_eq!(head, expected);

    let mut head = ListNode::from_slice(&[1, 2, 3, 4]);
    let expected = ListNode::from_slice(&[1, 4, 2, 3]);
    func(&mut head);
    ListNode::debug_print(&head);
    assert_eq!(head, expected);
}

fn main() {
    check_reorder_list(reorder_list1);
}

#[cfg(test)]
mod tests {
    use super::{check_reorder_list, reorder_list1};

    #[test]
    fn test_reorder_list1() {
        check_reorder_list(reorder_list1);
    }
}

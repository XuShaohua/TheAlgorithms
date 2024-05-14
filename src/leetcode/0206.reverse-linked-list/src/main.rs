// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[must_use]
    #[inline]
    pub const fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    #[must_use]
    pub fn from_slice(slice: &[i32]) -> Option<Box<ListNode>> {
        let mut list = None;
        for &val in slice.iter().rev() {
            list = Some(Box::new(ListNode { val, next: list }));
        }
        list
    }

    pub fn debug_print(head: &Option<Box<ListNode>>) {
        print!("head: [");
        let mut node_ref = head;
        while node_ref.is_some() {
            let val: i32 = node_ref.as_ref().unwrap().val;
            print!("{val}, ");
            node_ref = &node_ref.as_ref().unwrap().next;
        }
        println!("]");
    }
}

pub fn reverse_list1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }

    let mut current = None;
    let mut next = head;
    while next.is_some() {
        let new_next = next.as_mut()?.next.take();
        next.as_mut()?.next = current;
        current = next;
        next = new_next;
    }

    current
}

pub type SolutionFn = fn(Option<Box<ListNode>>) -> Option<Box<ListNode>>;

fn check_solution(func: SolutionFn) {
    let list = ListNode::from_slice(&[1, 2, 3, 4, 5]);
    let expected = ListNode::from_slice(&[5, 4, 3, 2, 1]);
    let result = func(list);
    ListNode::debug_print(&result);
    assert_eq!(result, expected);

    let list = ListNode::from_slice(&[1, 2]);
    let expected = ListNode::from_slice(&[2, 1]);
    let result = func(list);
    ListNode::debug_print(&result);
    assert_eq!(result, expected);
}

fn main() {
    check_solution(reverse_list1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, reverse_list1};

    #[test]
    fn test_reverse_list1() {
        check_solution(reverse_list1);
    }
}

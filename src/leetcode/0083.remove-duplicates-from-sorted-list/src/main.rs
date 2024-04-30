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
            list = Some(Box::new(Self {
                val: *item,
                next: list,
            }));
        }
        list
    }

    fn debug_print(head: &Option<Box<ListNode>>) -> Option<()> {
        let mut node_ref = head;
        print!("Head: [");
        while node_ref.is_some() {
            print!("{}, ", node_ref.as_ref()?.val);
            node_ref = &node_ref.as_ref()?.next;
        }
        println!("]");

        Some(())
    }
}

fn delete_duplicates1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut node_ref = &mut head;
    while node_ref.is_some() {
        let next_node_ref = &node_ref.as_ref()?.next;
        if next_node_ref.is_some() && next_node_ref.as_ref()?.val == node_ref.as_ref()?.val {
            let next_node = node_ref.as_mut()?.next.take();
            node_ref.as_mut()?.next = next_node?.next;
        } else {
            node_ref = &mut node_ref.as_mut()?.next;
        }
    }
    head
}

fn delete_duplicates2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;

    if head.is_none() {
        return head;
    }
    let mut current_ref: &mut ListNode = head.as_mut()?;
    while let Some(next_ref) = current_ref.next.as_mut() {
        if current_ref.val == next_ref.val {
            current_ref.next = next_ref.next.take();
        } else {
            current_ref = current_ref.next.as_mut()?;
        }
    }

    head
}

type SolutionFn = fn(Option<Box<ListNode>>) -> Option<Box<ListNode>>;

fn check_solution(func: SolutionFn) {
    let list = ListNode::from_slice(&[1, 1, 2]);
    let result = func(list);
    //println!("result: {result:?}");
    ListNode::debug_print(&result);
    let expected_result = ListNode::from_slice(&[1, 2]);
    assert_eq!(result, expected_result);

    let list = ListNode::from_slice(&[1, 1, 2, 3, 3]);
    let result = func(list);
    //println!("result: {result:?}");
    ListNode::debug_print(&result);
    let expected_result = ListNode::from_slice(&[1, 2, 3]);
    assert_eq!(result, expected_result);
}

fn main() {
    check_solution(delete_duplicates1);
    check_solution(delete_duplicates2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, delete_duplicates1, delete_duplicates2};

    #[test]
    fn test_solution1() {
        check_solution(delete_duplicates1);
    }

    #[test]
    fn test_solution2() {
        check_solution(delete_duplicates2);
    }
}

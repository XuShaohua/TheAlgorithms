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
        let mut node_ref = head;
        print!("head: [");
        while node_ref.is_some() {
            let val: i32 = node_ref.as_ref().unwrap().val;
            print!("{val}, ");
            node_ref = &node_ref.as_ref().unwrap().next;
        }
        println!("]");
    }
}

pub fn remove_elements1(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    // 加入 dummy head, 方便处理头部节点.
    let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
    let mut current_ref = &mut dummy_head;

    while current_ref.is_some() {
        let next_ref = &mut current_ref.as_mut().unwrap().next;

        if next_ref.is_some() && next_ref.as_ref().unwrap().val == val {
            // 移除该节点
            let next_next_node = next_ref.as_mut().unwrap().next.take();
            current_ref.as_mut().unwrap().next = next_next_node;
        } else {
            // 移到下一个节点
            current_ref = &mut current_ref.as_mut().unwrap().next;
        }
    }

    // 返回真正的链表头
    dummy_head.as_mut()?.next.take()
}

pub type SolutionFn = fn(Option<Box<ListNode>>, i32) -> Option<Box<ListNode>>;

fn check_solution(func: SolutionFn) {
    let head = ListNode::from_slice(&[1, 2, 6, 3, 4, 5, 6]);
    let expected = ListNode::from_slice(&[1, 2, 3, 4, 5]);
    let result = func(head, 6);
    ListNode::debug_print(&result);
    assert_eq!(result, expected);

    let head = ListNode::from_slice(&[]);
    let expected = ListNode::from_slice(&[]);
    let result = func(head, 1);
    ListNode::debug_print(&result);
    assert_eq!(result, expected);

    let head = ListNode::from_slice(&[7, 7, 7, 7]);
    let expected = ListNode::from_slice(&[]);
    let result = func(head, 7);
    ListNode::debug_print(&result);
    assert_eq!(result, expected);

    let head = ListNode::from_slice(&[1]);
    let expected = ListNode::from_slice(&[1]);
    let result = func(head, 2);
    ListNode::debug_print(&result);
    assert_eq!(result, expected);
}

fn main() {
    check_solution(remove_elements1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, remove_elements1};

    #[test]
    fn test_remove_elements1() {
        check_solution(remove_elements1);
    }
}

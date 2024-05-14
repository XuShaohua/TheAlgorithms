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

pub fn reverse_between1(
    head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    // 反转链表里面实现的, 只反转前面几个元素.
    fn reverse_list(head: Option<Box<ListNode>>, len: i32) -> Option<Box<ListNode>> {
        debug_assert!(len > 0);
        let mut current_node = None;
        let mut next_node = head;

        for _ in 0..len {
            let new_next_node = next_node.as_mut()?.next.take();
            // 反转一个节点
            next_node.as_mut()?.next = current_node;
            current_node = next_node;
            next_node = new_next_node;
        }

        // 遍历反转后的链表, 重新连接尾部元素.
        let mut node_ref = &mut current_node;
        for _ in 0..(len - 1) {
            node_ref = &mut node_ref.as_mut().unwrap().next;
        }
        node_ref.as_mut().unwrap().next = next_node;

        // 最后返回新的 list head.
        current_node
    }

    debug_assert!(left > 0 && left <= right);
    // 处理边角情况
    if left == right {
        return head;
    }

    // 添加一个dummy header, 方便反转第一个节点.
    let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
    let mut current_ref = &mut dummy_head;
    let mut index: i32 = 0;

    while current_ref.is_some() {
        index += 1;
        // 开始反转
        if index == left {
            // 首先把 current->next 取出来
            let rev_head = current_ref.as_mut()?.next.take();
            let new_head = reverse_list(rev_head, right - left + 1);
            // 终止反转
            // 进行新链表的拼装.
            current_ref.as_mut()?.next = new_head;
        }
        current_ref = &mut current_ref.as_mut()?.next;
    }

    dummy_head.as_mut()?.next.take()
}

pub type SolutionFn = fn(Option<Box<ListNode>>, i32, i32) -> Option<Box<ListNode>>;

fn check_solution(func: SolutionFn) {
    let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
    ListNode::debug_print(&head);
    let expected = ListNode::from_slice(&[1, 4, 3, 2, 5]);
    let result = func(head, 2, 4);
    ListNode::debug_print(&result);
    assert_eq!(result, expected);

    let head = ListNode::from_slice(&[5]);
    let expected = ListNode::from_slice(&[5]);
    let result = func(head, 1, 1);
    ListNode::debug_print(&result);
    assert_eq!(result, expected);
}

fn main() {
    check_solution(reverse_between1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, reverse_between1};

    #[test]
    fn test_reverse_between1() {
        check_solution(reverse_between1);
    }
}

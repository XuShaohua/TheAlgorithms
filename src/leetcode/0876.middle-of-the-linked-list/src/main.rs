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

// 计数法
// 先计算链表长度; 然后再遍历一次链表, 找到一半节点的位置, 并返回
pub fn middle_node1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn list_len(head: &Option<Box<ListNode>>) -> usize {
        let mut node_ref = head;
        let mut count = 0;

        while node_ref.is_some() {
            node_ref = &node_ref.as_ref().unwrap().next;
            count += 1;
        }
        count
    }

    let len: usize = list_len(&head);
    let half: usize = len / 2;

    let mut node = head;
    for _ in 0..half {
        //node = node->next;
        node = node.as_mut().unwrap().next.take();
    }

    node
}

// 快慢型双指针
// 遍历链表
pub fn middle_node2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // 处理特殊情况.
    if head.as_ref().unwrap().next.is_none() {
        return head;
    }
    // TODO(Shaohua):

    //let mut slow = &head;
    let fast = &head;

    while fast.is_some() {
        //fast = fast->next->next;
        //slow = slow->next;
    }

    todo!()
}

pub type SolutionFn = fn(Option<Box<ListNode>>) -> Option<Box<ListNode>>;

fn check_solution(func: SolutionFn) {
    let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
    ListNode::debug_print(&head);
    let expected = ListNode::from_slice(&[3, 4, 5]);
    let result = func(head);
    ListNode::debug_print(&result);
    assert_eq!(result, expected);

    let head = ListNode::from_slice(&[1, 2, 3, 4, 5, 6]);
    let expected = ListNode::from_slice(&[4, 5, 6]);
    let result = func(head);
    ListNode::debug_print(&result);
    assert_eq!(result, expected);
}

fn main() {
    check_solution(middle_node1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, middle_node1};

    #[test]
    fn test_middle_node1() {
        check_solution(middle_node1);
    }
}

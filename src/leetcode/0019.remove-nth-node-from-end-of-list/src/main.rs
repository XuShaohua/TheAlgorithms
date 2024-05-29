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
}

pub fn remove_nth_from_end1(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    #[must_use]
    fn list_len(head: &Option<Box<ListNode>>) -> usize {
        let mut node_ref = head;
        let mut count = 0;
        while node_ref.is_some() {
            node_ref = &node_ref.as_ref().unwrap().next;
            count += 1;
        }
        count
    }

    // 使用 dummy head, 方便移除头部节点
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let len: usize = list_len(&dummy);
    let mut node_ref = &mut dummy;
    let count: usize = len - n as usize - 1;

    for _i in 0..count {
        node_ref = &mut node_ref.as_mut().unwrap().next;
    }
    let mut to_be_removed = node_ref.as_mut().unwrap().next.take();
    node_ref.as_mut().unwrap().next = to_be_removed.as_mut().unwrap().next.take();

    dummy.as_mut().unwrap().next.take()
}

pub type SolutionFn = fn(Option<Box<ListNode>>, i32) -> Option<Box<ListNode>>;

fn check_remove_nth_from_end(func: SolutionFn) {
    let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
    let expected = ListNode::from_slice(&[1, 2, 3, 5]);
    assert_eq!(func(head, 2), expected);

    let head = ListNode::from_slice(&[1]);
    let expected = ListNode::from_slice(&[]);
    assert_eq!(func(head, 1), expected);

    let head = ListNode::from_slice(&[1, 2]);
    let expected = ListNode::from_slice(&[1]);
    assert_eq!(func(head, 1), expected);
}

fn main() {
    check_remove_nth_from_end(remove_nth_from_end1);
}

#[cfg(test)]
mod tests {
    use super::{check_remove_nth_from_end, remove_nth_from_end1};

    #[test]
    fn test_remove_nth_from_end1() {
        check_remove_nth_from_end(remove_nth_from_end1);
    }
}

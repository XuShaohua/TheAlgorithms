// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#![allow(clippy::ptr_arg)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
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
        for &val in slice.iter().rev() {
            list = Some(Box::new(Self { val, next: list }));
        }
        list
    }

    pub fn debug_print(head: &Option<Box<Self>>) {
        print!("head: [");
        let mut node_ref = head;
        while node_ref.is_some() {
            print!("{}, ", node_ref.as_ref().unwrap().val);
            node_ref = &node_ref.as_ref().unwrap().next;
        }
        println!("]");
    }

    pub fn debug_print2(head: &Option<Box<Self>>) -> Option<Box<Self>> {
        print!("head: [");
        if head.is_some() {
            let mut node_ref: &ListNode = head.as_ref()?;
            print!("{}, ", node_ref.val);
            while let Some(next_ref) = node_ref.next.as_ref() {
                print!("{}, ", next_ref.val);
                node_ref = node_ref.next.as_ref()?;
            }
        }
        println!("]");
        None
    }
}

// 双指针
pub fn delete_duplicates1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }

    // dummy_head 的下一个节点是 head.
    // 当 head 及其后面的节点是重复数值时, 这样设计就可以简化处理.
    // 因为最终返回的, 是 dummy_head->next
    let mut dummy_head = Some(Box::new(ListNode {
        val: i32::MIN,
        next: head,
    }));

    // 这里, 不再创建 prev 节点, current 节点就当作 prev 节点使用.
    // 这样为了方便移除后面的节点.
    let mut current: &mut ListNode = dummy_head.as_mut()?;

    // 当 current->next 为 None 是, 说明遍历完成.
    while let Some(next_ref) = current.next.as_mut() {
        // current->next.val == current->next->next.val
        if let Some(next_next_ref) = next_ref.next.as_mut() {
            if next_ref.val == next_next_ref.val {
                // 移除所有重复节点
                let mut temp: Option<Box<ListNode>> = current.next.take();
                // temp.val == temp->next.val
                while temp.is_some()
                    && temp.as_ref()?.next.is_some()
                    && temp.as_ref()?.val == temp.as_ref()?.next.as_ref()?.val
                {
                    // 这里 temp 指向的当前节点就被 drop 了.
                    // temp = temp->next;
                    temp = temp.as_mut()?.next.take();
                }
                // 将 current 的下一个节点指向 temp 的下一个节点,
                // 在下个循环时, 将开始检查它的值.
                // current->next = temp->next;
                current.next = temp.as_mut()?.next.take();
            } else {
                // 跳到下一个节点
                // current = current->next;
                current = current.next.as_mut()?;
            }
        } else {
            // 当 current->next->next 为 None 是, 说明 current->next.val 不重复,
            // 遍历也完成了.
            break;
        }
    }

    dummy_head.as_mut()?.next.take()
}

pub type SolutionFn = fn(Option<Box<ListNode>>) -> Option<Box<ListNode>>;

fn check_solution(func: SolutionFn) {
    let list = ListNode::from_slice(&[1, 2, 3, 3, 4, 4, 5]);
    let result = func(list);
    ListNode::debug_print(&result);
    //println!("result: {result:?}");
    let expected_result = ListNode::from_slice(&[1, 2, 5]);
    assert_eq!(result, expected_result);

    let list = ListNode::from_slice(&[1, 1, 1, 2, 3]);
    let result = func(list);
    ListNode::debug_print(&result);
    //println!("result: {result:?}");
    let expected_result = ListNode::from_slice(&[2, 3]);
    assert_eq!(result, expected_result);
}

fn main() {
    check_solution(delete_duplicates1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, delete_duplicates1};

    #[test]
    fn test_solution1() {
        check_solution(delete_duplicates1);
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::mem;

pub type ListNodeLink = Option<Box<ListNode>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListNode {
    pub val: i32,
    pub next: ListNodeLink,
}

impl ListNode {
    #[must_use]
    #[inline]
    pub const fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    #[must_use]
    #[inline]
    pub fn push_front(next: ListNodeLink, val: i32) -> ListNodeLink {
        Some(Box::new(Self { val, next }))
    }

    pub fn from_slice(slice: &[i32]) -> ListNodeLink {
        let mut root = None;
        for &num in slice.iter().rev() {
            root = Self::push_front(root, num);
        }
        root
    }

    fn debug_print(link: &ListNodeLink) -> Option<()> {
        let mut curr = link;
        while curr.is_some() {
            print!("{}, ", curr.as_ref()?.val);
            curr = &curr.as_ref()?.next;
        }
        println!();
        Some(())
    }
}

pub fn merge_two_lists1(list1: ListNodeLink, list2: ListNodeLink) -> ListNodeLink {
    let mut list1 = list1;
    let mut list2 = list2;
    let mut ref1 = &mut list1;

    while list2.is_some() {
        if ref1.is_none() || list2.as_ref()?.val < ref1.as_ref()?.val {
            mem::swap(ref1, &mut list2);
        }
        ref1 = &mut ref1.as_mut()?.next;
    }

    //ListNode::debug_print(&list1);
    list1
}
pub fn merge_two_lists2(list1: ListNodeLink, list2: ListNodeLink) -> ListNodeLink {
    let mut head = None;
    let mut head_ref = &mut head;
    let mut list1 = list1;
    let mut list2 = list2;

    while list1.is_some() && list2.is_some() {
        let ref1 = &mut list1;
        let ref2 = &mut list2;
        let tmp_ref = if ref1.as_ref()?.val < ref2.as_ref()?.val {
            ref1
        } else {
            ref2
        };

        mem::swap(head_ref, tmp_ref);
        mem::swap(tmp_ref, &mut head_ref.as_mut()?.next);
        head_ref = &mut head_ref.as_mut()?.next;
    }

    mem::swap(
        head_ref,
        if list1.is_some() {
            &mut list1
        } else {
            &mut list2
        },
    );

    ListNode::debug_print(&head);

    head
}

pub type SolutionFn = fn(list1: ListNodeLink, list2: ListNodeLink) -> ListNodeLink;

fn check_solution(func: SolutionFn) {
    let list1 = ListNode::from_slice(&[1, 2, 4]);
    let list2 = ListNode::from_slice(&[1, 3, 4]);
    let out = func(list1, list2);
    let exp = ListNode::from_slice(&[1, 1, 2, 3, 4, 4]);
    assert_eq!(out, exp);

    let list1 = ListNode::from_slice(&[]);
    let list2 = ListNode::from_slice(&[]);
    let out = func(list1, list2);
    let exp = ListNode::from_slice(&[]);
    assert_eq!(out, exp);

    let list1 = ListNode::from_slice(&[]);
    let list2 = ListNode::from_slice(&[0]);
    let out = func(list1, list2);
    let exp = ListNode::from_slice(&[0]);
    assert_eq!(out, exp);
}

fn main() {
    check_solution(merge_two_lists1);
    check_solution(merge_two_lists2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, merge_two_lists1, merge_two_lists2};

    #[test]
    fn test_solution1() {
        check_solution(merge_two_lists1);
    }

    #[test]
    fn test_solution2() {
        check_solution(merge_two_lists2);
    }
}

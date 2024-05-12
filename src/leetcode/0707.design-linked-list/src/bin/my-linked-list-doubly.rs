// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::ptr::NonNull;

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub prev: Option<NodeLink>,
    pub next: Option<NodeLink>,
}

type NodeLink = NonNull<ListNode>;

#[derive(Debug)]
pub struct MyLinkedList {
    len: usize,
    head: Option<NodeLink>,
    tail: Option<NodeLink>,
}

impl Default for MyLinkedList {
    fn default() -> Self {
        Self::new()
    }
}

impl MyLinkedList {
    /// Initializes the MyLinkedList object.
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self {
            len: 0,
            head: None,
            tail: None,
        }
    }

    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Get the value of the indexth node in the linked list.
    ///
    /// If the index is invalid, return -1.
    pub fn get(&self, index: i32) -> i32 {
        assert!(index >= 0);
        if index < 0 || (index as usize) > self.len {
            // Invalid index.
            return -1;
        }
        if index == 0 {
            if let Some(node) = self.head {
                unsafe {
                    return node.as_ref().val;
                }
            } else {
                return -1;
            }
        }

        if (index as usize) == self.len - 1 {
            if let Some(node) = self.tail {
                unsafe {
                    return node.as_ref().val;
                }
            } else {
                return -1;
            }
        }

        let mut current = self.head;
        let mut count = index;
        while let Some(node) = current {
            if count <= 0 {
                break;
            }
            current = unsafe { node.as_ref().next };
            count -= 1;
        }
        if let Some(node) = current {
            unsafe { node.as_ref().val }
        } else {
            -1
        }
    }

    /// Add a node of value val before the first element of the linked list.
    ///
    /// After the insertion, the new node will be the first node of the linked list.
    pub fn add_at_head(&mut self, val: i32) {
        let new_head = NonNull::new(Box::into_raw(Box::new(ListNode {
            val,
            prev: None,
            next: self.head,
        })));
        if let Some(mut old_head) = self.head {
            unsafe {
                old_head.as_mut().prev = new_head;
            }
        } else {
            self.tail = new_head;
        }

        self.head = new_head;
        self.len += 1;
    }

    /// Append a node of value val as the last element of the linked list.
    pub fn add_at_tail(&mut self, val: i32) {
        let new_tail = NonNull::new(Box::into_raw(Box::new(ListNode {
            val,
            prev: self.tail,
            next: None,
        })));
        if let Some(mut old_tail) = self.tail {
            unsafe {
                old_tail.as_mut().next = new_tail;
            }
        } else {
            self.head = new_tail;
        }
        self.tail = new_tail;
        self.len += 1;
    }

    /// Add a node of value val before the indexth node in the linked list.
    ///
    /// If index equals the length of the linked list, the node will be appended
    /// to the end of the linked list. If index is greater than the length,
    /// the node will not be inserted.
    pub fn add_at_index(&mut self, index: i32, val: i32) {
        assert!(index >= 0);
        if index < 0 || (index as usize) > self.len {
            // Invalid index.
            return;
        }
        if index == 0 {
            self.add_at_head(val);
            return;
        }
        if (index as usize) == self.len {
            self.add_at_tail(val);
            return;
        }

        let mut current = self.head;
        let mut count = index;
        while let Some(node) = current {
            if count <= 0 {
                break;
            }
            current = unsafe { node.as_ref().next };
            count -= 1;
        }

        if let Some(mut next_node) = current {
            let mut new_node = ListNode {
                val,
                prev: None,
                next: Some(next_node),
            };
            if let Some(prev) = unsafe { next_node.as_ref().prev } {
                new_node.prev = Some(prev);
            }
            let new_node = NonNull::new(Box::into_raw(Box::new(new_node)));
            if let Some(mut prev) = unsafe { next_node.as_mut().prev } {
                unsafe {
                    prev.as_mut().next = new_node;
                }
            }
            unsafe {
                next_node.as_mut().prev = new_node;
            }
        }

        self.len += 1;
    }

    /// Delete the indexth node in the linked list, if the index is valid.
    pub fn delete_at_index(&mut self, index: i32) {
        if index < 0 {
            return;
        }
        let index = index as usize;
        if index >= self.len {
            // Invalid index
            return;
        }

        if index == 0 {
            self.delete_at_head();
            return;
        }
        if index == self.len - 1 {
            self.delete_at_tail();
            return;
        }

        let mut count = index;
        let mut current = self.head;
        while let Some(node) = current {
            if count == 0 {
                break;
            }
            current = unsafe { node.as_ref().next };
            count -= 1;
        }
        if let Some(node) = current {
            if let Some(mut prev) = unsafe { node.as_ref().prev } {
                unsafe {
                    prev.as_mut().next = node.as_ref().next;
                }
            }
            if let Some(mut next) = unsafe { node.as_ref().next } {
                unsafe {
                    next.as_mut().prev = node.as_ref().prev;
                }
            }

            // Drop current node
            let _ = unsafe { Box::from_raw(node.as_ptr()) };
        }

        self.len -= 1;
    }

    pub fn delete_at_head(&mut self) {
        if let Some(old_head) = self.head {
            self.head = unsafe { old_head.as_ref().next };
            if let Some(mut new_head) = self.head {
                unsafe {
                    new_head.as_mut().prev = None;
                }
            }
            // Drop old head
            let _ = unsafe { Box::from_raw(old_head.as_ptr()) };
        } else {
            self.head = None;
            self.tail = None;
        }

        if self.len > 0 {
            self.len -= 1;
        }
    }

    pub fn delete_at_tail(&mut self) {
        if let Some(old_tail) = self.tail {
            self.tail = unsafe { old_tail.as_ref().prev };
            if let Some(mut new_tail) = self.tail {
                unsafe {
                    new_tail.as_mut().next = None;
                }
            }
            // Drop old tail
            let _ = unsafe { Box::from_raw(old_tail.as_ptr()) };
        } else {
            self.head = None;
            self.tail = None;
        }
        if self.len > 0 {
            self.len -= 1;
        }
    }

    fn debug_print(&self) {
        print!("len: {}, items: ", self.len);
        let mut current = self.head;
        while let Some(node) = current {
            let val = unsafe { node.as_ref().val };
            print!("{val},");
            current = unsafe { node.as_ref().next };
        }
        println!();
    }
}

fn check_solution() {
    let mut obj = MyLinkedList::new();
    obj.debug_print();
    assert_eq!(obj.get(0), -1);

    println!("=== Add 1 at head ===");
    obj.add_at_head(1);
    obj.debug_print();
    assert_eq!(obj.get(0), 1);

    println!("=== Add 3 at tail ===");
    obj.add_at_tail(3);
    obj.debug_print();
    assert_eq!(obj.get(0), 1);
    assert_eq!(obj.get(1), 3);

    println!("=== Add 2 at index 1 ===");
    obj.add_at_index(1, 2);
    obj.debug_print();
    assert_eq!(obj.get(0), 1);
    assert_eq!(obj.get(1), 2);
    assert_eq!(obj.get(2), 3);

    println!("=== Delete at index 2 ===");
    obj.delete_at_index(2);
    obj.debug_print();
    assert_eq!(obj.get(0), 1);
    assert_eq!(obj.get(1), 2);
    assert_eq!(obj.get(2), -1);
}

fn main() {
    check_solution();
}

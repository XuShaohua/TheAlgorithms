// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub struct ListNode {
    val: i32,
    next: Option<Box<Self>>,
}

pub struct MyLinkedList {
    len: usize,
    head: Option<Box<ListNode>>,
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
        Self { len: 0, head: None }
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
        if let Some(val) = self.get_detail(index) {
            val
        } else {
            -1
        }
    }

    #[must_use]
    fn get_detail(&self, index: i32) -> Option<i32> {
        assert!(index >= 0);
        let index = index as usize;

        if index >= self.len {
            return None;
        }

        let mut node_ref = &self.head;
        for _i in 0..index {
            node_ref = &node_ref.as_ref()?.next;
        }

        Some(node_ref.as_ref()?.val)
    }

    /// Add a node of value val before the first element of the linked list.
    ///
    /// After the insertion, the new node will be the first node of the linked list.
    pub fn add_at_head(&mut self, val: i32) {
        let old_head = self.head.take();
        let new_node = Box::new(ListNode {
            val,
            next: old_head,
        });
        self.len += 1;
        self.head.replace(new_node);
    }

    /// Append a node of value val as the last element of the linked list.
    pub fn add_at_tail(&mut self, val: i32) {
        // Add to head.
        if self.len == 0 {
            self.add_at_head(val);
            return;
        }

        let new_node = Box::new(ListNode { val, next: None });
        let mut node_ref = &mut self.head;
        for _i in 0..(self.len - 1) {
            node_ref = &mut node_ref.as_mut().unwrap().next;
        }
        node_ref.as_mut().unwrap().next.replace(new_node);
        self.len += 1;
    }

    /// Add a node of value val before the indexth node in the linked list.
    ///
    /// If index equals the length of the linked list, the node will be appended
    /// to the end of the linked list. If index is greater than the length,
    /// the node will not be inserted.
    pub fn add_at_index(&mut self, index: i32, val: i32) {
        assert!(index >= 0);
        let index = index as usize;

        if index == 0 {
            self.add_at_head(val);
            return;
        }
        // Append to end of list.
        if index == self.len {
            self.add_at_tail(val);
            return;
        }
        // Index is too large, ignore it.
        if index > self.len {
            return;
        }

        let mut node_ref = &mut self.head;
        for _i in 0..(index - 1) {
            node_ref = &mut node_ref.as_mut().unwrap().next;
        }
        let old_next = node_ref.as_mut().unwrap().next.take();
        let new_node = Box::new(ListNode {
            val,
            next: old_next,
        });
        node_ref.as_mut().unwrap().next.replace(new_node);
        self.len += 1;
    }

    /// Delete the indexth node in the linked list, if the index is valid.
    pub fn delete_at_index(&mut self, index: i32) {
        if index < 0 {
            return;
        }
        if self.is_empty() {
            return;
        }
        if index == 0 {
            self.delete_at_head();
            return;
        }
        let index = index as usize;
        if (index + 1) == self.len {
            self.delete_at_tail();
            return;
        }
        if index >= self.len {
            return;
        }

        let mut node_ref = &mut self.head;
        for _i in 0..(index - 1) {
            node_ref = &mut node_ref.as_mut().unwrap().next;
        }
        let next_node = node_ref.as_mut().unwrap().next.take();
        node_ref.as_mut().unwrap().next = next_node.unwrap().next;
        self.len -= 1;
    }

    pub fn delete_at_head(&mut self) {
        if self.is_empty() {
            return;
        }

        let mut old_head = self.head.take();
        let new_head = old_head.as_mut().unwrap().next.take();
        self.head = new_head;
        self.len -= 1;
    }

    pub fn delete_at_tail(&mut self) {
        println!("delete at tail");
        if self.is_empty() {
            return;
        }

        if self.len == 1 {
            self.delete_at_head();
            return;
        }

        let mut node_ref = &mut self.head;
        for _i in 0..(self.len - 1) {
            node_ref = &mut node_ref.as_mut().unwrap().next;
        }
        // Drop tail node
        let _tail_node = node_ref.as_mut().unwrap().next.take();
        self.len -= 1;
    }

    fn debug_print(&self) {
        let mut node_ref = &self.head;
        print!("len: {}, items: ", self.len);
        while node_ref.is_some() {
            print!("{},", node_ref.as_ref().unwrap().val);
            node_ref = &node_ref.as_ref().unwrap().next;
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

#[cfg(test)]
mod tests {
    use super::check_solution;

    #[test]
    fn test_solution() {
        check_solution();
    }
}

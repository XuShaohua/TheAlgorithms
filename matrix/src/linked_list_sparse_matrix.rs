// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(dead_code)]

use std::fmt;
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::traits::IsZero;

/// Each element node in the linked list.
pub struct Node<T: IsZero> {
    /// Row number of element.
    pub row: usize,
    /// Column number of element.
    pub column: usize,
    /// Value of element.
    pub value: T,
    /// Pointer to next node.
    prev: NodePtr<T>,
    /// Pointer to next node.
    next: NodePtr<T>,
}

type NodePtr<T> = Option<NonNull<Node<T>>>;

/// Store sparse matrix with linked list.
#[allow(clippy::linkedlist)]
pub struct LinkedListSparseMatrix<T: IsZero> {
    len: usize,
    head: NodePtr<T>,
    tail: NodePtr<T>,
}

pub struct Iter<'a, T: 'a + IsZero> {
    head: NodePtr<T>,
    len: usize,
    _marker: PhantomData<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a + IsZero> {
    head: NodePtr<T>,
    len: usize,
    _marker: PhantomData<&'a mut Node<T>>,
}

impl<T: IsZero> LinkedListSparseMatrix<T> {
    #[must_use]
    pub fn construct<I, I2>(sparse_matrix: I) -> Self
    where
        I: IntoIterator<Item = I2>,
        I2: IntoIterator<Item = T>,
    {
        let mut head: NodePtr<T> = None;
        let mut tail: NodePtr<T> = None;
        let mut len: usize = 0;

        for (row, row_list) in sparse_matrix.into_iter().enumerate() {
            for (column, value) in row_list.into_iter().enumerate() {
                if value.is_not_zero() {
                    let mut node: NonNull<Node<T>> = Node::new_ptr(row, column, value);
                    len += 1;
                    if let Some(mut tail_ref) = tail {
                        unsafe {
                            node.as_mut().prev = tail;
                            tail_ref.as_mut().next = Some(node);
                        }
                    } else {
                        head = Some(node);
                    }
                    tail = Some(node);
                }
            }
        }
        Self { len, head, tail }
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

    #[must_use]
    pub fn value(&self, row: usize, column: usize) -> Option<T> {
        for node in self {
            if node.row == row && node.column == column {
                return Some(node.value);
            }
            if node.row > row {
                return None;
            }
        }
        None
    }

    #[must_use]
    pub fn value_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        for node in self.iter_mut() {
            if node.row == row && node.column == column {
                return Some(&mut node.value);
            }
            if node.row > row {
                return None;
            }
        }
        None
    }

    /// Add an element to the beginning of list.
    pub fn push_front(&mut self, row: usize, column: usize, value: T) {
        let node_ptr = Node::new_ptr(row, column, value);
        self.push_front_node(node_ptr);
    }

    /// Remove the first node in the list.
    pub fn pop_front(&mut self) -> Option<(usize, usize, T)> {
        self.pop_front_node().map(Node::into_inner)
    }

    pub fn push_back(&mut self, row: usize, column: usize, value: T) {
        let node_ptr = Node::new_ptr(row, column, value);
        self.push_back_node(node_ptr);
    }

    pub fn pop_back(&mut self) -> Option<(usize, usize, T)> {
        self.pop_back_node().map(Node::into_inner)
    }

    /// If found old node at (row, column), returns old value; otherwise returns None.
    #[must_use]
    pub fn add_element(&mut self, row: usize, column: usize, value: T) -> Option<T> {
        let len = self.len;
        for (index, node) in self.iter_mut().enumerate() {
            if node.row == row && node.column == column {
                let old_value = node.value;
                node.value = value;
                return Some(old_value);
            }
            if (node.row == row && node.column > column) || node.row > row {
                if index == 0 {
                    self.push_front(row, column, value);
                } else if index == len - 1 {
                    self.push_back(row, column, value);
                } else {
                    // Insert new node to previous of current node.
                    let new_node: NonNull<Node<T>> = Node::new_ptr(row, column, value);
                    unsafe { Self::insert_before(node, new_node) };
                    self.len += 1;
                }

                return None;
            }
        }

        // Add new node to end of list.
        self.push_back(row, column, value);
        None
    }

    /// If found node at (row, column), returns value of that node; otherwise returns None.
    #[must_use]
    pub fn remove_element(&mut self, row: usize, column: usize) -> Option<T> {
        let len = self.len;

        for (index, node) in self.iter_mut().enumerate() {
            if node.row == row && node.column == column {
                let value = node.value;
                if index == 0 {
                    self.pop_front();
                } else if index == len - 1 {
                    self.pop_back();
                } else {
                    unsafe {
                        Self::remove_node(node);
                    }
                    self.len -= 1;
                }
                return Some(value);
            }
            if (node.row == row && node.column > column) || node.row > row {
                // Node not found.
                return None;
            }
        }
        None
    }

    // Iterators
    #[must_use]
    pub const fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            len: self.len,
            _marker: PhantomData,
        }
    }

    #[allow(clippy::needless_pass_by_ref_mut)]
    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            head: self.head,
            len: self.len,
            _marker: PhantomData,
        }
    }
}

impl<T: IsZero> LinkedListSparseMatrix<T> {
    /// Insert `new_node` before `current_node`.
    unsafe fn insert_before(current_node_ref: &mut Node<T>, mut new_node: NonNull<Node<T>>) {
        if let Some(mut prev_node) = current_node_ref.prev {
            new_node.as_mut().prev = Some(prev_node);
            let current_node = prev_node.as_mut().next.take().unwrap();
            prev_node.as_mut().next = Some(new_node);

            new_node.as_mut().next = Some(current_node);
            current_node_ref.prev = Some(new_node);
        }
    }

    /// Insert `new_node` after `current_node`.
    unsafe fn insert_after(mut current_node: NonNull<Node<T>>, mut new_node: NonNull<Node<T>>) {
        if let Some(mut next_node) = current_node.as_mut().next {
            new_node.as_mut().next = Some(next_node);
            next_node.as_mut().prev = Some(new_node);
        }
        new_node.as_mut().prev = Some(current_node);
        current_node.as_mut().next = Some(new_node);
    }

    /// Remove `node` from list.
    ///
    /// Both prev and next node are valid.
    unsafe fn remove_node(node: &mut Node<T>) {
        let mut prev_node = node.prev.unwrap();
        let mut next_node = node.next.unwrap();
        prev_node.as_mut().next = Some(next_node);
        next_node.as_mut().prev = Some(prev_node);

        node.prev = None;
        node.next = None;
    }

    fn push_front_node(&mut self, node_ptr: NonNull<Node<T>>) {
        unsafe {
            (*node_ptr.as_ptr()).next = self.head;
            (*node_ptr.as_ptr()).prev = None;
        }
        let node = Some(node_ptr);

        match self.head {
            Some(head) => unsafe { (*head.as_ptr()).prev = node },
            None => self.tail = node,
        }

        self.head = node;
        self.len += 1;
    }

    fn push_back_node(&mut self, node_ptr: NonNull<Node<T>>) {
        unsafe {
            (*node_ptr.as_ptr()).next = None;
            (*node_ptr.as_ptr()).prev = self.tail;
        }
        let node = Some(node_ptr);

        match self.tail {
            Some(tail) => unsafe { (*tail.as_ptr()).next = node },
            None => self.head = node,
        }

        self.tail = node;
        self.len += 1;
    }

    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|old_head| {
            let old_head = unsafe { Node::from_ptr(old_head) };
            self.head = old_head.next;

            match self.head {
                Some(head) => unsafe { (*head.as_ptr()).prev = None },
                None => self.tail = None,
            }

            self.len -= 1;
            old_head
        })
    }

    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        self.tail.map(|old_tail| {
            let old_tail = unsafe { Node::from_ptr(old_tail) };
            self.tail = old_tail.prev;

            match self.tail {
                Some(tail) => unsafe { (*tail.as_ptr()).next = None },
                None => self.head = None,
            }

            self.len -= 1;
            old_tail
        })
    }
}

impl<T: fmt::Debug + IsZero> fmt::Debug for LinkedListSparseMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<'a, T: IsZero> IntoIterator for &'a LinkedListSparseMatrix<T> {
    type Item = &'a Node<T>;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: IsZero> IntoIterator for &'a mut LinkedListSparseMatrix<T> {
    type Item = &'a mut Node<T>;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T: IsZero> Iterator for Iter<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                let node: &Node<T> = node.as_ref();
                self.len -= 1;
                self.head = node.next;
                node
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<T: IsZero> ExactSizeIterator for Iter<'_, T> {}

impl<'a, T: IsZero> Iterator for IterMut<'a, T> {
    type Item = &'a mut Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|mut node| unsafe {
                let node: &mut Node<T> = node.as_mut();
                self.len -= 1;
                self.head = node.next;
                node
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<T: IsZero> ExactSizeIterator for IterMut<'_, T> {}

impl<T: IsZero> Node<T> {
    #[must_use]
    #[inline]
    const fn new(row: usize, column: usize, value: T) -> Self {
        Self {
            row,
            column,
            value,
            prev: None,
            next: None,
        }
    }

    #[must_use]
    #[inline]
    fn new_ptr(row: usize, column: usize, value: T) -> NonNull<Self> {
        let node = Box::new(Self::new(row, column, value));
        NonNull::from(Box::leak(node))
    }

    #[must_use]
    #[inline]
    #[allow(clippy::unnecessary_box_returns)]
    unsafe fn from_ptr(ptr: NonNull<Self>) -> Box<Self> {
        Box::from_raw(ptr.as_ptr())
    }

    #[must_use]
    #[inline]
    #[allow(clippy::boxed_local)]
    fn into_inner(self: Box<Self>) -> (usize, usize, T) {
        (self.row, self.column, self.value)
    }
}

#[allow(clippy::missing_fields_in_debug)]
impl<T: fmt::Debug + IsZero> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("row", &self.row)
            .field("column", &self.column)
            .field("value", &self.value)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedListSparseMatrix;

    #[test]
    fn test_array() {
        const MATRIX: [[i32; 5]; 4] = [
            [0, 0, 3, 0, 4],
            [0, 0, 5, 7, 0],
            [0, 0, 0, 0, 0],
            [0, 2, 6, 0, 0],
        ];
        let sm = LinkedListSparseMatrix::construct(MATRIX);
        println!("sm: {sm:?}");
        assert_eq!(sm.len(), 6);
    }

    #[test]
    fn test_vec_of_array() {
        let matrix = vec![
            [0, 0, 3, 0, 4],
            [0, 0, 5, 7, 0],
            [0, 0, 0, 0, 0],
            [0, 2, 6, 0, 0],
        ];
        let sm = LinkedListSparseMatrix::construct(matrix);
        println!("sm: {sm:?}");
        assert_eq!(sm.len(), 6);
    }

    #[test]
    fn test_vec_of_vec() {
        let matrix = vec![
            vec![0, 0, 3, 0, 4],
            vec![0, 0, 5, 7, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 2, 6, 0, 0],
        ];
        let sm = LinkedListSparseMatrix::construct(matrix);
        println!("sm: {sm:?}");
        assert_eq!(sm.len(), 6);
    }

    #[test]
    fn test_value() {
        const MATRIX: [[i32; 5]; 4] = [
            [0, 0, 3, 0, 4],
            [0, 0, 5, 7, 0],
            [0, 0, 0, 0, 0],
            [0, 2, 6, 0, 0],
        ];
        let sm = LinkedListSparseMatrix::construct(MATRIX);
        assert_eq!(sm.value(0, 2), Some(3));
        assert_eq!(sm.value(1, 3), Some(7));
        assert_eq!(sm.value(1, 4), None);
    }

    #[test]
    fn test_value_mut() {
        const MATRIX: [[i32; 5]; 4] = [
            [0, 0, 3, 0, 4],
            [0, 0, 5, 7, 0],
            [0, 0, 0, 0, 0],
            [0, 2, 6, 0, 0],
        ];
        let mut sm = LinkedListSparseMatrix::construct(MATRIX);
        let ret = sm.value_mut(0, 2);
        assert!(ret.is_some());
        if let Some(value) = ret {
            *value *= 2;
        }
    }

    #[test]
    fn test_add_element() {
        const MATRIX: [[i32; 5]; 4] = [
            [0, 0, 3, 0, 4],
            [0, 0, 5, 7, 0],
            [0, 0, 0, 0, 0],
            [0, 2, 6, 0, 0],
        ];
        let mut sm = LinkedListSparseMatrix::construct(MATRIX);
        let ret = sm.add_element(1, 0, 1);
        assert!(ret.is_none());
        assert_eq!(sm.len(), 7);
        assert_eq!(sm.value(1, 0), Some(1));
    }

    #[test]
    fn test_remove_element() {
        const MATRIX: [[i32; 5]; 4] = [
            [0, 0, 3, 0, 4],
            [0, 0, 5, 7, 0],
            [0, 0, 0, 0, 0],
            [0, 2, 6, 0, 0],
        ];
        let mut sm = LinkedListSparseMatrix::construct(MATRIX);
        let ret = sm.remove_element(1, 0);
        assert!(ret.is_none());
        let ret = sm.remove_element(3, 2);
        assert_eq!(ret, Some(6));
        assert_eq!(sm.len(), 5);
    }
}

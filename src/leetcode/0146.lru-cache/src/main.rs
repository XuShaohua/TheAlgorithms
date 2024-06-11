// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

pub struct LRUCache {
    capacity: usize,
    // key -> index-in-list
    map: HashMap<i32, usize>,
    list: LinkedList,
}

/// 双链表
#[derive(Debug, Default)]
struct LinkedList {
    vec: Vec<ListNode>,
    front: usize,
    back: usize,
}

#[derive(Debug, Default, Clone)]
struct ListNode {
    key: i32,
    value: i32,
    prev: usize,
    next: usize,
}

impl LinkedList {
    pub fn new(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
            front: usize::MAX,
            back: usize::MAX,
        }
    }

    fn is_valid_index(index: usize) -> bool {
        index != usize::MAX
    }

    pub fn set_value(&mut self, index: usize, key: i32, value: i32) {
        debug_assert!(index < self.vec.capacity());
        let node = &mut self.vec[index];
        node.key = key;
        node.value = value;
    }

    // 返回的是旧的列表尾部元素
    pub fn push_back(&mut self, key: i32, value: i32) -> (Option<i32>, usize) {
        println!("self: {self:?}");
        if self.vec.len() == self.vec.capacity() {
            // 链表已经满了, 先把链表头部元素移除

            let last_front_index = self.front;
            assert!(Self::is_valid_index(last_front_index));

            // 更新链表头部节点.
            // 重置链表头部的左侧节点为空.
            if self.vec.len() > 1 {
                self.front = self.vec[last_front_index].next;
                self.vec[self.front].prev = usize::MAX;
            }

            // 更新链表的旧尾部节点的右侧链表
            self.vec[self.back].next = last_front_index;

            let last_front_node = &mut self.vec[last_front_index];
            // 更新节点的值.
            let last_front_key = last_front_node.key;
            last_front_node.key = key;
            last_front_node.value = value;
            last_front_node.prev = self.back;
            last_front_node.next = usize::MAX;

            // 更新链表的尾部节点索引.
            self.back = last_front_index;

            return (Some(last_front_key), last_front_index);
        }

        let new_node = ListNode {
            key,
            value,
            prev: self.back,
            next: usize::MAX,
        };

        self.vec.push(new_node);
        let new_index = self.vec.len() - 1;

        // 如果有必要, 就更新链表头部索引
        if self.vec.len() == 1 {
            self.front = new_index;
        }

        // 更新链表头部索引
        if Self::is_valid_index(self.back) {
            let last_back = &mut self.vec[self.back];
            last_back.next = new_index;
        }
        self.back = new_index;

        (None, self.back)
    }

    // 将节点调整为链表的尾部节点
    pub fn move_to_back(&mut self, index: usize) {
        debug_assert!(index < self.vec.len());
        debug_assert!(Self::is_valid_index(self.front));
        debug_assert!(Self::is_valid_index(self.back));
        // 如果当前节点就是尾部节点, 就什么都不做.
        if self.back == index {
            return;
        }

        if self.front == index {
            // 更新链表的头部节点索引
            debug_assert!(self.vec[index].prev == usize::MAX);
            self.front = self.vec[index].next;
            // 重置链表的头部节点的左侧索引.
            self.vec[self.front].prev = usize::MAX;
        } else {
            // 更新前一个节点和后一个节点
            let curr = &self.vec[index];
            let old_prev = curr.prev;
            let old_next = curr.next;
            self.vec[old_prev].next = old_next;
            self.vec[old_next].prev = old_prev;
        }

        // 更新旧的尾部节点
        self.vec[self.back].next = index;
        // 把当前节点移到尾部
        let curr = &mut self.vec[index];
        curr.prev = self.back;
        curr.next = usize::MAX;
        self.back = index;
    }

    #[must_use]
    pub fn back(&self) -> (i32, i32) {
        if self.vec.is_empty() {
            (i32::MIN, i32::MIN)
        } else {
            let item = &self.vec[self.back];
            (item.key, item.value)
        }
    }

    fn debug_print(&self) {
        println!("debug print(), list: {:?}", self.vec);
        if self.vec.is_empty() {
            println!("List is empty");
            return;
        }

        let mut node_index: usize = self.back;
        while Self::is_valid_index(node_index) {
            let item = &self.vec[node_index];
            print!("({}, {}) -> ", item.key, item.value);
            node_index = item.prev;
        }
        println!();
    }
}

impl LRUCache {
    /// Initialize the LRU cache with positive size capacity.
    pub fn new(capacity: i32) -> Self {
        debug_assert!(capacity >= 1);
        let capacity = capacity as usize;
        Self {
            capacity,
            map: HashMap::new(),
            list: LinkedList::new(capacity),
        }
    }

    /// Return the value of the key if the key exists, otherwise return -1.
    #[must_use]
    pub fn get(&mut self, key: i32) -> i32 {
        // 如果元素存在, 将把它调整到链表的尾部
        if let Some(&index) = self.map.get(&key) {
            self.list.move_to_back(index);
            // 返回元素的值
            self.list.back().1
        } else {
            -1
        }
    }

    /// Update the value of the key if the key exists.
    ///
    /// Otherwise, add the key-value pair to the cache.
    ///
    /// If the number of keys exceeds the capacity from this operation, evict the least recently used key.
    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&index) = self.map.get(&key) {
            // 如果元素已经存在链表中, 就把它移到链表尾部,
            // 并更新旧的尾部元素的索引信息.
            self.list.move_to_back(index);
            self.list.set_value(index, key, value);
        } else {
            // 如果元素不存在, 就把它加到链表中:
            // - 如果链表已满, 先移除链表的表头节点
            // - 如果链表未满, 只需要它新的节点追加到链表上即可
            match self.list.push_back(key, value) {
                (Some(old_key), new_index) => {
                    self.map.remove(&old_key);
                    self.map.insert(key, new_index);
                }
                (None, new_index) => {
                    self.map.insert(key, new_index);
                }
            }
        }
    }

    #[must_use]
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    fn debug_print(&self) {
        println!("map: {:?}", self.map);
        self.list.debug_print();
        println!("=====");
    }
}

fn check_solution() {
    let mut cache = LRUCache::new(2);
    cache.debug_print();
    cache.put(1, 1); // cache is {1=1}
    cache.debug_print();
    cache.put(2, 2); // cache is {1=1, 2=2}
    cache.debug_print();
    assert_eq!(cache.get(1), 1); // return 1
    cache.debug_print();
    cache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
    cache.debug_print();
    assert_eq!(cache.get(2), -1); // returns -1 (not found)
    cache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
    cache.debug_print();
    assert_eq!(cache.get(1), -1); // return -1 (not found)
    assert_eq!(cache.get(3), 3); // return 3
    assert_eq!(cache.get(4), 4); // return 4

    let mut cache = LRUCache::new(1);
    cache.put(1, 1); // cache is {1=1}
    cache.debug_print();
    cache.put(2, 2);
    assert_eq!(cache.get(6), -1);
    assert_eq!(cache.get(8), -1);
    cache.put(12, 1);
    assert_eq!(cache.get(2), -1);
    cache.put(15, 11);
    cache.put(5, 2);
    cache.put(1, 15);
    cache.put(4, 2);
    assert_eq!(cache.get(5), -1);
    cache.put(15, 15);
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

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(dead_code)]

pub fn heap_sort(list: &mut [i32]) {
    let mut max_heap = MaxHeap::from_slice(list);

    // 建立初始堆.
    // 从最后一个非叶节点开始, 进行下移操作.
    let len = max_heap.len();
    let parent = (len - 1) / 2;
    for i in (0..=parent).rev() {
        max_heap.shift_down(i, len);
    }

    for i in (0..len).rev() {
        // 交换根节点与当前堆的最后一个节点
        max_heap.heap.swap(0, i);

        // 从根节点开始, 进行下移操作
        max_heap.shift_down(0, i);
    }

    // 将排序后的结果更新到 list.
    list.swap_with_slice(&mut max_heap.heap);
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MaxHeap {
    heap: Vec<i32>,
}

impl Default for MaxHeap {
    fn default() -> Self {
        Self::new()
    }
}

impl MaxHeap {
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self { heap: Vec::new() }
    }

    pub fn from_slice(slice: &[i32]) -> Self {
        Self {
            heap: slice.to_vec(),
        }
    }

    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    #[must_use]
    #[inline]
    pub fn peak(&self) -> Option<i32> {
        self.heap.first().copied()
    }

    pub fn push(&mut self, val: i32) {
        // 先将新的元素插入到数组尾部.
        self.heap.push(val);

        // 然后将该元素进行上移 (shift up).
        self.shift_up(self.heap.len() - 1);
    }

    #[must_use]
    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        // 将堆顶的元素交换到数组的尾部, 并弹出它.
        let len = self.len();
        self.heap.swap(0, len - 1);
        let last = self.heap.pop();

        // 然后调整堆顶的位置, 让它保持大顶堆的特性.
        self.shift_down(0, len);

        last
    }

    // 将该节点与父节点进行比较, 如果比父节点大, 就让它跟父节点交换,
    // 一直重复这个过程, 直到找到合适的位置.
    //
    // 时间复杂度是 O(log(n)), 是树的高度.
    fn shift_up(&mut self, mut pos: usize) {
        if pos == 0 {
            return;
        }

        let mut parent_pos = (pos - 1) / 2;
        while self.heap[parent_pos] < self.heap[pos] {
            if parent_pos == 0 {
                break;
            }
            pos = parent_pos;
            parent_pos = (pos - 1) / 2;
        }
    }

    // 将该节点与子节点进行比较, 如果比子节点大, 就让它跟较大的那个子节点交换,
    // 一直重复这个过 程, 直到找到合适的位置.
    //
    // 时间复杂度是 O(log(n)), 是树的高度.
    fn shift_down(&mut self, mut pos: usize, len: usize) {
        while pos * 2 + 1 < len {
            let left = pos * 2 + 1;
            let right = pos * 2 + 2;

            let larger = if right >= len {
                // 右子树不存在, 它超出了数组的范围.
                left
            } else if self.heap[left] >= self.heap[right] {
                // 左侧子树较大
                left
            } else {
                // 右侧子树较大
                right
            };

            // 当前节点与较大的子节点进行比较, 如果比它小就进行交换.
            if self.heap[pos] < self.heap[larger] {
                self.heap.swap(pos, larger);
                pos = larger;
            } else {
                // 当前节点比最大的子节点还大
                break;
            }
        }
    }
}

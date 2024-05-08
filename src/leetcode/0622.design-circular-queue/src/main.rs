// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[derive(Debug, Clone)]
pub struct MyCircularQueue {
    size: usize,
    // front 指向头部元素的上一个元素
    front: usize,
    // back 指向尾部元素的位置
    back: usize,
    data: Vec<i32>,
}

impl MyCircularQueue {
    /// Initializes the object with the size of the queue to be k.
    #[must_use]
    #[inline]
    pub fn new(k: i32) -> Self {
        assert!(k > 0);
        // 多分配一个元素, 这个位置用于标记队列是否已满, 它并不存储数据.
        let size = k as usize + 1;
        Self {
            size,
            front: 0,
            back: 0,
            data: vec![0; size],
        }
    }

    /// Inserts an element into the circular queue.
    ///
    /// Return true if the operation is successful.
    #[must_use]
    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.back = (self.back + 1) % self.size;
            self.data[self.back] = value;
            true
        }
    }

    #[must_use]
    pub fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.front = (self.front + 1) % self.size;
            true
        }
    }

    ///  Gets the front item from the queue.
    ///
    ///  If the queue is empty, return -1.
    #[must_use]
    pub fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            let index = (self.front + 1) % self.size;
            self.data[index]
        }
    }

    ///  Gets the last item from the queue.
    ///
    ///  If the queue is empty, return -1.
    #[must_use]
    pub fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.back]
        }
    }

    /// Checks whether the circular queue is empty or not.
    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        // front 和 back 指向同一个元素, 表示队列为空.
        self.front == self.back
    }

    /// Checks whether the circular queue is full or not.
    #[must_use]
    #[inline]
    pub const fn is_full(&self) -> bool {
        // back 与 front 之间留下一个元素, 表示队列已满.
        (self.back + 1) % self.size == self.front
    }
}

fn check_solution() {
    let mut obj = MyCircularQueue::new(3);
    assert!(obj.en_queue(1));
    assert!(obj.en_queue(2));
    assert!(obj.en_queue(3));
    assert!(obj.de_queue());
    assert_eq!(obj.front(), 2);
    assert_eq!(obj.rear(), 3);
    assert!(!obj.is_empty());
    assert!(!obj.is_full());
}

fn main() {
    check_solution();
}

#[cfg(test)]
mod tests {
    use super::check_solution;

    #[test]
    fn test_circular_queue() {
        check_solution();
    }
}

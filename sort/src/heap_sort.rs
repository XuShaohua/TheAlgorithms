// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn heap_sort<T: PartialOrd>(list: &mut [T]) {
    let max_heap = list;

    // 建立初始堆.
    // 从最后一个非叶节点开始, 进行下移操作.
    let len = max_heap.len();
    let parent = (len - 1) / 2;
    for i in (0..=parent).rev() {
        shift_down(max_heap, i, len);
    }

    // 每个循环, 将最大元素下沉到数组尾部, 每次循环, 数组长度减少1,
    // 直到最后只剩下 1 个元素.
    for i in (0..len).rev() {
        // 交换根节点与当前堆的最后一个节点, 把最大的元素放到数组尾部.
        max_heap.swap(0, i);

        // 重新调整堆顶, 从根节点开始, 进行下移操作.
        // 这里, 忽略了刚刚的最大的那个元素.
        shift_down(max_heap, 0, i);
    }
}

// 将该节点与子节点进行比较, 如果比子节点大, 就让它跟较大的那个子节点交换,
// 一直重复这个过 程, 直到找到合适的位置.
//
// 时间复杂度是 O(log(n)), 是树的高度.
fn shift_down<T: PartialOrd>(heap: &mut [T], mut pos: usize, len: usize) {
    while pos * 2 + 1 < len {
        let left = pos * 2 + 1;
        let right = pos * 2 + 2;

        let larger = if right >= len {
            // 右子树不存在, 它超出了数组的范围.
            left
        } else if heap[left] >= heap[right] {
            // 左侧子树较大
            left
        } else {
            // 右侧子树较大
            right
        };

        // 当前节点与较大的子节点进行比较, 如果比它小就进行交换.
        if heap[pos] < heap[larger] {
            heap.swap(pos, larger);
            pos = larger;
        } else {
            // 当前节点比最大的子节点还大
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::heap_sort;

    #[test]
    fn test_heap_sort() {
        let mut list = [0, 5, 3, 2, 2];
        heap_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        heap_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        heap_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        heap_sort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }
}

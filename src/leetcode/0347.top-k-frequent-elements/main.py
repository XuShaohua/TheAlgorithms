#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import heapq

class Solution:
    def topKFrequent(self, nums: list[int], k: int) -> list[int]:
        # 首先统计数值的频率
        word_count = {}
        for num in nums:
            word_count[num] = word_count.get(num, 0) + 1
        # 构造最大堆, 堆中的元素是 (频率, 数值)
        pq = [(count, value) for (value, count) in word_count.items()]
        heapq.heapify(pq)
        # 得到最大堆的 top-k
        lst = heapq.nlargest(k, pq)
        # 提取 top-k 中的数值
        return [value for (_count, value) in lst]

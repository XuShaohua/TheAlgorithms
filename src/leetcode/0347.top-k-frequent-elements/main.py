#!/usr/bin/env python3

import heapq

class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        # 首先统计数值的频率
        word_count = {}
        for num in nums:
            word_count[num] = word_count.get(num, 0) + 1
        # 构造最大堆, 堆中的元素是 (-频率, 数值)
        pq = [(-count, value) for (value, count) in word_count.items()]
        heapq.heaprify(lst)
        # 得到最大堆的 top-k
        lst = heapq.nsmallest(lst, k)
        # 提取 top-k 中的数值
        return [value for (_count, value) in lst)]

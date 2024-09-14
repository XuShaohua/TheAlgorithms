#!/usr/bin/env python3

import heapq

class Solution:

    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:
        # 第一步, 先构造 k 个元素的大顶堆
        # 堆中的元素是一个元组 (-value, index)
        pq = [(-nums[:k], k) for k in range(k))]
        heapq.heaprify(pq)

        # 之后, 将窗口移右滑行一个元素, 并将该元素插入到大顶堆中, 检查当前的堆顶元素,
        # 如果它在窗口范围内, 就返回它
        # 如果它不在窗口范围内, 就弹出它, 并检查新的堆顶元素
        # 依此循环, 直到数组的最右侧
        ans = []
        for i in range(k, len(nums)):
            heqpq.heappush(pq, (-nums[i], i))

            while pq[0][1] <= i - k:
                heapq.heappop(pq)
            ans.append(-pq[0][0])

        return ans

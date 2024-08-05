# 队列的分类

根据队列可以容纳的元素个数不同, 可以被分为:

- 静态队列, 或者固定队列: 即队列在初始化时就指定它的容量, 队列中的元素个数不同超过该容量, 否则就出队 (enqueue) 失败
- 动态队列: 即队列中的元素个数不受限制

根据其结构不同, 队列可以分成几种类型:

- 简单队列 simple queue: 从一端入队 (enqueue), 而从另一端出队 (dequeue)
- 双端队列 double-ended queue(deque): 左右两端都可以入队出队
    - 限制入队队列 input-restricted queue: 元素可以从两端出队, 但只能从一端入队
    - 限制出队队列 output-restricted queue: 元素可以从两端入队, 但只能从一端出队
- 环形队列 circular queue: 又称为环状缓冲区, 整个队列的队首与队尾相连, 元素只从队列的头部出队, 从队列的尾部入队
- 优先级队列 priority queue: 队列中的元素按照某个规则升序或者降序依次排列

因为 [双端队列](../deque/index.md) 和 [优先级队列](../priority-queue/index.md) 比较复杂,
在后面有单独的章节介绍它们, 本章内容不再提及.
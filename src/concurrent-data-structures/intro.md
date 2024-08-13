# 简介

并发数据结构 Concurrent data structures, CDS 有三个关键的主题:

- 安全 Safety, 满足多线程并发的规范
    - 顺序规则 sequential specification, 像一个队列一样按顺序操作
    - 同步 synchronization
- 可扩展性 Scalability, 随着处理器核心数的增多, 性能更好
    - 理想情况下, 线性递增
    - 实际情况, 超过 16 个线程之后, 会退化成次线性递增 (sublinear scaling)
- 有进度 Progress, 保证操作过程向前推进
    - lock freedom: 至少有一个进度向前推进
    - wait freedom: 所有的进度都向前推进

## 安全性 Safety

使用锁或者其它同步原语 (primitive synchronization) 来保护并发数据结构.

- 使用全局锁来保护顺序数据结构
- 使用自定义的同步协议来保护数据结构

## 可扩展性 Scalability

- 减少锁保护的作用域
    - 读写锁 read-write locking
    - hand-over-hand locking
    - lock coupling
- 避免写数据以便减少无效的缓存
    - 乐观锁
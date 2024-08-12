# 链表的基本操作

常用的链表操作比较多.

构造函数:

- new(), 创建一个新的链表, 不包含任何节点

元素访问:

- front(), 访问第一个元素
- back(), 访问最后一个元素

链表容量:

- len(), 返回节点个数
- is_empty(), 链表是否为空

修改链表:

- clear(), 移除链表中的所有节点, 移除之后, `len()` 函数返回 0
- insert(pos, value), 在给定的特定位置插入新的节点
- insert_range(pos, iter), 在给定的特定位置插入一系列的节点
- erase(pos), 在给定的特定位置移除节点
- erase_range(first, last), 移除特定范围内的所有节点
- push_back(), 在链表尾部追加新的节点
- pop_back(), 移除链表尾部的节点
- append(iter), 在链表尾部追加一系列的节点
- push_front(), 在链表头部加入新的节点
- pop_front(), 移除链表头部的节点
- prepend(iter), 在链表头部加入一系列的节点
- resize(new_size), 调整链表中节点的个数, 如果需要追加新的节点, 就使用默认值
- resize_with(new_size, new_value), 调整链表中节点的个数, 如果需要追加新的节点, 就使用 `new_value`

链表操作:

- merge(), 合并两个链表
- splice(), 将节点从一个链表转移到另一个链表
- remove(), 从链表中移除特定值的节点
- remove_if(), 从链表中移除满足特定条件的节点
- reverse(), 将链表中的节点反转
- unique(), 从链表中移除有相同值的相邻的节点
- sort(), 对链表中的节点进行排序, 排序相关的函数放在了后面排序算法章节
- sort_with(), 依照相应的条件函数对链表中的节点进行排序

实现的 traits:

- Debug
- Clone
- PartialEq
- Eq
- Hash
- Drop

迭代器:

- iter(), 返回一个迭代器
- iter_mut(), 返回一个迭代器, 可以通过它修改链表中节点的值
- into_iter()
- DoubleEndedIterator, 对于双链表, 返回的迭代器需要实现双向迭代

## 插入 Insertion

在链表中插入一个新的节点, 分好几种情况:

- 在链表的头部插入节点
- 在链表的尾部插入节点
- 在给定的索引位置插入节点
- 在给定的节点后面插入节点
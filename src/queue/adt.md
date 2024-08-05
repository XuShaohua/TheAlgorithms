# 队列的基本操作

队列的基本接口包括:

- `fn new() -> Self`, 创建一个动态队列, 其容量不受限制
- `fn new(capacity) -> Self`, 创建一个静态队列, 初始化时就指定队列的容量
- `fn len() -> usize`, 返回当前队列中的元素个数
- `fn capacity() -> usize`, 对于静态队列, 返回队列中的容量
- `fn is_empty() -> bool`, 对于静态队列, 查看队列是否已满
- `fn front() -> Option<&T>`, 返回队列头部元素的共享引用, 如果有的话
- `fn front_mut() -> Option<&mut T>`, 返回队列头部元素的可变引用, 如果有的话
- `fn back() -> Option<&T>`, 返回队列尾部元素的共享引用, 如果有的话
- `fn back_mut() -> Option<&mut T>`, 返回队列尾部元素的可变引用, 如果有的话
- `fn push(value: T) -> Result<(), T>`, 简单队列需要实现的接口, 从队列的一端插入元素
- `fn pop() -> Option<T>`, 简单队列需要实现的接口, 从队列的另一端弹出元素

要实现的 traits 有这些:

- `FromIterator<T>`: 从迭代器构造隐列, 如果是静态队列, 其容量大小就是迭代器中包含的元素个数
- `PartialEq<T>, Eq<T>, PartialOrd<T>, Ord<T>`, 比较操作
- `Hash<T>`: 支持哈稀函数

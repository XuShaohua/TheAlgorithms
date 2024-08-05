# 实现简单队列

## 使用数组实现

对于有静态队列, 使用数组来实现比较符合直觉.

```rust
{{#include assets/array_queue.rs:5:150}}
```

## 使用数组实现 - 消除 `Option<T>` 类型

上面中的队列, 使用了 `[Option<T>]` 来表示数组中的元素类型, 这有些占用空间, 我们可以将这个问题消除,
通过手动操作内存的方式. 当然这会引入 `unsafe` 的函数:

```rust
{{#include assets/array_queue2.rs:5:198}}
```

## 使用链表实现

可以使用链表来实现动态数组, 不限制队列中的元素个数.

对标准库中的双链表, 就可以很容易支持队列的接口.

```rust
{{#include assets/list_queue.rs:5:112}}
```
# 栈的实现

## 使用数组实现

使用数组实现的栈结构, 它能保存的元素个数是固定的, 需要在初始化栈时指定栈的容量.

这里, 我们使用 `Box<[Option<T>]>` 用于指示数组中是否存储了元素, 如果它为 `None` 则表示在位置没有元素.

另外一种实现方式是 `Box<[T]>`, 并且要求类型 `T` 实现 `Clone` trait.

![array stack](assets/array-stack.svg)

```rust
{{#include assets/array_stack.rs:5:99}}
```

## 使用动态数组 Vec 实现

## 使用链表实现
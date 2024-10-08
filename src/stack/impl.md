# 栈的实现

## 使用数组实现

使用数组实现的栈结构, 它能保存的元素个数是固定的, 需要在初始化栈时指定栈的容量.

这里, 我们使用 `Box<[Option<T>]>` 用于指示数组中是否存储了元素, 如果它为 `None` 则表示在位置没有元素.

另外一种实现方式是 `Box<[T]>`, 并且要求类型 `T` 实现 `Clone` trait.

![array stack](assets/array-stack.svg)

```rust
{{#include assets/array_stack.rs:5:125}}
```

## 使用数组实现 - 消除 Option<T>

上面的实现过程中, 使用了 `Option<T>` 来向数组中存储元素, 这会额外占用一些内存, 操作效率有影响.
我们可以手动操作内存, 来消除 `Option<T>`:

```rust
{{#include assets/array_stack2.rs:5:165}}
```

## 使用动态数组 Vec 实现动态栈

使用 `Vec<T>` 实现的栈可以进行动态扩容, 但每次扩容时可能要进行内存的批量拷贝.

这个比较简单, 因为 `Vec<T>` 本身就实现了基本的栈操作接口, 我们只需要再包装一下就行:

```rust
{{#include assets/vec_stack.rs:5:115}}
```

## 使用链表实现动态栈

使用链表实现动态栈, 也是一个可行的方式, 为了简化代码, 我们使用了标准库中的双链表.
但是在这里使用单链表就足够了.

```rust
{{#include assets/list_stack.rs:5:101}}
```
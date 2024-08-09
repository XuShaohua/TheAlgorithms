# 链表的类型

根据链表的结构, 有这几种类型:

- 单链表 singly linked list
- 双链表 doubly linked list
-

## 单链表 Singly Linked List

在单链表中, 每个节点包括一个指针, 指向下个节点.

特点:

- 只能从链表头部单向地遍历整个链表
- 每个节点只需要存储一个指针元素, 可以节省一些内存空间

单链表的结构如下图所示:

![singly linked list](assets/singly-linked-list.svg)

C语言中对应的结构体声明如下:

```C
{{#include assets/list-types.c:5:9}}
```

## 双链表 Doubly Linked List

在双链表中, 每个节点持有两个指针, 分别指向它的前一个节点以及后一个节点.

特点:

- 可以向前和向后双向遍历整个链表
- 每个节点要存储两个指针, 占用更多的内存空间

双链表的结构如下图所示:

![doubly linked list](assets/doubly-linked-list.svg)

C语言中对应的结构体声明如下:

```C
{{#include assets/list-types.c:11:16}}
```
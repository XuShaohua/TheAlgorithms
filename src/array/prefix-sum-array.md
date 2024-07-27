# 前缀和数组 Prefix Sum Array

什么是前缀和数组? `prefix_sum_array[i] = prefix_sum_array[i - 1] + arr[i]`,

上面的定义不好理解的话, 我们再看一下例子, 原数组是 `arr[] = [1, 2, 3, 4, 5];`, 则前缀和数组就是:
`prefix_sum = [1, 3, 6, 10, 15];`.

前缀和数组的算法倒是蛮简单, 如下所示:

```rust
{{#include assets/prefix_sum.rs:7:23}}
```

该算法的时间复杂度是 `O(n)`, 空间复杂度是 `O(n)`

## 前缀和数组的应用
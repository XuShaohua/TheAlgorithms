
# 11. Container with most water

这个问题的直观解法就是两次遍历数组, 计算最大的值.
```rust, not_run
for i in 0..(len-1) {
  for j in (i+1)..len {
    ...
  }
}
```

但这样的解法, 会浪费很多步骤去计算很小的值, 尤其是 `i` 和 `j` 比较接近时, 基本上
是没有用的.

比较好的解法是分别从左右两侧进行遍历, 直到它们在某一点重叠就终止遍历:
```rust, not_run
let mut left = 0;
let mut right = len - 1;
let mut max_area = 0;
while left < right {
  let area;
  if height[left] < height[right] {
    area = height[left] * (right - left);
    left += 1;
  } else {
    area = height[right] * (right - left);
    right -= 1;
  }
  max_area = max_area.max(area);
}
```

要注意, 以上遍历有两个特点:
- 每一步只移动其中的一侧
- 每次只移动高度值较小的那一个


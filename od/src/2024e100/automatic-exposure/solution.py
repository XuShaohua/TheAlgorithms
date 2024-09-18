#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 读取输入
    img = list(map(int, input().split()))
    num_pixels = len(img)
    assert 1 <= num_pixels <= 100

    # 平均值与 128 之间的差距
    min_diff = 255
    # 与平均值与 128 之间的差距最小时的 k 值
    best_k = 0

    # 然后遍历所有可能的k值, 计算经它调整之后所有像素的平均值
    for k in range(-127, 128):
        # 计算当前的平均值
        sum_pixels = 0
        for pixel in img:
            # 遍历每个像素点, 计算新的像素值
            # 注意像素值的范围是 [0, 255]
            new_pixel = pixel + k
            new_pixel = min(new_pixel, 255)
            new_pixel = max(new_pixel, 0)
            sum_pixels += new_pixel
        avg_pixel = sum_pixels / num_pixels
        pixel_diff = abs(avg_pixel - 128)
        if pixel_diff < min_diff:
            min_diff = pixel_diff
            best_k = k
        elif pixel_diff == min_diff and best_k != 0:
            # 如果平均值相等, 那就选择较小的那个
            best_k = min(best_k, k)

    # 打印结果
    print(best_k)


if __name__ == "__main__":
    main()

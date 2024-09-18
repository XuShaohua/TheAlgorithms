#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import re

def main():
    # 读取输入
    source = input()
    assert 1 <= len(source) <= 100
    pattern = input()

    # 使用正则库
    re_pattern = re.compile(pattern)
    matches = re_pattern.search(source)
    if matches:
        print(matches.start())
    else:
        print(-1)

if __name__ == "__main__":
    main()

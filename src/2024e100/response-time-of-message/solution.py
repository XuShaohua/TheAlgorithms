#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import sys

def main():
    def get_resp_time(req_time):
        if req_time < 128:
            return req_time
        else:
            mant = req_time & 0b1111
            exp = (req_time >> 4 ) & 0b0110
            return (mant | 0x10) << (exp + 3)

    num_request = int(input().strip())

    # 读取所有的请求报文
    req_list = []
    for line in sys.stdin.readlines():
        parts = line.split()
        delay = int(parts[0])
        req_time = int(parts[1])
        req_list.append((delay, req_time))
    assert num_request == len(req_list)

    # 计算每个请求报文的响应时间, 并找到最小的值
    min_resp_time = 2 ** 32
    for delay, req_time in req_list:
        resp_time = get_resp_time(req_time)
        abs_resp_time = delay + resp_time
        min_resp_time = min(min_resp_time, abs_resp_time)
    print(min_resp_time)

if __name__ == "__main__":
    main()

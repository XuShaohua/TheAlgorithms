// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>
#include <climits>

#include <iostream>
#include <vector>

int get_resp_time(int req_time) {
  if (req_time < 128) {
    return req_time;
  } else {
    int mant = req_time & 0b1111;
    int exp = (req_time >> 4) & 0b0110;
    return (mant | 0x10) << (exp + 3);
  }
}

int main() {
  // 读取所有的请求报文

  int num_request = 0;
  std::cin >> num_request;

  std::vector<std::tuple<int, int>> req_list;
  int delay = 0;
  int req_time = 0;

  while (std::cin >> delay >> req_time) {
    req_list.emplace_back(delay, req_time);
  }
  assert(num_request == req_list.size());

  // 计算每个请求报文的响应时间, 并找到最小的值
  int min_resp_time = INT_MAX;
  for (const auto tuple: req_list) {
    int delay = std::get<0>(tuple);
    int req_time = std::get<1>(tuple);
    int resp_time = get_resp_time(req_time);
    int abs_resp_time = delay + resp_time;
    min_resp_time = std::min(min_resp_time, abs_resp_time);
  }

  std::cout << min_resp_time << std::endl;

  return 0;
}

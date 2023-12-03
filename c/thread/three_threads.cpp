// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <iostream>
#include <mutex>
#include <thread>

std::mutex g_test_mutex;
int g_num_count = 0;
constexpr int kTargetNum = 100;

void print_nums(int tid, int num_order) {
  while (true) {
    std::lock_guard<std::mutex> lock_guard(g_test_mutex);
    if (g_num_count >= kTargetNum) {
      break;
    }
    if (g_num_count % 3 == num_order) {
      std::cout << "tid: " << tid << " +" << (g_num_count + 1)<< std::endl;
      g_num_count += 1;
    }
  }
}

int main(void) {
  std::thread t1(print_nums, 1, 0);
  std::thread t2(print_nums, 2, 1);
  std::thread t3(print_nums, 3, 2);
  t1.join();
  t2.join();
  t3.join();

  return 0;
}

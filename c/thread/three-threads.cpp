// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <chrono>
#include <iostream>
#include <mutex>
#include <thread>

std::mutex g_test_mutex;
int g_num_count = 0;

void print_nums(int tid) {
  bool is_running = false;
  int local_num_count = 0;
  {
    std::lock_guard<std::mutex> lock_guard(g_test_mutex);
    local_num_count = g_num_count;
    if (g_num_count < 100) {
      g_num_count += 1;
      is_running = true;
    }
  }

  if (is_running) {
    std::cout << "tid: " << tid << " +" << (local_num_count + 1)<< std::endl;
    std::this_thread::sleep_for(std::chrono::milliseconds(10));
    print_nums(tid);
  }
}

void method1() {
  std::thread t1(print_nums, 1);
  std::thread t2(print_nums, 2);
  std::thread t3(print_nums, 3);
  t1.join();
  t2.join();
  t3.join();
}

int main(void) {
  method1();

  return 0;
}

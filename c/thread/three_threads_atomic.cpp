
#include <atomic>
#include <iostream>
#include <thread>

void print_nums(int tid) {
  static std::atomic_int counter = 0;
  int local_counter = 0;
  while ((local_counter = counter.fetch_add(1, std::memory_order_seq_cst)) < 100) {
    std::cout << "tid: " << tid << " +" << (local_counter + 1) << std::endl;
  }
}

int main(void) {
  std::thread t1(print_nums, 1);
  std::thread t2(print_nums, 2);
  std::thread t3(print_nums, 3);
  t1.join();
  t2.join();
  t3.join();

  return 0;
}

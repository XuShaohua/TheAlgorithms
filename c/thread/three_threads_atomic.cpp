
#include <atomic>
#include <iostream>
#include <thread>

void print_nums(int tid, int num_order) {
  static std::atomic_int counter = 0;
  constexpr int kTargetNum = 10000;

  int local_counter;
  while (true) {
    local_counter = counter.load(std::memory_order_seq_cst);
    if (local_counter >= kTargetNum) {
      break;
    }
    if (local_counter % 3 == num_order) {
      std::cout << "tid: " << tid << " +" << (local_counter + 1) << std::endl;
      counter.store(local_counter + 1, std::memory_order_seq_cst);
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

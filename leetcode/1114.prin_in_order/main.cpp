
#include <cstdio>

#include <atomic>
#include <functional>
#include <thread>

void printFirst() {
  printf("first");
}

void printSecond() {
  printf("second");
}

void printThird() {
  printf("third");
}

class Foo {
 public:
  Foo() : counter_(0) {}

  void first() { 
    while (counter_.load(std::memory_order_seq_cst) != 0) {
      // empty
    }
    counter_.store(1, std::memory_order_seq_cst);
    printFirst();
  }

  void second() {
    while (counter_.load(std::memory_order_seq_cst) != 1) {
      // empty
    }
    counter_.store(2, std::memory_order_seq_cst);
    printSecond();
  }

  void third() { 
    while (counter_.load(std::memory_order_seq_cst) != 2) {
      // empty
    }
    counter_.store(3, std::memory_order_seq_cst);
    printThird();
  }

 private:
  std::atomic_int counter_;
};

int main() {
  Foo foo;
  std::thread a(&Foo::first, &foo);
  std::thread b(&Foo::second, &foo);
  std::thread c(&Foo::third, &foo);
  a.join();
  b.join();
  c.join();
  return 0;
}

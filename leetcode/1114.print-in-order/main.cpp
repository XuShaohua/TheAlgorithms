
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

  void first(std::function<void()> printFirst) { 
    printFirst();
    counter_.store(1, std::memory_order_seq_cst);
  }

  void second(std::function<void()> printSecond) {
    while (counter_.load(std::memory_order_seq_cst) != 1) {
      // empty
    }
    printSecond();
    counter_.store(2, std::memory_order_seq_cst);
  }

  void third(std::function<void()> printThird) { 
    while (counter_.load(std::memory_order_seq_cst) != 2) {
      // empty
    }
    printThird();
  }

 private:
  std::atomic_int counter_;
};

int main() {
  Foo foo;
  std::thread a([&foo]() {
      foo.first(printFirst);
    });
  std::thread b([&foo]() {
      foo.second(printSecond);
    });
  std::thread c([&foo]() {
      foo.third(printThird);
    });
  a.join();
  b.join();
  c.join();
  return 0;
}

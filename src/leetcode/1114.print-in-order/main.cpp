// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cstdio>
#include <cassert>
#include <cstdio>

#include <condition_variable>
#include <functional>
#include <mutex>
#include <thread>

class Foo {
 public:
  Foo(): process_(0) {
  }

  void first(std::function<void()> printFirst) {
    printFirst();
    this->process_ = 1;
    this->cond_var_.notify_all();
  }

  void second(std::function<void()> printSecond) {
    std::unique_lock<std::mutex> lock(this->mutex_);
    while (this->process_ != 1) {
      this->cond_var_.wait(lock);
    }
    printSecond();
    this->process_ = 2;
    lock.unlock();
    this->cond_var_.notify_all();
  }

  void third(std::function<void()> printThird) {
    std::unique_lock<std::mutex> lock(this->mutex_);
    while (this->process_ != 2) {
      this->cond_var_.wait(lock);
    }
    this->process_ = 3;
    printThird();
    lock.unlock();
  }

 private:
  std::condition_variable cond_var_;
  std::mutex mutex_;
  int process_;
};

void printFirst() {
  printf("first()\n");
}

void printSecond() {
  printf("second()\n");
}

void printThird() {
  printf("third()\n");
}

void checkSolution() {
  Foo* foo = new Foo;

  std::thread t1([foo]{
      foo->first(printFirst);
      return 0;
  });

  std::thread t2([foo] {
      foo->second(printSecond);
      return 0;
  });

  std::thread t3([foo] {
      foo->third(printThird);
      return 0;
  });

  t3.join();
  t2.join();
  t1.join();

  delete foo;
}

int main() {
  checkSolution();
  return 0;
}

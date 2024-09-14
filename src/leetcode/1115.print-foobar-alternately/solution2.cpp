// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <condition_variable>
#include <functional>
#include <mutex>
#include <thread>

class FooBar {
 public:
  FooBar(int n): n_(n) { }

  void foo(std::function<void()> printFoo) {
    for (int i = 0; i < this->n_; i++) {
      std::unique_lock<std::mutex> lock(this->mutex_);

      this->cond_var_.wait(lock, [this]() {
        if (this->count_ % 2 == 0) {
          this->count_ += 1;
          return true;
        } else {
          return false;
        }
      });

      // printFoo() outputs "foo". Do not change or remove this line.
      printFoo();
      this->cond_var_.notify_all();
    }
  }

  void bar(std::function<void()> printBar) {
    for (int i = 0; i < this->n_; i++) {
      std::unique_lock<std::mutex> lock(this->mutex_);

      this->cond_var_.wait(lock, [this]() {
          if (this->count_ % 2 != 0) {
            this->count_ += 1;
            return true;
          } else {
            return false;
          }
      });
      // printBar() outputs "bar". Do not change or remove this line.
      printBar();
      this->cond_var_.notify_all();
    }
  }

 private:
  int n_;
  std::mutex mutex_;
  std::condition_variable cond_var_;
  int count_ = 0;
};

void printFoo() {
  printf("foo");
}

void printBar() {
  printf("bar");
}

void checkSolution() {
  FooBar* foo_bar = new FooBar(2);
  std::thread t1([foo_bar]() {
      foo_bar->foo(printFoo);
      return 0;
  });

  std::thread t2([foo_bar]() {
      foo_bar->bar(printBar);
      return 0;
  });

  t1.join();
  t2.join();

  delete foo_bar;
}

int main() {
  checkSolution();
  return 0;
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <condition_variable>
#include <functional>
#include <mutex>
#include <thread>

class ZeroEvenOdd {
 public:
  ZeroEvenOdd(int n) : n_(n) { }

  void zero(std::function<void(int)> printNumber) {
    for (int i = 0; i < this->n_; ++i) {
      std::unique_lock<std::mutex> lock(this->mutex_);

      while (this->stage_ != 0 && this->stage_ != 2) {
        this->cond_var_.wait(lock);
      }

      printNumber(0);
      this->stage_ += 1;
      lock.unlock();
      this->cond_var_.notify_all();
    }
  }

  void even(std::function<void(int)> printNumber) {
    const int count = this->n_ / 2;
    for (int i = 0; i < count; ++i) {
      std::unique_lock<std::mutex> lock(this->mutex_);

      while (this->stage_ != 3) {
        this->cond_var_.wait(lock);
      }

      printNumber(i * 2 + 2);
      this->stage_ = 0;
      lock.unlock();
      this->cond_var_.notify_all();
    }
  }

  void odd(std::function<void(int)> printNumber) {
    const int count = (this->n_ + 1) / 2;
    for (int i = 0; i < count; ++i) {
      std::unique_lock<std::mutex> lock(this->mutex_);

      while (this->stage_ != 1) {
        this->cond_var_.wait(lock);
      }

      printNumber(i * 2 + 1);
      this->stage_ = 2;
      lock.unlock();
      this->cond_var_.notify_all();
    }
  }

 private:
  std::mutex mutex_;
  std::condition_variable cond_var_;

  int n_;
  int stage_ = 0;
};

void printNumber(int num) {
  printf("%d", num);
}

void checkSolution() {
  //ZeroEvenOdd* obj = new ZeroEvenOdd(2);
  ZeroEvenOdd* obj = new ZeroEvenOdd(5);

  std::thread t1([obj]() {
    obj->zero(printNumber);
  });

  std::thread t2([obj]() {
    obj->odd(printNumber);
  });

  std::thread t3([obj]() {
    obj->even(printNumber);
  });

  t1.join();
  t2.join();
  t3.join();

  delete obj;
}

int main() {
  checkSolution();
  return 0;
}

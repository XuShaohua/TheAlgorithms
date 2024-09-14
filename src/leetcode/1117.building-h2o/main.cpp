// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <condition_variable>
#include <functional>
#include <mutex>
#include <string>
#include <thread>

class H2O {
 public:
  H2O() { }

  void hydrogen(std::function<void()> releaseHydrogen) {
    std::unique_lock<std::mutex> lock(this->mutex_);
    while (this->stage_ == 2) {
      this->cond_var_.wait(lock);
    }
    // releaseHydrogen() outputs "H". Do not change or remove this line.
    releaseHydrogen();
    this->stage_ += 1;
    this->cond_var_.notify_all();
  }

  void oxygen(std::function<void()> releaseOxygen) {
    std::unique_lock<std::mutex> lock(this->mutex_);
    while (this->stage_ != 2) {
      this->cond_var_.wait(lock);
    }

    // releaseOxygen() outputs "O". Do not change or remove this line.
    releaseOxygen();
    this->stage_ = 0;
    this->cond_var_.notify_all();
  }

 private:
  std::mutex mutex_;
  std::condition_variable cond_var_;
  int stage_ = 0;
};

void releaseHydrogen() {
  printf("H");
  fflush(stdout);
}

void releaseOxygen() {
  printf("O");
  fflush(stdout);
}

void checkSolution() {
  H2O* h2o = new H2O();

  std::thread t1([h2o]() {
      h2o->hydrogen(releaseHydrogen);
  });

  std::thread t2([h2o]() {
      h2o->oxygen(releaseOxygen);
  });

  t1.join();
  t2.join();

  delete h2o;
}

int main() {
  checkSolution();
  return 0;
}

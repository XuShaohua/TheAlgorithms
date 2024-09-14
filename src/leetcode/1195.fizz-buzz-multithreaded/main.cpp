// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <condition_variable>
#include <functional>
#include <mutex>
#include <thread>

class FizzBuzz {
 public:
  FizzBuzz(int n): n_(n) { }

  // printFizz() outputs "fizz".
  void fizz(std::function<void()> printFizz) {
    while (this->current_ <= this->n_) {
      std::unique_lock<std::mutex> lock(this->mutex_);
      while (this->current_ <= this->n_ && 
          !(this->current_ % 3 == 0 && this->current_ % 5 != 0)) {
        this->cond_var_.wait(lock);
      }

      if (this->increase_num()) {
        printFizz();
      }
      this->cond_var_.notify_all();
    }
  }

  // printBuzz() outputs "buzz".
  void buzz(std::function<void()> printBuzz) {
    while (this->current_ <= this->n_) {
      std::unique_lock<std::mutex> lock(this->mutex_);
      while (this->current_ <= this->n_ && 
          !(this->current_ % 5 == 0 && this->current_ % 3 != 0)) {
        this->cond_var_.wait(lock);
      }

      if (this->increase_num()) {
        printBuzz();
      }
      this->cond_var_.notify_all();
    }
  }

  // printFizzBuzz() outputs "fizzbuzz".
  void fizzbuzz(std::function<void()> printFizzBuzz) {
    while (this->current_ <= this->n_) {
      std::unique_lock<std::mutex> lock(this->mutex_);
      while (this->current_ <= this->n_ && this->current_ % 15 != 0) {
        this->cond_var_.wait(lock);
      }

      if (this->increase_num()) {
        printFizzBuzz();
      }
      this->cond_var_.notify_all();
    }
  }

  // printNumber(x) outputs "x", where x is an integer.
  void number(std::function<void(int)> printNumber) {
    while (this->current_ <= this->n_) {
      std::unique_lock<std::mutex> lock(this->mutex_);
      while (this->current_ <= this->n_ &&
        (this->current_ % 3 == 0 || this->current_ % 5 == 0)) {
        this->cond_var_.wait(lock);
      }

      if (this->increase_num()) {
        printNumber(this->current_ - 1);
      }
      this->cond_var_.notify_all();
    }
  }

 private:
  bool increase_num() {
    if (this->current_ <= this->n_) {
      this->current_ += 1;
      return true;
    } else {
      return false;
    }
  }

 std::mutex mutex_;
 std::condition_variable cond_var_;

  int n_;
  int current_ = 1;
};

void printFizz() {
  printf("fizz, ");
  fflush(stdout);
}

void printBuzz() {
  printf("buzz, ");
  fflush(stdout);
}

void printFizzBuzz() {
  printf("fizzbuzz, ");
  fflush(stdout);
}

void printNumber(int num) {
  printf("%d, ", num);
  fflush(stdout);
}

void checkSolution() {
  FizzBuzz* fizz_buzz = new FizzBuzz(15);

  std::thread t1([fizz_buzz]() {
    fizz_buzz->fizz(printFizz);
  });

  std::thread t2([fizz_buzz]() {
    fizz_buzz->buzz(printBuzz);
  });

  std::thread t3([fizz_buzz]() {
    fizz_buzz->fizzbuzz(printFizzBuzz);
  });
  std::thread t4([fizz_buzz]() {
    fizz_buzz->number(printNumber);
  });

  t1.join();
  t2.join();
  t3.join();
  t4.join();

  delete fizz_buzz;
  printf("\n");
}

int main() {
  checkSolution();
  return 0;
}

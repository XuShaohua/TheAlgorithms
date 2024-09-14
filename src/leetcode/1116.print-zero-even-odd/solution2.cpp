// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>
#include <semaphore.h>

#include <functional>
#include <thread>

class ZeroEvenOdd {
 public:
  ZeroEvenOdd(int n) : n_(n) { 
    sem_init(&this->zero_sem_, 0, 1);
    sem_init(&this->odd_sem_, 0, 0);
    sem_init(&this->even_sem_, 0, 0);
  }

  void zero(std::function<void(int)> printNumber) {
    for (int i = 0; i < this->n_; ++i) {
      sem_wait(&this->zero_sem_);
      printNumber(0);
      if (i % 2 == 1) {
        sem_post(&this->even_sem_);
      } else {
        sem_post(&this->odd_sem_);
      }
    }
  }

  void even(std::function<void(int)> printNumber) {
    for (int i = 2; i <= this->n_; i += 2) {
      sem_wait(&this->even_sem_);
      printNumber(i);
      sem_post(&this->zero_sem_);
    }
  }

  void odd(std::function<void(int)> printNumber) {
    for (int i = 1; i <= this->n_; i += 2) {
      sem_wait(&this->odd_sem_);
      printNumber(i);
      sem_post(&this->zero_sem_);
    }
  }

 private:
  sem_t zero_sem_;
  sem_t odd_sem_;
  sem_t even_sem_;

  int n_;
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

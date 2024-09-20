// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

#include <stack>

class MinStack {
 public:
  MinStack() {}

  void push(int val) {
    this->stack_.push(val);
    if (this->min_stack_.empty()) {
      this->min_stack_.push(val);
    } else {
      const int current_min = this->min_stack_.top();
      const int new_min = std::min(current_min, val);
      this->min_stack_.push(new_min);
    }
  }

  void pop() {
    this->stack_.pop();
    this->min_stack_.pop();
  }

  int top() {
    return this->stack_.top();
  }

  int getMin() {
    return this->min_stack_.top();
  }

 private:
  std::stack<int> stack_;
  std::stack<int> min_stack_;
};

int main() {
  return 0;
}

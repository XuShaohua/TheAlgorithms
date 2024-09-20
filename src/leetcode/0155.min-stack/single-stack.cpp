// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

#include <stack>

class MinStack {
 public:
  MinStack() {}

  void push(int val) {
    if (this->stack_.empty()) {
      this->stack_.emplace(val, val);
    } else {
      const int current_min = this->getMin();
      const int new_min = std::min(current_min, val);
      this->stack_.emplace(val, new_min);
    }
  }

  void pop() {
    this->stack_.pop();
  }

  int top() {
    return this->stack_.top().first;
  }

  int getMin() {
    return this->stack_.top().second;
  }

 private:
  std::stack<std::pair<int, int>> stack_;
};

int main() {
  return 0;
}


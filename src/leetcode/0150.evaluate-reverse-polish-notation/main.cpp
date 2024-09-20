// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

#include <stack>
#include <string>
#include <vector>

int evalRPN(std::vector<std::string>& tokens) {
  std::stack<int> stack;

  for (const std::string& token : tokens) {
    if (token == "+") {
      const int num1 = stack.top();
      stack.pop();
      const int num2 = stack.top();
      stack.pop();
      const int num = num2 + num1;
      stack.push(num);
    } else if (token == "-") {
      const int num1 = stack.top();
      stack.pop();
      const int num2 = stack.top();
      stack.pop();
      const int num = num2 - num1;
      stack.push(num);
    } else if (token == "*") {
      const int num1 = stack.top();
      stack.pop();
      const int num2 = stack.top();
      stack.pop();
      const int num = num2 * num1;
      stack.push(num);
    } else if (token == "/") {
      const int num1 = stack.top();
      stack.pop();
      const int num2 = stack.top();
      stack.pop();
      const int num = num2 / num1;
      stack.push(num);
    } else {
      const int num = std::stoi(token);
      stack.push(num);
    }
  }

  // 栈顶的元素就是计算结果.
  return stack.top();
}

void check_solution() {
  std::vector<std::string> tokens = {"4","13","5","/","+"};
  const int ret = evalRPN(tokens);
  assert(ret == 6);
}

int main() {
  check_solution();
  return 0;
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>

#include <stack>
#include <string>

bool isValid(std::string s) {
  std::stack<char> stack;

  for (char c : s) {
    if (c == '(' || c == '[' || c == '{') {
      stack.push(c);
    } else {
      if (stack.empty()) {
        return false;
      }
      char old_c = stack.top();
      stack.pop();
      if ((old_c == '(' && c != ')') ||
          (old_c == '[' && c != ']') ||
          (old_c == '{' && c != '}')) {
        return false;
      }
    }
  }

  return stack.empty();
}

bool isValid2(std::string s) {
  std::stack<char> stack;

  for (char c : s) {
    // 先匹配左侧的括号, 并把与之成对的右侧括号入栈.
    if (c == '(') {
      stack.push(')');
    } else if (c == '[') {
      stack.push(']');
    } else if (c == '{') {
      stack.push('}');
    } else if (stack.empty()) {
      return false;
    } else {
      // 如果遇到右侧括号, 就把它与栈顶元素比较, 看是否相同.
      char old_c = stack.top();
      stack.pop();
      if (old_c != c) {
        return false;
      }
    }
  }

  return stack.empty();
}

void checkSolution(std::string s, bool exp_valid) {
  bool is_valid = isValid(s);
  assert(is_valid == exp_valid);
}

int main() {
  checkSolution("()", true);
  checkSolution("()[]{}", true);

  return 0;
}

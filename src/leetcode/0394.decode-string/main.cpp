// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

#include <stack>
#include <string>

std::string decode_string(std::string s) {
  // 数字栈, 用于存放遇到的数字.
  std::stack<int> num_stack;
  // 字符串栈, 用于存放中间的字符串.
  std::stack<std::string> str_stack;

  // 存放当前的数字
  int num = 0;
  // 存放当前的字符串
  std::string letters;

  for (char chr : s) {
    if (std::isdigit(chr)) {
      // 数字
      const int digit = static_cast<int>(chr - '0');
      num = num * 10 + digit;
    } else if (chr == '[') {
      // 将当前的数字和字符串入栈
      num_stack.push(num);
      num = 0;

      str_stack.push(letters);
      letters.clear();
    } else if (chr == ']') {
      // 遇到了右括号, 进行字符串的拼装.
      int last_num = num_stack.top();
      num_stack.pop();

      // new_letters = last_str + last_num * letters
      std::string last_str = str_stack.top();
      str_stack.pop();

      std::string new_letters = last_str;
      for (int i = 0; i < last_num; ++i) {
        new_letters += letters;
      }
      letters = new_letters;
    } else {
      // 其它字符
      letters += chr;
    }
  }

  return letters;
}

void check_solution() {
  const auto s1 = decode_string("3[a]2[bc]");
  assert(s1 == "aaabcbc");

  const auto s2 = decode_string("3[a2[c]]");
  assert(s2 == "accaccacc");

  const auto s3 = decode_string("2[abc]3[cd]ef");
  assert(s3 == "abcabccdcdcdef");

  const auto s4 = decode_string("100[leetcode]");
  assert(s4.size() == 800);

  const auto s5 = decode_string("3[z]2[2[y]pq4[2[jk]e1[f]]]ef");
  assert(s5 == "zzzyypqjkjkefjkjkefjkjkefjkjkefyypqjkjkefjkjkefjkjkefjkjkefef");
}

int main() {
  check_solution();
  return 0;
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>
#include <iostream>
#include <tuple>
#include <vector>

int main() {
  // 读取输入
  int num_engines = 0 ;
  int num_initial_startup = 0;
  std::cin >> num_engines >> num_initial_startup;
  assert(1 <= num_engines && num_engines <= 1000);
  assert(1 <= num_initial_startup && num_engines <= 1000);

  // 引擎初始状态, (tick, position)
  std::vector<std::tuple<int, int>> initials;

  // 哪些引擎是被"手动启动"的
  for (int i = 0; i < num_initial_startup; ++i) {
    int tick = 0;
    int pos = 0;
    std::cin >> tick >> pos;
    initials.emplace_back(tick, pos);
  }

  // 标记引擎是否点火
  std::vector<bool> engines(num_engines, false);

  int engines_started = 0;
  // 记录本轮中点火的引擎
  std::vector<int> started_this_round;

  // 模拟每个时间点
  // 如果所有引擎都已点火, 就终止循环
  for (int tick = 0; engines_started < num_engines; ++tick) {
    started_this_round.clear();

    // 当前时间点中的快照
    std::vector<bool> snapshot = engines;

    // "关联启动"模式, 启动相邻的引擎
    for (int index = 0; index < num_engines; ++index) {
      // 当前引擎已经被启动
      if (engines[index]) {
        //std::cout << "CHECK sibling: " << index << std::endl;
        const int previous_index = (num_engines + index - 1) % num_engines;
        const int next_index = (index + 1) % num_engines;
        if (!snapshot[previous_index]) {
          snapshot[previous_index] = true;
          started_this_round.push_back(previous_index);
          engines_started += 1;
          //std::cout << "  START previous: " << previous_index << std::endl;
        }
        if (!snapshot[next_index]) {
          snapshot[next_index] = true;
          started_this_round.push_back(next_index);
          engines_started += 1;
          //std::cout << "  START next: " << next_index << std::endl;
        }
      }
    }

    // 检查"手动启动"的引擎
    for (const auto initial: initials) {
      const int initial_tick = std::get<0>(initial);
      const int initial_position = std::get<1>(initial);
      if (initial_tick == tick && !snapshot[initial_position]) {
        snapshot[initial_position] = true;
        engines_started += 1;
        started_this_round.push_back(initial_position);
      }
    }

    // 保存快照
    engines = snapshot;
  }

  // 打印结果
  std::cout << started_this_round.size() << std::endl;
  for (int i = 0; i + 1 < started_this_round.size(); ++i) {
    std::cout << started_this_round[i] << " ";
  }
  if (!started_this_round.empty()) {
    std::cout << started_this_round[started_this_round.size() - 1] << std::endl;
  }

  return 0;
}

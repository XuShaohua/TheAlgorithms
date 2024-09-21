// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

#include <iostream>
#include <unordered_map>
#include <vector>

// 递归计算一个 ID 的收入 
int get_salary_recursive(int parent_id, 
    std::unordered_map<int, std::vector<int>>& hierachy_tree,
    std::unordered_map<int, std::pair<int, int>>& salary_tree
    ) {
  if (hierachy_tree.find(parent_id) != hierachy_tree.cend()) {
    const std::vector<int> children = hierachy_tree[parent_id];
    int total_salary = 0;
    for (int child_id : children) {
      const int salary = get_salary_recursive(child_id, hierachy_tree, salary_tree);
      total_salary += (salary / 100) * 15;
    }
    return total_salary;
  } else {
    // 得到当前用户的收入
    return salary_tree[parent_id].second;
  }
}

void solution() {
  // 用于存储各个ID之间的关系, (parent-id, [children])
  std::unordered_map<int, std::vector<int>> hierachy_tree;
  // 用于存储各个用户的salary, (user-id, (parent-id, salary))
  std::unordered_map<int, std::pair<int, int>> salary_tree;

  // 读取输入
  int num_persons = 0;
  std::cin >> num_persons;

  int parent_id = 0;
  for (int i = 0; i < num_persons; ++i) {
    int current_id = 0;
    int salary = 0;
    std::cin >> current_id >> parent_id >> salary;
    hierachy_tree[parent_id].push_back(current_id);
    salary_tree.emplace(current_id, std::make_pair(parent_id, salary));
  }

  // 先得到 boss 的 ID
  int boss_id = parent_id;
  for (auto iter = salary_tree.find(boss_id); iter != salary_tree.cend(); iter = salary_tree.find(boss_id)) {
    boss_id = iter->second.first;
  }
  //std::cout << "boss id: " << boss_id << std::endl;

  const int boss_salary = get_salary_recursive(boss_id, hierachy_tree, salary_tree);
  // 打印结果
  std::cout << boss_salary << std::endl;
}

int main() {
  solution();

  return 0;
}

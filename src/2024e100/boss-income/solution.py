#!/usr/bin/env python3

import sys

def solution():
    salary_tree = dict()
    hierachy_tree = dict()

    first_line = input()
    # TODO(Shaohua): Check lines

    for line in sys.stdin.readlines():
        parts = line.split()
        current_id = int(parts[0])
        parent_id = int(parts[1])
        salary = int(parts[2])
        salary_tree[current_id] = (parent_id, salary)
        sibling = hierachy_tree.get(parent_id)
        if sibling is None:
            sibling = []
            hierachy_tree[parent_id] = sibling
        sibling.append(current_id)

    # Get boss id
    boss_id = current_id
    while boss_id in salary_tree:
        boss_id = salary_tree[boss_id][0]
    print("boss id:", boss_id)

    # Get boss salary
    def get_salary_recursive(parent_id):
        if parent_id in hierachy_tree:
            children = hierachy_tree[parent_id]
            total_salary = 0
            for i in range(len(children)):
                child = children[i]
                salary = get_salary_recursive(child)
                total_salary += (salary // 100) * 15
            return total_salary
        else:
            # Get real salary for current user
            return salary_tree[parent_id][1]
    
    boss_salary = get_salary_recursive(boss_id)
    print(boss_salary)

def main():
    solution()

if __name__ == "__main__":
    main()

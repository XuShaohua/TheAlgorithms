#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 读取输入
    parts = input().split()
    num_students = int(parts[0])
    num_courses = int(parts[1])
    course_list = list(input().split())
    students = []
    for i in range(num_students):
        part = input().split()
        name = part[0]
        individual_scores = list(map(int, part[1:]))
        assert len(individual_scores) == num_courses
        # 计算该学生的总分
        total_score = sum(individual_scores)
        students.append((name, individual_scores, total_score))

    sorted_by_course = input()
    # 分数要按照降序的方式来排序
    if sorted_by_course != "zongfen":
        # 如果指定的科目, 先找到它在分数列表中的索引位置, 再依此排序
        course_index = course_list.index(sorted_by_course)
        students.sort(key = lambda student: student[1][course_index], reverse = True)
    else:
        # 如果没有指定排序的科目, 就依总分来排序
        students.sort(key = lambda student: student[2], reverse = True)

    # 最后打印学生的名字
    print(" ".join(student[0] for student in students))

if __name__ == "__main__":
    main()

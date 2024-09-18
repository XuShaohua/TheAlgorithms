#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import sys

def main():
    def to_state(s: str) -> int:
        if s == "absent":
            return ABSENT
        elif s == "late":
            return LATE
        elif s == "leaveearly":
            return LEAVE_EARLY
        elif s == "present":
            return PRESENT
        else:
            assert False, "Invalid state"
    ABSENT = 0
    LATE = 1
    LEAVE_EARLY = 2
    PRESENT = 3

    # 先解析出所有的出勤记录
    num_person = int(input())
    person_presents = []
    for line in sys.stdin.readlines():
        person = list(map(to_state, line.split()))
        person_presents.append(person)
    assert num_person == len(person_presents)

    # 然后基于三条规则, 过滤是否都成立
    # 如果有一条不成立, 就返回 false
    # 否则返回 true
    person_awards  = []
    for person_present in person_presents:
        award = True
        # 缺勤
        if person_present.count(ABSENT) > 1:
            award = False
        # 没有连续的迟到/早退
        for i in range(len(person_present) - 1):
            if person_present[i] in (LATE, LEAVE_EARLY) and person_present[i + 1] in (LATE, LEAVE_EARLY):
                award = False
                break

        # 连续7次考勤, 迟到/早退/缺勤不超过3次
        if len(person_present) > 7:
            # 如果多于7次考勤, 就用滑动窗口的方式遍历所有考勤
            for i in range(len(person_present) - 7):
                if person_present[i:i+7].count(PRESENT) < 4:
                    award = False
                    break
        else:
            # 否则就只计算所有考勤
            if person_present.count(ABSENT) + person_present.count(LATE) + person_present.count(LEAVE_EARLY) > 3:
                award = False


        person_awards.append(award)

    # 输出结果
    print(" ".join("true" if award else "false" for award in person_awards))

if __name__ == "__main__":
    main()

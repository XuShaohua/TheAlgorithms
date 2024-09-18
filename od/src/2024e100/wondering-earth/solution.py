#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import sys

def main():
    # 读取输入
    parts = input().split()
    assert len(parts) == 2
    num_engines = int(parts[0])
    num_initial_startup = int(parts[1])

    # 引擎初始状态
    initials = []

    # Parse engines which started manually.
    for i in range(num_initial_startup):
        parts = input().split()
        assert len(parts) == 2
        tick = int(parts[0])
        position = int(parts[1])
        initials.append((tick, position))

    # 标记引擎是否点火
    engines = [False for i in range(num_engines)]

    engines_started = 0
    # 记录本轮中点火的引擎
    started_this_round = []

    # Simulate time tick of each round.
    for tick in range(num_engines):
        # 如果所有引擎都已点火, 就终止循环
        if engines_started == num_engines:
            break

        started_this_round.clear()
        # Take snapshot before current tick.
        snapshot = engines[:]

        # Start sibling engines.
        for index in range(num_engines):
            # Current engine is already started.
            if engines[index]:
                print("CHECK sibling:", index)
                previous_index = (num_engines + index - 1) % num_engines
                next_index = (index + 1) % num_engines
                if not snapshot[previous_index]:
                    snapshot[previous_index] = True
                    started_this_round.append(previous_index)
                    engines_started += 1
                    print("  START previous:", previous_index)
                if not snapshot[next_index]:
                    snapshot[next_index] = True
                    started_this_round.append(next_index)
                    engines_started += 1
                    print("  START next:", next_index)

        # Start initial engines.
        for (initial_tick, initial_position) in initials:
            if initial_tick == tick and not snapshot[initial_position]:
                snapshot[initial_position] = True
                engines_started += 1
                started_this_round.append(initial_position)
                print("START initial:", initial_position)

        # Save snapshot.
        engines = snapshot

    print("%d" % len(started_this_round))
    print(" ".join(str(pos) for pos in started_this_round))

if __name__ == "__main__":
    main()

#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def solution():
    invalid_words = input().split(",")
    words = input().split(",")
    words_set = [(set(word), word) for word in words]
    ans = []

    for invalid_word in invalid_words:
        invalid_chars = set(invalid_word)
        matched_word = "Not found"
        for chars_set, word in words_set:
            if chars_set == invalid_chars:
                matched_word = word
        ans.append(matched_word)
    print(",".join(ans))

def main():
    solution()

if __name__ == "__main__":
    main()

#!/usr/bin/env python3

import os
import sys


def main():
    filename = sys.argv[1]
    words = dict()
    with open(filename) as fh:
        for line in fh.readlines():
            for word in line.split():
                # if not word[-1].isalpha():
                #     word = word[:-1]
                count = words.get(word, 0)
                words[word] = count + 1

    for item in sorted(words.keys()):
        print(item, "occurs", words[item], "times")


if __name__ == "__main__":
    main()
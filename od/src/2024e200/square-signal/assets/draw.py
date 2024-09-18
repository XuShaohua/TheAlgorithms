#!/usr/bin/env python3

import matplotlib.pyplot as plt
import numpy as np


def main():
    plt.figure(2)
    wave = "00101010101100001010010"
    wave_list = list(map(int, wave))
    word = np.array(wave_list)
    plt.step(np.arange(0, len(word)), word)
    # plt.show()
    plt.savefig("draw1.svg")


if __name__ == "__main__":
    main()

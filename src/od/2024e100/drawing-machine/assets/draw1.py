#!/usr/bin/env python3

import matplotlib.pyplot as plt
from matplotlib import patches
from matplotlib.path import Path


def main():
    figure, axes = plt.subplots()
    plt.grid(True, alpha=0.5)
    plt.tight_layout()
    vertex = [
        (1, 0), (1, 1),
        (2, 1), (2, 2),
        (3, 2), (3, 3),
        (4, 3), (4, 1),
        (10, 1), (10, 0),
        (1, 0)
    ]
    codes = [
        Path.MOVETO, Path.LINETO,
        Path.LINETO, Path.LINETO,
        Path.LINETO, Path.LINETO,
        Path.LINETO, Path.LINETO,
        Path.LINETO, Path.LINETO,
        Path.CLOSEPOLY,
    ]

    path = Path(vertex, codes)
    patch = patches.PathPatch(path)
    axes.add_patch(patch)
    axes.set_xlim(-1, 11)
    axes.set_ylim(-1, 4)
    axes.set_aspect("equal")
    plt.savefig("draw1.svg")
    # plt.show()


if __name__ == "__main__":
    main()

#!/usr/bin/env python3

import matplotlib.pyplot as plt
from matplotlib import patches
from matplotlib.path import Path


def main():
    figure, axes = plt.subplots()
    plt.grid(True, alpha=0.5)
    plt.tight_layout()
    vertex = [
        (0, 0), (0, 1),
        (2, 1), (2, -2),
        (4, -2), (4, 0),
        (0, 0)
    ]
    codes = [
        Path.MOVETO, Path.LINETO,
        Path.LINETO, Path.LINETO,
        Path.LINETO, Path.LINETO,
        Path.CLOSEPOLY,
    ]

    path = Path(vertex, codes)
    patch = patches.PathPatch(path)
    axes.add_patch(patch)
    axes.set_xlim(-1, 5)
    axes.set_ylim(-3, 2)
    axes.set_aspect("equal")
    plt.savefig("draw2.svg")
    # plt.show()


if __name__ == "__main__":
    main()

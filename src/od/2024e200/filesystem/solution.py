#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import string
import sys

class FileNode:

    def __init__(self, path, parent=None):
        # 当前目录的绝对路径
        self.path = path
        # 指向父目录节点
        self.parent = parent

        # 子目录节点, 默认为空
        self.children = {}

        # 创建特属的子节点, 指向父目录
        if self.parent:
            self.children[".."] = self.parent

    def validate_folder_name(self, folder_name) -> bool:
        # 检查目录名是否包含无效字符
        for char in folder_name:
            if char not in string.ascii_lowercase:
                return False
        return True

    def mkdir(self, folder_name):
        if not self.validate_folder_name(folder_name):
            return False

        # 检查相同的目录名是否已经存在
        if folder_name in self.children:
            return False

        # 创建新的目录节点, 并存储到子目录中
        path = self.path + folder_name + "/"
        new_folder = FileNode(path, self)
        self.children[folder_name] = new_folder
        return True

    def cd(self, folder_name):
        # 进入到父目录
        if folder_name == "..":
            return True, self.parent

        # 校验目录名
        if not self.validate_folder_name(folder_name):
            return False, self

        # 未找到子目录
        if folder_name not in self.children:
            return False, self

        return True, self.children[folder_name]

def main():
    # 首先创建根目录
    root = FileNode("/")
    # 创建根目录的引用, 当前工作目录
    cwd = root

    # 然后依次读取输入, 如果输入无效, 则直接忽略, 并继续读取下一条输入
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue

        # 解析命令
        parts = line.split()
        cmd = parts[0]
        if cmd == "pwd":
            # 打印当前的目录
            if len(parts) == 1:
                print(cwd.path)
        elif cmd == "cd":
            if len(parts) == 2:
                # 切换工作目录
                folder_name = parts[1]
                ok, new_cwd = cwd.cd(folder_name)
                if not ok:
                    print("[cd] Invalid command:", line)
                else:
                    cwd = new_cwd
        elif cmd == "mkdir":
            # 创建子目录
            if len(parts) == 2:
                folder_name = parts[1]
                ok = cwd.mkdir(folder_name)
                if not ok:
                    print("[mkdir] Invalid command:", line)


if __name__ == "__main__":
    main()

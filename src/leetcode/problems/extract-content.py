#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import json
import os
import sys

def dump_content(html_file, metadata):
    title = "{0}. {1}".format(metadata["frontendQuestionId"], metadata["title"])
    content = metadata["content"]
    print(F"title: {title}")
    with open(html_file, "w") as fd:
        fd.write(F"""<!doctype html>
<html lang="en_US">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width">
  <title>{title}</title>
</head>
<body>
  <h1>{title}</h1>
  {content}
</body>
</html>
""")


def main():
    root_dir = "problems"
    for entry in os.listdir(root_dir):
        problem_dir = os.path.join(root_dir, entry)
        if not os.path.isdir(problem_dir):
            continue
        metadata_path = os.path.join(problem_dir, "metadata.json")
        with open(metadata_path) as fd:
            metadata = json.load(fd)
        html_file = os.path.join(problem_dir, "content.html")
        dump_content(html_file, metadata)


if __name__ == "__main__":
    main()

#!/bin/bash
# Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be
# found in the LICENSE file.

# Build and release the book.

set -xe

mdbook build
rsync --delete -ave "ssh -p ${ZU_SERVER_PORT}" book-out/ ${ZU_SERVER}/algs

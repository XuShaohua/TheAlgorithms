/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */

EXPLAIN
SELECT DISTINCT author_id AS id
FROM views
WHERE author_id = viewer_id;

EXPLAIN
SELECT author_id AS id
FROM views
WHERE author_id = viewer_id
GROUP BY author_id;
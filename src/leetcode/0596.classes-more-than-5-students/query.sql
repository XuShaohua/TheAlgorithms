/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */

SELECT class
FROM courses
GROUP BY class
HAVING COUNT(class) >= 5;

EXPLAIN
SELECT class
FROM courses
GROUP BY class
HAVING COUNT(class) >= 5;

EXPLAIN
SELECT class
FROM courses
GROUP BY class
HAVING COUNT(*) >= 5;
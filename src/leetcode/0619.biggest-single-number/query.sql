/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */


SELECT num
FROM mynumbers
GROUP BY num
HAVING COUNT(num) = 1
ORDER BY num DESC
LIMIT 1;

EXPLAIN
SELECT num
FROM mynumbers
GROUP BY num
HAVING COUNT(num) = 1
ORDER BY num DESC
LIMIT 1;


SELECT MAX(num) AS num
FROM (SELECT num
      FROM mynumbers
      GROUP BY num
      HAVING COUNT(num) = 1) as mn;

EXPLAIN
SELECT MAX(num) AS num
FROM (SELECT num
      FROM mynumbers
      GROUP BY num
      HAVING COUNT(num) = 1) as mn;

WITH mn AS (SELECT num
            FROM mynumbers
            GROUP BY num
            HAVING COUNT(num) = 1)
SELECT MAX(num) AS num
FROM mn;

EXPLAIN
WITH mn AS (SELECT num
            FROM mynumbers
            GROUP BY num
            HAVING COUNT(num) = 1)
SELECT MAX(num) AS num
FROM mn;
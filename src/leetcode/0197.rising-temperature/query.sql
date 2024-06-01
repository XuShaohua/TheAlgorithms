/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */

SELECT w2.id
FROM weather AS w1,
     weather AS w2
WHERE w1.recorddate + 1 = w2.recorddate
  AND w1.temperature < w2.temperature;

EXPLAIN
SELECT w2.id
FROM weather AS w1,
     weather AS w2
WHERE w1.recorddate + 1 = w2.recorddate
  AND w1.temperature < w2.temperature;

SELECT w2.id
FROM weather AS w1
         INNER JOIN
     weather AS w2
     ON w1.recorddate + 1 = w2.recorddate
         AND w1.temperature < w2.temperature;

EXPLAIN
SELECT w2.id
FROM weather AS w1
         INNER JOIN
     weather AS w2
     ON w1.recorddate + 1 = w2.recorddate
         AND w1.temperature < w2.temperature;

SELECT w1.id
FROM weather AS w1
WHERE EXISTS(SELECT 1
             FROM weather AS w2
             WHERE w2.recorddate + 1 = w1.recorddate
               AND w2.temperature < w1.temperature);

EXPLAIN
SELECT w1.id
FROM weather AS w1
WHERE EXISTS(SELECT 1
             FROM weather AS w2
             WHERE w2.recorddate + 1 = w1.recorddate
               AND w2.temperature < w1.temperature);


EXPLAIN
SELECT w1.id
FROM weather AS w1
WHERE EXISTS(SELECT 1
             FROM weather AS w2
             WHERE w2.temperature < w1.temperature
               AND w2.recorddate + 1 = w1.recorddate);
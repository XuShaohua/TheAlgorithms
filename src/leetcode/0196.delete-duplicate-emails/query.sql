/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */

SELECT *
FROM person AS p1,
     person AS p2
WHERE p1.email = p2.email
  AND p1.id > p2.id;

DELETE
FROM person AS p1 USING person AS p2
WHERE p1.email = p2.email
  AND p1.id > p2.id;


SELECT p1.id
FROM person AS p1,
     person AS p2
WHERE p1.id > p2.id
  AND p1.email = p2.email;

EXPLAIN
SELECT p1.id
FROM person AS p1,
     person AS p2
WHERE p1.id > p2.id
  AND p1.email = p2.email;

DELETE
from person
WHERE id IN (SELECT p1.id
             FROM person AS p1,
                  person AS p2
             WHERE p1.id > p2.id
               AND p1.email = p2.email);

SELECT min(id)
FROM person
GROUP BY email;

EXPLAIN
SELECT *
FROM person
WHERE id NOT IN (SELECT min(id)
                 FROM person
                 GROUP BY email);

DELETE
FROm person
WHERE id NOT IN (SELECT min(id)
                 FROM person
                 GROUP BY email);
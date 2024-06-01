/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */

SELECT p.email
FROM (SELECT email, COUNT(email) AS count
      FROM person
      GROUP BY email) AS p
WHERE p.count > 1;

EXPLAIN
SELECT p.email
FROM (SELECT email, COUNT(email) AS count
      FROM person
      GROUP BY email) AS p
WHERE p.count > 1;

SELECT email
FROM person
GROUP BY email
HAVING count(email) > 1;

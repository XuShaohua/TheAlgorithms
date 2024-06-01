/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */

SELECT name AS Employee
FROM employee AS e1
WHERE e1.salary >
      (SELECT salary
       FROM employee AS e2
       WHERE e2.id = e1.managerid);

EXPLAIN
SELECT name AS Employee
FROM employee AS e1
WHERE e1.salary >
      (SELECT salary
       FROM employee AS e2
       WHERE e2.id = e1.managerid);

SELECT e1.name AS Employee
FROM employee as e1,
     employee AS e2
WHERE e1.salary > e2.salary
  AND e1.managerid = e2.id;

EXPLAIN
SELECT e1.name AS Employee
FROM employee as e1,
     employee AS e2
WHERE e1.salary > e2.salary
  AND e1.managerid = e2.id;
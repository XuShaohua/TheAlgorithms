/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */

EXPLAIN
SELECT CASE WHEN COUNT(salary) > 0 THEN salary END AS SecondHighestSalary
FROM employee
GROUP BY id
ORDER BY salary DESC
LIMIT 1 OFFSET 3;

EXPLAIN
SELECT (SELECT DISTINCT salary
        FROM employee
        ORDER BY salary DESC
        LIMIT 1 OFFSET 1)
           AS SecondHighestSalary;


EXPLAIN
SELECT sa AS SecondHighestSalary
FROM (SELECT DISTINCT salary
      FROM employee
      ORDER BY salary DESC
      LIMIT 1 OFFSET 1)
         AS sa;

EXPLAIN
SELECT MAX(salary) AS SecondHighestSalary
FROM employee
WHERE salary < (SELECT MAX(salary) AS salary FROM employee);


EXPLAIN
WITH context AS (SELECT DISTINCT salary
                 FROM employee
                 ORDER BY salary DESC)
SELECT (SELECT salary
        FROM context
        LIMIT 1 OFFSET 1)
           AS SecondHighestSalary;
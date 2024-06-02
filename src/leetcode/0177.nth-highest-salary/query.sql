/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */


EXPLAIN
SELECT (SELECT DISTINCT salary
        FROM employee
        ORDER BY salary DESC
        LIMIT 1 OFFSET 1)
           AS SecondHighestSalary;

SELECT NULLIF((SELECT DISTINCT salary
               FROM employee
               ORDER BY salary DESC
               LIMIT 1 OFFSET 3), -1) AS SecondHighestSalary;

SELECT (SELECT salary
        FROM employee
        ORDER BY salary DESC
        LIMIT 1 OFFSET 1) AS getNthHighestSalary;

CREATE FUNCTION NthHighestSalary(N IN INTEGER)
    RETURNS INTEGER
    LANGUAGE plpgsql
AS
$$
DECLARE
    nth_salary INTEGER;
    offset_num INTEGER;
BEGIN
    --SET offset_num = MAX(N - 1, 0);
    SELECT CASE WHEN N > 0 THEN N - 1 ELSE 0 END INTO offset_num;

    SELECT DISTINCT salary
    INTO nth_salary
    FROM employee
    ORDER BY salary DESC
    LIMIT 1 OFFSET offset_num;

    RETURN nth_salary;
END;
$$;

CREATE FUNCTION NthHighestSalary(N IN INTEGER)
    RETURNS INTEGER
    LANGUAGE plpgsql
AS
$$
DECLARE
    nth_salary INTEGER;
    offset_num INTEGER;
BEGIN
    --SET offset_num = MAX(N - 1, 0);
    SELECT CASE WHEN N > 0 THEN N - 1 ELSE 0 END INTO offset_num;

    SELECT DISTINCT salary
    INTO nth_salary
    FROM employee
    ORDER BY salary DESC
    LIMIT 1 OFFSET offset_num;

    RETURN (SELECT CASE WHEN N > 0 THEN nth_salary END);
END;
$$;

CREATE FUNCTION NthHighestSalary(N IN INTEGER)
    RETURNS INTEGER
    LANGUAGE plpgsql
AS
$$
BEGIN
    RETURN (SELECT CASE
                       WHEN N < 1 THEN NULL
                       WHEN (SELECT DISTINCT COUNT(1) FROM employee) < N THEN NULL
                       ELSE (SELECT DISTINCT salary
                             FROM employee
                             ORDER BY salary DESC
                             LIMIT 1 OFFSET N - 1)
                       END);
END;
$$;

CREATE FUNCTION NthHighestSalary(N IN INTEGER)
    RETURNS TABLE
            (
                salary INTEGER
            )
    LANGUAGE plpgsql
AS
$$
BEGIN
    RETURN QUERY (SELECT CASE
                             WHEN N < 1 THEN NULL
                             WHEN (SELECT DISTINCT COUNT(1) FROM employee) < N THEN NULL
                             ELSE (SELECT DISTINCT salary
                                   FROM employee
                                   ORDER BY salary DESC
                                   LIMIT 1 OFFSET N - 1)
                             END);
END;
$$;

DROP FUNCTION NthHighestSalary;

SELECT NthHighestSalary(3);
SELECT NthHighestSalary(2);
SELECT NthHighestSalary(1);
SELECT NthHighestSalary(0);
SELECT NthHighestSalary(-1);
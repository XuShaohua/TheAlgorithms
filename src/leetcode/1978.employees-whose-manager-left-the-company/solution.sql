
SELECT e1.employee_id AS "employee_id"
FROM employees e1
WHERE e1.salary < 30000
  AND e1.manager_id IS NOT NULL
  AND NOT EXISTS(SELECT 1
                 FROM employees e2
                 WHERE e1.manager_id = e2.employee_id)
ORDER BY e1.employee_id;

-- solution 2
SELECT employee_id
FROM employees
WHERE salary < 30000
  AND manager_id NOT IN
      (SELECT employee_id
       FROM employees)
ORDER BY employee_id;

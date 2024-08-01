
-- union select

SELECT employee_id FROM salaries WHERE employee_id NOT IN (SELECT employee_id FROM employees)
UNION
SELECT employee_id FROM employees WHERE employee_id NOT IN (SELECT employee_id FROM salaries)
ORDER BY employee_id;

-- full join

SELECT employee_id FROM employees
FULL JOIN salaries
    USING (employee_id)
WHERE salaries.salary is NULL OR employees.name is NULL;


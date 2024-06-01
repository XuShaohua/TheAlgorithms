/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */

SELECT p.project_id
FROM project AS p
         LEFT JOIN employee AS e
                   ON p.employee_id = e.employee_id;

SELECT p.project_id, AVG(experience_years)
FROM employee AS e
         LEFT JOIN public.project AS p
                   ON e.employee_id = p.employee_id
GROUP BY p.project_id;

SELECT p.project_id, AVG(experience_years)::NUMERIC(10, 2) AS average_years
FROM project AS p
         LEFT JOIN employee AS e
                   ON e.employee_id = p.employee_id
GROUP BY p.project_id;


SELECT p.project_id, ROUND(AVG(experience_years)::NUMERIC, 2) AS average_years
FROM project AS p
         LEFT JOIN employee AS e
                   ON e.employee_id = p.employee_id
GROUP BY p.project_id;
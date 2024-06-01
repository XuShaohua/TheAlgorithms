/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */

SELECT name
FROM salesperson;

SELECT name
FROM salesperson
WHERE sales_id NOT IN
      (SELECT orders.sales_id
       FROM orders
                INNER JOIN company
                           ON company.com_id = orders.com_id
       WHERE company.name = 'RED');


EXPLAIN
SELECT name
FROM salesperson
WHERE sales_id NOT IN
      (SELECT orders.sales_id
       FROM orders
                LEFT JOIN company
                          ON company.com_id = orders.com_id
       WHERE company.name = 'RED');
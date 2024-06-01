/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */

SELECT c.name AS Customers
FROM customers AS c
WHERE c.id NOT IN
      (SELECT DISTINCT o.customerid
       FROM orders AS o);

EXPLAIN
SELECT c.name AS Customers
FROM customers AS c
WHERE c.id NOT IN
      (SELECT DISTINCT o.customerid
       FROM orders AS o);

EXPLAIN
SELECT c.name AS Customers
FROM customers AS c
WHERE c.id NOT IN
      (SELECT o.customerid
       FROM orders AS o);
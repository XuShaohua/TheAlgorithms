/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */

SELECT *
FROM orders;

SELECT customer_number
FROM orders
GROUP BY customer_number
ORDER BY count(order_number) DESC
LIMIT 1;

EXPLAIN
SELECT customer_number
FROM orders
GROUP BY customer_number
ORDER BY count(order_number) DESC
LIMIT 1;

SELECT customer_number, COUNT(order_number)
FROM orders
GROUP BY customer_number;

SELECT customer_number
FROM orders
GROUP BY customer_number
ORDER BY count(*) DESC
LIMIT 1;

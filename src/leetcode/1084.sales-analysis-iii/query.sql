/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */

SELECT p.product_id, p.product_name
FROM sales AS s
         LEFT JOIN product AS p
                   ON p.product_id = s.product_id
WHERE date_ge(s.sale_date, '2019-01-01')
  AND date_le(s.sale_date, '2019-03-31');


SELECT p.product_id, p.product_name
FROM sales AS s
         LEFT JOIN product AS p
                   ON p.product_id = s.product_id
GROUP BY p.product_id
HAVING MIN(s.sale_date) >= CAST('2019-01-01' AS DATE)
   AND MAX(s.sale_date) <= CAST('2019-03-31' AS DATE);


SELECT p.product_id, p.product_name
FROM sales AS s
         INNER JOIN product AS p
                    ON p.product_id = s.product_id
GROUP BY p.product_id, p.product_name
HAVING MIN(s.sale_date) >= '2019-01-01'
   AND MAX(s.sale_date) <= '2019-03-31';


EXPLAIN
SELECT p.product_id, p.product_name
FROM product AS p
         INNER JOIN sales AS s
                    ON p.product_id = s.product_id
GROUP BY p.product_id, p.product_name
HAVING MIN(s.sale_date) >= '2019-01-01'
   AND MAX(s.sale_date) <= '2019-03-31';


EXPLAIN
SELECT product_id, product_name
FROM product
WHERE product_id IN
      (SELECT s.product_id
       FROM sales AS s
       GROUP BY s.product_id
       HAVING MIN(s.sale_date) >= '2019-01-01'
          AND MAX(s.sale_date) <= '2019-03-31')
;

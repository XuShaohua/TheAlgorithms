/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */

SELECT product.product_name, sales.year, sales.price
FROM sales
         INNER JOIN product
                    ON sales.product_id = product.product_id;
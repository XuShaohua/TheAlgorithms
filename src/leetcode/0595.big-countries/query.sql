/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */

SELECT *
FROM world;

SELECT name, population, area
FROM world
WHERE area >= 3000000
   OR population >= 25000000;
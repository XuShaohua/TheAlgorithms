/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */

SELECT *
FROM cinema
WHERE description != 'boring'
  AND id % 2 = 1
ORDER BY rating DESC;
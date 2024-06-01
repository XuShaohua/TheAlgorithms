/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */

SELECT actor_id, director_id
FROM actordirector
GROUP BY (actor_id, director_id)
HAVING count(1) >= 3;

EXPLAIN
SELECT actor_id, director_id
FROM actordirector
GROUP BY (actor_id, director_id)
HAVING count(1) >= 3;
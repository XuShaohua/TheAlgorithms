/*
 * Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
 * Use of this source is governed by General Public License that can be found
 * in the LICENSE file.
 */


SELECT p.firstname, p.lastname, a.city, a.state
FROM person AS p
         LEFT JOIN address AS a
                   ON p.personId = a.personId;
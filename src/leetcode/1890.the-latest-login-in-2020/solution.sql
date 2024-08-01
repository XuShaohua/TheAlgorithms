
-- max
SELECT user_id, max(time_stamp) AS last_stamp
FROM logins
WHERE DATE_PART('year', time_stamp::date) = 2020
GROUP BY user_id;

-- order by
SELECT
    DISTINCT ON(user_id)
    user_id, time_stamp as last_stamp
FROM logins
WHERE EXTRACT(YEAR FROM time_stamp) = 2020
ORDER By user_id, time_stamp DESC;

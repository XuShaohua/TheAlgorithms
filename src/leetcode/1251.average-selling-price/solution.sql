
SELECT prices.product_id,
       COALESCE(ROUND(CAST(SUM(units * price) AS NUMERIC) / SUM(units), 2), 0) AS average_price
FROM prices
LEFT JOIN unitssold
ON unitssold.product_id = prices.product_id
AND unitssold.purchase_date >= prices.start_date
AND unitssold.purchase_date <= prices.end_date
GROUP BY prices.product_id
ORDER BY prices.product_id;

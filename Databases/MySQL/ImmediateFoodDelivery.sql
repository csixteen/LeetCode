-- Solution 1
SELECT
    ROUND(
        100 * SUM(order_date=customer_pref_delivery_date) / COUNT(*),
        2
    ) AS immediate_percentage
FROM Delivery;

-- Solution 2
SELECT
    ROUND(
        100 * COUNT(*) / (SELECT COUNT(*) FROM Delivery),
        2
    ) AS immediate_percentage
FROM Delivery
WHERE order_date = customer_pref_delivery_date;

-- Solution 3
SELECT
    ROUND(
        100 * SUM(
            CASE
                WHEN order_date = customer_pref_delivery_date THEN 1
                ELSE 0
            END
        ) / MAX(delivery_id),
        2
    ) AS immediate_percentage
FROM Delivery;

-- Solution 4
SELECT
    ROUND(
        100 * SUM(
            IF(order_date = customer_pref_delivery_date, 1, 0)
        ) / MAX(delivery_id),
        2
    ) AS immediate_percentage
FROM Delivery;

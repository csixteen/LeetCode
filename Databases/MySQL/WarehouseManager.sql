-- Warehouse Manager
-- https://leetcode.com/problems/warehouse-manager/
SELECT
    Warehouse.name AS WAREHOUSE_NAME,
    SUM(Warehouse.units*Products.Width*Products.Length*Products.Height) AS VOLUME
FROM Warehouse INNER JOIN Products
ON Warehouse.product_id = Products.product_id
GROUP BY Warehouse.name;

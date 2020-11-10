-- Patients with a condition
-- https://leetcode.com/problems/patients-with-a-condition/
SELECT * FROM Patients
WHERE conditions LIKE '% DIAB1%' OR conditions LIKE 'DIAB1%';


-- Group Sold Products by the Date
-- https://leetcode.com/problems/group-sold-products-by-the-date/
SELECT
    sell_date,
    COUNT(DISTINCT product) AS num_sold,
    GROUP_CONCAT(DISTINCT product) AS products
FROM Activities
GROUP BY sell_date;


-- Product Sales Analysis II
-- https://leetcode.com/problems/product-sales-analysis-ii/
SELECT Sales.product_id, SUM(Sales.quantity) AS total_quantity
FROM Sales
GROUP BY product_id;


-- Unique Orders and Customers per Month
-- https://leetcode.com/problems/unique-orders-and-customers-per-month/
SELECT
    DATE_FORMAT(Orders.order_date, '%Y-%m') AS month,
    COUNT(Orders.order_id) AS order_count,
    COUNT(DISTINCT Orders.customer_id) AS customer_count
FROM Orders
WHERE Orders.invoice > 20
GROUP BY 1;


-- Product Sales Analysis I
-- https://leetcode.com/problems/product-sales-analysis-i/
SELECT
    Product.product_name,
    Sales.year,
    Sales.price
FROM Sales INNER JOIN Product
ON Sales.product_id = Product.product_id;

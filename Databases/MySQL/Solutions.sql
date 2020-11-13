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

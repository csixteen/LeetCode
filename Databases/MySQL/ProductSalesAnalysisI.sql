-- Product Sales Analysis I
-- https://leetcode.com/problems/product-sales-analysis-i/
SELECT
    Product.product_name,
    Sales.year,
    Sales.price
FROM Sales INNER JOIN Product
ON Sales.product_id = Product.product_id;

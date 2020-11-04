-- Bank Account Summary II
-- https://leetcode.com/problems/bank-account-summary-ii/
SELECT Users.NAME, SUM(Transactions.amount) AS BALANCE
FROM Users INNER JOIN Transactions
ON Users.account = Transactions.account
GROUP BY Transactions.account
HAVING balance > 10000;


-- Customer who visited but did not make any transactions
-- https://leetcode.com/problems/customer-who-visited-but-did-not-make-any-transactions/
SELECT Visits.customer_id, COUNT(*) AS count_no_trans
FROM Visits LEFT JOIN Transactions
ON Visits.visit_id = Transactions.visit_id
WHERE Transactions.visit_id IS NULL
GROUP BY Visits.customer_id;


-- Warehouse Manager
-- https://leetcode.com/problems/warehouse-manager/
SELECT
    Warehouse.name AS WAREHOUSE_NAME,
    SUM(Warehouse.units*Products.Width*Products.Length*Products.Height) AS VOLUME
FROM Warehouse INNER JOIN Products
ON Warehouse.product_id = Products.product_id
GROUP BY Warehouse.name;


-- Replace employee ID with unique identifier
-- https://leetcode.com/problems/replace-employee-id-with-the-unique-identifier/
SELECT EmployeeUNI.unique_id, Employees.name
FROM Employees LEFT JOIN EmployeeUNI
ON Employees.id = EmployeeUNI.id;


-- Find the team size
-- https://leetcode.com/problems/find-the-team-size/
WITH
    team_sizes AS (SELECT team_id, COUNT(*) AS team_size FROM Employee GROUP BY team_id)
SELECT Employee.employee_id, team_sizes.team_size
FROM Employee INNER JOIN team_sizes
ON Employee.team_id = team_sizes.team_id;


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

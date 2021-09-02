-- Second Highest Salary
-- https://leetcode.com/problems/second-highest-salary/
SELECT IFNULL(
    (SELECT DISTINCT Salary FROM Employee ORDER BY Salary DESC LIMIT 1,1),
    NULL
) AS SecondHighestSalary;

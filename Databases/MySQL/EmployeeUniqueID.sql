-- Replace employee ID with unique identifier
-- https://leetcode.com/problems/replace-employee-id-with-the-unique-identifier/
SELECT EmployeeUNI.unique_id, Employees.name
FROM Employees LEFT JOIN EmployeeUNI
ON Employees.id = EmployeeUNI.id;

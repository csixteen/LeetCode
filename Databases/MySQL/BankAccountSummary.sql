-- Bank Account Summary II
-- https://leetcode.com/problems/bank-account-summary-ii/
SELECT Users.NAME, SUM(Transactions.amount) AS BALANCE
FROM Users INNER JOIN Transactions
ON Users.account = Transactions.account
GROUP BY Transactions.account
HAVING balance > 10000;

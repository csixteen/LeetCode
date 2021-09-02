-- Reformat Department Table
-- https://leetcode.com/problems/reformat-department-table/
SELECT
    d.id,
    SUM(CASE WHEN d.month = 'Jan' THEN d.revenue END) AS Jan_Revenue,
    SUM(CASE WHEN d.month = 'Feb' THEN d.revenue END) AS Feb_Revenue,
    SUM(CASE WHEN d.month = 'Mar' THEN d.revenue END) AS Mar_Revenue,
    SUM(CASE WHEN d.month = 'Apr' THEN d.revenue END) AS Apr_Revenue,
    SUM(CASE WHEN d.month = 'May' THEN d.revenue END) AS May_Revenue,
    SUM(CASE WHEN d.month = 'Jun' THEN d.revenue END) AS Jun_Revenue,
    SUM(CASE WHEN d.month = 'Jul' THEN d.revenue END) AS Jul_Revenue,
    SUM(CASE WHEN d.month = 'Aug' THEN d.revenue END) AS Aug_Revenue,
    SUM(CASE WHEN d.month = 'Sep' THEN d.revenue END) AS Sep_Revenue,
    SUM(CASE WHEN d.month = 'Oct' THEN d.revenue END) AS Oct_Revenue,
    SUM(CASE WHEN d.month = 'Nov' THEN d.revenue END) AS Nov_Revenue,
    SUM(CASE WHEN d.month = 'Dec' THEN d.revenue END) AS Dec_Revenue
FROM Department AS d
GROUP BY d.id;

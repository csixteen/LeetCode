-- Find the team size
-- https://leetcode.com/problems/find-the-team-size/
WITH
    team_sizes AS (SELECT team_id, COUNT(*) AS team_size FROM Employee GROUP BY team_id)
SELECT Employee.employee_id, team_sizes.team_size
FROM Employee INNER JOIN team_sizes
ON Employee.team_id = team_sizes.team_id;

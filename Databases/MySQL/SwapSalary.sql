-- Swap Salary
-- https://leetcode.com/problems/swap-salary/

-- Solution 1
UPDATE salary
SET sex = 
    CASE sex
        WHEN 'm' THEN 'f'
        ELSE 'm'
    END;

-- Solution 2
UPDATE salary SET sex = IF(sex = 'm', 'f', 'm');

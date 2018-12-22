-- https://leetcode-cn.com/problems/second-highest-salary

-- limit offset 
-- 解决 “NULL” 问题的另一种方法是使用 “IFNULL” 函数

SELECT
    (SELECT DISTINCT
            Salary
        FROM
            Employee
        ORDER BY Salary DESC
        LIMIT 1 OFFSET 1) AS SecondHighestSalary
;

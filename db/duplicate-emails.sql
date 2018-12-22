-- https://leetcode-cn.com/problems/duplicate-emails/

SELECT
    Email
FROM
    Person
GROUP BY
    Email
HAVING
    count(Email) > 1

-- or


SELECT
    Email
FROM (
    SELECT
        Email,
        count(*) AS count
    FROM
        Person
    GROUP BY
        Email) AS a
WHERE
    a.count > 1;
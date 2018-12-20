
-- https://leetcode-cn.com/problems/combine-two-tables/

SELECT
    FirstName,
    LastName,
    City,
    State
FROM
    Person
    LEFT JOIN Address ON Address.PersonId = Person.PersonId
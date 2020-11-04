-- Students with Invalid Departments
-- https://leetcode.com/problems/students-with-invalid-departments/
-- Note: obviously this would be solved with FOREIGN KEY constraints.

DROP TABLE IF EXISTS Students;
DROP TABLE IF EXISTS Departments;

CREATE TABLE Departments (
        id SMALLINT UNSIGNED,
        name VARCHAR(60),
        PRIMARY KEY (id)
)

INSERT INTO Departments VALUES
        (1, 'Electrical Engineering'),
        (7, 'Computer Engineering'),
        (13, 'Business Administration');

CREATE TABLE Students (
        id SMALLINT UNSIGNED,
        name VARCHAR(255),
        department_id SMALLINT UNSIGNED
)

INSERT INTO Students VALUES
        (23, 'Alice', 1),
        (1, 'Bob', 7),
        (5, 'Jennifer', 13),
        (2, 'John', 14),
        (4, 'Jasmine', 77),
        (3, 'Steve', 74),
        (6, 'Luis', 1),
        (8, 'Jonathan', 7),
        (7, 'Daiana', 33),
        (11, 'Madelynn', 1);

SELECT students.id, Students.name
FROM Students LEFT JOIN Departments
ON Students.department_id = Departments.id
WHERE Departments.id IS NULL;


CREATE TABLE problems(
id SERIAL PRIMARY KEY,
name VARCHAR(255) NOT NULL,
problem_num INT NOT NULL,
num_tests INT NOT NULL,
num_samples INT NOT NULL,
contest_id VARCHAR(255) NOT NULL REFERENCES contests(id) ON DELETE CASCADE ON UPDATE CASCADE
);
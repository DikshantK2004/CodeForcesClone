-- Your SQL goes here
CREATE TABLE test_cases(
    contest_id VARCHAR(255) NOT NULL,
    problem_num INT NOT NULL,
    test_num INT NOT NULL,
    content VARCHAR(1500) NOT NULL,
    PRIMARY KEY(contest_id, problem_num, test_num),
    FOREIGN KEY(contest_id, problem_num) REFERENCES problems(contest_id , problem_num ) ON DELETE CASCADE ON UPDATE CASCADE
);
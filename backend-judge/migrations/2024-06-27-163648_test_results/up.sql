-- Your SQL goes here
CREATE TABLE test_results(
    id SERIAL PRIMARY KEY,
    submission_id INT NOT NULL,
    test_num INT NOT NULL,
    out VARCHAR(1500) NOT NULL,
    verdict varchar(40) NOT NULL,
    time_taken INT NOT NULL
);
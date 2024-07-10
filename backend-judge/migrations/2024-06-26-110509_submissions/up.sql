-- Your SQL goes here
CREATE TABLE submissions (
    id SERIAL PRIMARY KEY,
    code VARCHAR(6000) NOT NULL,
    extension VARCHAR(20) NOT NULL,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    problem_id INT NOT NULL REFERENCES problems(id) ON DELETE CASCADE ON UPDATE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    verdict VARCHAR(255) NOT NULL DEFAULT 'pending',
    time_taken INT
);


CREATE OR REPLACE FUNCTION SET_CREATED_ON_ACCEPTED() RETURNS TRIGGER AS $$
BEGIN
    -- check if this particular user has not accepted the problem previously

    IF NEW.verdict = 'Accepted' THEN
        -- in table problems, increment the accepted column by 1
        UPDATE problems
        SET accepted = accepted + 1
        WHERE id = NEW.problem_id AND NOT EXISTS(
            SELECT 1
            FROM submissions
            WHERE problem_id = NEW.problem_id AND user_id = NEW.user_id AND verdict = 'Accepted'
        );
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE PLPGSQL;

CREATE OR REPLACE TRIGGER SET_CREATED_ON_ACCEPTED
AFTER UPDATE ON submissions
FOR EACH ROW
EXECUTE FUNCTION SET_CREATED_ON_ACCEPTED();
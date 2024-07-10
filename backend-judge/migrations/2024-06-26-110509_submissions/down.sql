-- This file should undo anything in `up.sql`
DROP TRIGGER SET_CREATED_ON_ACCEPTED ON submissions;
DROP TABLE submissions;
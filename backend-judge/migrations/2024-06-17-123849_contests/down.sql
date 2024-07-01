-- This file should undo anything in `up.sql`
DROP TRIGGER set_updated_at_on_contests ON contests;
DROP TABLE contests;

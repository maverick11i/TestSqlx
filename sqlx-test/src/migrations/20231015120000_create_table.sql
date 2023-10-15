-- 20231015120000_create_table.sql
CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    is_completed BOOLEAN DEFAULT false
);

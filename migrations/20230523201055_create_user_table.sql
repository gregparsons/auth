-- 5/23/2023
-- never edit this file after sqlx migrate has been run on it
CREATE TABLE if not exists users(
    user_id uuid PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);

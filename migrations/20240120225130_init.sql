-- Create a table for our Fruit Jokes
CREATE TABLE IF NOT EXISTS jokes (
    id SERIAL PRIMARY KEY NOT NULL,
    content TEXT NOT NULL
);
CREATE TABLE users (
  id INT PRIMARY KEY,
  username TEXT,
  pwd TEXT
);

CREATE TABLE files (
    id INT PRIMARY KEY,
    name TEXT NOT NULL,
    ext TEXT NOT NULL,
    last_modified TEXT NOT NULL,
    who_modified TEXT NOT NULL
)

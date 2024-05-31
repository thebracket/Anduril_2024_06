CREATE TABLE my_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    age INTEGER NOT NULL
);

INSERT INTO my_data (name, age) VALUES ('Alice', 42);
INSERT INTO my_data (name, age) VALUES ('Bob', 69);
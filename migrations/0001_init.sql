CREATE TABLE IF NOT EXISTS products (
    no SERIAL PRIMARY KEY,
    name VARCHAR(255),
    price INTEGER,
    deleted BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS labels (
    no SERIAL PRIMARY KEY,
    name VARCHAR(255),
    initial TIMESTAMP,
    finish TIMESTAMP,
    deleted BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS ingredients (
    no SERIAL PRIMARY KEY,
    name VARCHAR(255),
    product INTEGER REFERENCES products(no),
    quantity FLOAT,
    unit VARCHAR(100),
    deleted BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS orders (
    no SERIAL PRIMARY KEY,
    product INTEGER REFERENCES products(no),
    quantity INTEGER,
    price INTEGER,
    labels TEXT,
    time TIMESTAMP,
    deleted BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS settings (
    redundancy FLOAT DEFAULT 1.3
);

-- Add migration script here
CREATE TYPE user_role AS ENUM ('customer', 'staff', 'admin');

CREATE TABLE users (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        VARCHAR(255) NOT NULL,
    email       VARCHAR(255) UNIQUE NOT NULL,
    password    TEXT NOT NULL,
    role        user_role DEFAULT 'customer',
    created_at  TIMESTAMP DEFAULT NOW()
);

CREATE TYPE category_type AS ENUM ('books', 'film', 'manga');

CREATE TABLE genres (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL UNIQUE,
    category    category_type NOT NULL,
    created_at  TIMESTAMP DEFAULT NOW(),
    UNIQUE (name, category)
);

CREATE TABLE products (
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(255) NOT NULL,
    author      VARCHAR(255) NOT NULL,
    category    category_type NOT NULL,
    genre_id    INTEGER REFERENCES genres(id) ON DELETE SET NULL,
    price       DECIMAL(10, 2) NOT NULL,
    image       TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at  TIMESTAMP DEFAULT NOW() 
);


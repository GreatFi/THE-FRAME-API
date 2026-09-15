-- Add migration script here
CREATE TABLE genres (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    category    category_type NOT NULL,
    created_at  TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE (name, category)
);

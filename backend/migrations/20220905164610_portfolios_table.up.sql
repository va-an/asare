CREATE TABLE portfolios (
    id SERIAL PRIMARY KEY,
    user_id SERIAL NOT NULL REFERENCES users(id),
    portfolio TEXT
);
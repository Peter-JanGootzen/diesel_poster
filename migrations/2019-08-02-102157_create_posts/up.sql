CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    body text NOT NULL,
    published boolean NOT NULL DEFAULT 'f'
)

CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  text TEXT NOT NULL,
  create_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
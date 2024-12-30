-- Your SQL goes here

create TYPE status_enum AS ENUM ('pending', 'completed');
CREATE TABLE task (
    id  SERIAL PRIMARY KEY ,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status status_enum DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

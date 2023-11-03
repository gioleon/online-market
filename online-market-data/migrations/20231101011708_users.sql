-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id UUID PRIMARY KEY default uuid_generate_v4(),
    dni VARCHAR(10) NOT NULL UNIQUE,
    email VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(250) NOT NULL,
    name VARCHAR(50) NOT NULL,
    date_of_birth DATE NOT NULL,
    registered_at TIMESTAMP WITH TIME ZONE NOT NULL,
    is_seller BOOLEAN NOT NULL default FALSE,
    updated_at TIMESTAMP WITH TIME ZONE,
    latitude real NOT NULL,
    longitude real NOT NULL,
    contact_number VARCHAR(10) NOT NULL,
    category_id bigint,
    rol roles NOT NULL,
    CONSTRAINT fk_category_user
        FOREIGN KEY (category_id)
            REFERENCES categories (id)
);
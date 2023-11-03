-- Add migration script here

CREATE TABLE services (
    id UUID PRIMARY KEY default uuid_generate_v4(),
    user_id VARCHAR(10) NOT NULL unique,
    category_id bigint NOT NULL,
    price real NOT NULL,
    description VARCHAR(200) NOT NULL,
    modality modality NOT NULL,
    CONSTRAINT fk_services_users 
        FOREIGN KEY (user_id)
            REFERENCES users (dni),
    CONSTRAINT fk_services_categories
        FOREIGN KEY (category_id)
            REFERENCES categories (id) 
)
-- Add migration script here
CREATE TABLE rates (
    rater VARCHAR(10) NOT NULL,
    rated VARCHAR(10) NOT NULL,
    rate real NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE,
    CONSTRAINT fk_rates_rater
        FOREIGN KEY (rater)
            REFERENCES users (dni),
    CONSTRAINT fk_rates_rated
        FOREIGN KEY (rated)
            REFERENCES users (dni),
    UNIQUE (rater, rated)
)
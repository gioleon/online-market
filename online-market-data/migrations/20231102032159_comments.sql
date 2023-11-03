-- Add migration script here
CREATE TABLE comments (
    commentator VARCHAR(10) NOT NULL,
    commented VARCHAR(10) NOT NULL,
    comment VARCHAR(200) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE,
    CONSTRAINT fk_comments_commentator
        FOREIGN KEY (commentator)
            REFERENCES users (dni), 
    CONSTRAINT fk_comments_commented
        FOREIGN KEY (commented)
            REFERENCES users (dni),
    UNIQUE (commentator, commented)
)

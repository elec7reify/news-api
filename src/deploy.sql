CREATE TABLE articles (
    id SERIAL NOT NULL,
    title VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
);

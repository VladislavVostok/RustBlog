CREATE TABLE posts(
                      id serial primary key,
                      title varchar(255) NOT NULL,
                      body text not null,
                      published boolean not null default false
);

INSERT INTO posts(title, body, published)
VALUES
    ('Первый пост', 'Содержание первого поста!', true),
    ('Второй пост', 'Содержание второго поста!', true);

SELECT * FROM posts;



CREATE TABLE users(
                      id serial primary key,
                      username varchar(50) UNIQUE NOT NULL,
                      email varchar(100) unique not null,
                      password_hash varchar(255) not null,
                      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);



CREATE TABLE users(
                      id serial primary key,
                      username varchar(50) UNIQUE NOT NULL,
                      email varchar(100) unique not null,
                      password_hash varchar(255) not null,
                      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
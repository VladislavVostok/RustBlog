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
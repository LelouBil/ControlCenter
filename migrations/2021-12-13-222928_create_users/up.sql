create table users
(
    username VARCHAR not null
        constraint users_pk
            primary key,
    password VARCHAR not null
);
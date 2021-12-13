create table Users
(
    username VARCHAR not null
        constraint Users_pk
            primary key,
    password VARCHAR not null
);
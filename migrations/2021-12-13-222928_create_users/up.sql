create table users
(
    username VARCHAR not null
        constraint users_pk
            primary key,
    password VARCHAR 
);

insert into users (username, password) VALUES ('admin','$argon2d$v=19$m=16,t=2,p=1$MTRxaldnbllJTms4U0xKYg$D0HxZsPvAHnc6bkOjfb2xA')
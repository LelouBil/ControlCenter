create table postes
(
    is_on          boolean not null,
    is_compromised boolean not null,
    os             varchar not null,
    hostname       varchar not null,
    ip             varchar not null
        constraint postes_pk
            primary key
);

create unique index postes_hostname_uindex
    on postes (hostname);


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

insert into postes (is_on, is_compromised, os, hostname, ip) 
VALUES (
        true,
        false,
        'Debian',
        'dinfo142',
        '192.168.0.142'
       );

insert into postes (is_on, is_compromised, os, hostname, ip)
VALUES (
           false,
           false,
           'Windows',
           'dinfo198',
           '192.168.0.198'
       );

insert into postes (is_on, is_compromised, os, hostname, ip)
VALUES (
           true,
           false,
           'Windows',
           'dinfo216',
           '192.168.0.216'
       );

insert into postes (is_on, is_compromised, os, hostname, ip)
VALUES (
           true,
           true,
           'CentOS',
           'dinfo234',
           '192.168.0.234'
       );
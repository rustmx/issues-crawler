create table projects (
    id bigserial primary key,
    name varchar(256) not null,
    url varchar(4096) not null,
    watchers bigint default 0,
    forks bigint default 0,
    stars bigint default 0
)
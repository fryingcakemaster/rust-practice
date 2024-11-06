create table if not exists tasks (
    id serial not null,
    addr1 varchar(48) not null,
    addr2 varchar(48) not null,
    addr3 varchar(48) not null,
    status integer not null default(0)
);
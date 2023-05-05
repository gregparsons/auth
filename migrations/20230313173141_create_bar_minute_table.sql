create table if not exists bar_minute
(
    id           bigserial,
    dtg          timestamp,
    symbol       varchar,
    price_open   numeric(20, 10),
    price_high   numeric(20, 10),
    price_low    numeric(20, 10),
    price_close  numeric(20, 10),
    volume       integer
);

alter table bar_minute
    owner to postgres;

create index if not exists widx_bar_minute_dtg
    on bar_minute (dtg);
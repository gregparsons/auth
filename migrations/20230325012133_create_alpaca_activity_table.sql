-- drop table alpaca_activity;
create table if not exists alpaca_activity
(
    id varchar,
    activity_type varchar,
    activity_subtype varchar,
    transaction_time timestamptz,
    symbol varchar,
    side varchar,
    qty numeric(20, 10),
    price numeric(20, 10),
    cum_qty numeric(20, 10),
    leaves_qty numeric(20, 10),
    order_id varchar

);

alter table alpaca_activity
    owner to postgres;

create index if not exists idx_order_id
    on alpaca_activity (id);-- Add migration script here

ALTER TABLE alpaca_activity
    ADD CONSTRAINT alpaca_activity_id_unique
        UNIQUE (id) ;
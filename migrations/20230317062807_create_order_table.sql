create table if not exists alpaca_order
(
    id varchar,
    client_order_id varchar,
    created_at timestamptz,
    updated_at timestamptz,
    submitted_at timestamptz,
    filled_at timestamptz,
    expired_at timestamptz,
    canceled_at timestamptz,
    failed_at timestamptz,
    replaced_at timestamptz,
    replaced_by timestamptz,
    replaces varchar,
    asset_id varchar,
    symbol varchar,
    asset_class varchar,
    notional numeric(20, 10),
    qty integer,
    filled_qty integer,
    filled_avg_price numeric(20, 10),
    order_class varchar,
    order_type_v2 varchar,
    side varchar,
    time_in_force varchar,
    limit_price numeric(20, 10),
    stop_price numeric(20, 10),
    status varchar,
    extended_hours boolean,
    trail_percent numeric(20, 10),
    trail_price numeric(20, 10),
    hwm numeric(20, 10)

);

alter table alpaca_order
    owner to postgres;

create index if not exists idx_order_id
    on alpaca_order (id);-- Add migration script here

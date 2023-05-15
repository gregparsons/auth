# Alpaca Collector
A Docker-based websocket and rest API collector of Alpaca stock ticker data. Collects data to a PostgreSQL  
database. Cross-talking threads store inbound data, analyze it, and take appropriate trade actions on it.

## Environment Notes  
Dev environment: forward a local port to the machine where postgres is running; makes it look like postgres is running locally but secured over SSH without exposing the Postgres port. 
ssh -L 54320:10.1.1.205:54320 swimr205

Prep sqlx for offline build inside docker:
export DATABASE_URL=postgres://postgres:eJk16bVgFNkJI74s3uY248vwCX7rEkUbGXrZtS8V4PDn8e2HcC@localhost:54320/alpaca
cargo sqlx prepare -- --lib

## TODO
- remove chrono per https://github.com/chronotope/chrono/issues/602 and cargo audit
- not resilient to local power/internet outage?
- 406 error occurs when too many connections, close websocket and attempt reconnect after some delay

## sqlx
sqlx migrate add create_user_table  
sqlx migrate run --database-url postgres://postgres:password@localhost:54320/alpaca  

## postgres
Generate v4 uuids:
> CREATE EXTENSION "uuid-ossp";
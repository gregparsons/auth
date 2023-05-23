# frontend

## Environment Notes  
Dev environment: forward a local port to the machine where postgres is running; makes it look like postgres is running locally but secured over SSH without exposing the Postgres port. 
ssh -L 54320:10.1.1.205:54320 swimr205

Prep sqlx for offline build inside docker:
export DATABASE_URL=postgres://postgres:e7cED6yPdnREyonjVDfBupkqXGjT8Xbnfe4pMote48HCPvAxY4@localhost:54320/auth_template
cargo sqlx prepare -- --lib

## sqlx
sqlx migrate add create_user_table  
sqlx migrate run --database-url postgres://postgres:password@localhost:54320/alpaca  
sqlx migrate run --database-url postgres://postgres:e7cED6yPdnREyonjVDfBupkqXGjT8Xbnfe4pMote48HCPvAxY4@localhost:54320/auth_template

=
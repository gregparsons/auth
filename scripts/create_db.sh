


docker volume create volume_postgres_auth_00
docker network create auth-net

docker run -d --rm \
--name postgres_auth \
-p 54320:5432 \
-v volume_postgres_auth_00:/var/lib/postgresql/data \
-e PGDATA=/var/lib/postgresql/data/pgdata \
-e POSTGRES_PASSWORD=e7cED6yPdnREyonjVDfBupkqXGjT8Xbnfe4pMote48HCPvAxY4 \
postgres


# test
psql -h localhost -p 54320 -U postgres -W e7cED6yPdnREyonjVDfBupkqXGjT8Xbnfe4pMote48HCPvAxY4

# create database auth_template with owner postgres;
# create schema schema_auth;
# alter schema schema_auth owner to postgres;

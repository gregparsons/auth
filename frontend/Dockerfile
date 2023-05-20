# Dockerfile
# Split into stages to reduce final image size

FROM rust:1.68.2 as builder
RUN apt-get update && apt-get -y upgrade && apt install lld clang -y

WORKDIR /app
COPY . .
COPY ../common_lib /app/common_lib
ENV SQLX_OFFLINE=true
RUN cargo build -p frontend --release

# runtime stage
FROM rust:1.68.2-slim as runtime
WORKDIR /app
COPY --from=builder /app/target/release/trade_frontend trade_frontend

COPY .env .
# copy the contents of static (the templates folder) to a path that works without drama equally in/out of docker
COPY static /app/frontend/static

# relative to the Dockerfile
COPY config/configuration.yaml .

ENTRYPOINT ["./trade_frontend"]


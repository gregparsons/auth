# frontend/all_docker.sh
make docker_build_frontend

# cd frontend
# docker build . --tag frontend
docker stop frontend
docker rm frontend
sleep 2
# -e API_INTERVAL_MILLIS=30000 \
docker run -d \
 --restart=always \
 --name frontend \
 -p 8080:8080 \
 -e RUST_LOG="debug,rustls=error,tokio::postgres=error" \
 -e ALPACA_API_URL=https://data.alpaca.markets \
 -e ALPACA_WS_URL=wss://stream.data.alpaca.markets/v2/iex \
 -e DATABASE_URL="postgres://postgres:eJk16bVgFNkJI74s3uY248vwCX7rEkUbGXrZtS8V4PDn8e2HcC@10.1.1.205:54320/alpaca" \
 -e CONFIG_LOCATION=docker \
 frontend:latest
sleep 3
docker logs --follow frontend
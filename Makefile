frontend:
	./frontend/all.sh
sqlx:
	cargo sqlx prepare --merged
maintenance:
	cargo fmt
	cargo clippy
	cargo audit
	cargo test
	# rustup update
	# rustup self update
	# cargo doc
docker_build_frontend:
	docker build --file dockerfile_frontend --tag frontend .
docker_run_frontend:
	docker run --rm --net auth-net --name frontend frontend
docker_run_postgres:
	# todo
	docker run --net auth-net --name postgres-auth postgres

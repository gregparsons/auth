web:
	cargo run --release -- frontend_ui
webdev:
	cargo run -- frontend_ui
frontend:
	./frontend_ui/all.sh
backend:
	./backend_data/all.sh
sqlx_frontend:
	cd frontend_ui;cargo sqlx prepare -- --lib
sqlx_backend:
	cd backend_data;cargo sqlx prepare -- --lib
maintenance:
	cargo fmt
	cargo clippy
	cargo audit
	cargo test
	# rustup update
	# rustup self update
	# cargo doc
test:
	export CONFIG_LOCATION=test;cargo test -p trader
devport:
	ssh -L 54320:10.1.1.205:54320 swimr205

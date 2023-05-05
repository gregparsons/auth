web:
	cargo run --release -- frontend_ui
webdev:
	cargo run -- frontend_ui
prod:
	./frontend_ui/all.sh
dev:
	cargo run -p trader
maintenance:
	cargo fmt
	cargo clippy
	cargo audit
	cargo test
	# rustup update
	# rustup self update
	# cargo doc
devport:
	ssh -L 54320:10.1.1.205:54320 swimr205
sqlx:
	cd trader;cargo sqlx prepare -- --lib
test:
	export CONFIG_LOCATION=test;cargo test -p trader
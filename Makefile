all:
	test production

test:
	cargo test

production:
	cargo build --release
	strip target/release/
	mv target/debug/palladium /usr/local/bin/
	chmod ugo+x /usr/local/bin/

dev:
	cargo build
	mv target/debug/palladium /usr/local/bin/
	chmod ugo+x /usr/local/bin/

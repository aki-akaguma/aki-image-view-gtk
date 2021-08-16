
all: README.md

README.md: src/main.rs
	cargo readme > $@

test:
	cargo test

clean:
	cargo clean

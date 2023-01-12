
all: readme

readme: README.md

README.md: README.tpl src/main.rs
	cargo readme > $@

test:
	cargo test

clean:
	cargo clean

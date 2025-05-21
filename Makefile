NAME=codex

all:
	cargo run -- $(ARGS)

install:
	sudo cp target/release/$(NAME) /usr/local/bin/

build-install:
	cargo b -r
	sudo cp target/release/$(NAME) /usr/local/bin/


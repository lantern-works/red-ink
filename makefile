TARGET := arm-unknown-linux-musleabihf
BIN := target/$(TARGET)/release/red-ink

.PHONY: all upload run

all: upload run

$(BIN): src/*.rs
  docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:arm-musleabihf cargo build --release

upload: $(BIN)
	scp $(BIN) lantern:/home/admin/

run: upload
	ssh lantern sudo /home/admin/red-ink hello, from, make, land

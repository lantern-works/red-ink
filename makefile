TARGET := arm-unknown-linux-musleabihf
BIN := target/$(TARGET)/release/red-ink

.PHONY: all check upload run

all: check upload run

check:
	cross check --target=$(TARGET)
	
$(BIN): src/main.rs
	docker run --rm -it -v "$(PWD)":/home/rust/src messense/rust-musl-cross:arm-musleabihf cargo build --release

upload: $(BIN)
	scp $(BIN) lantern:/home/admin/

run:
	ssh lantern sudo /home/admin/red-ink hello, from, make, land

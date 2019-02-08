TARGET := arm-unknown-linux-musleabihf
BIN := target/$(TARGET)/release/red-ink
DESTDIR := /home/admin/bin

.PHONY: all check upload run

all: check upload run

check:
	cross check --target=$(TARGET)
	
$(BIN): src/main.rs
	docker run --rm -it -v "$(PWD)":/home/rust/src messense/rust-musl-cross:arm-musleabihf cargo build --release

upload: $(BIN)
	scp $(BIN) lantern:$(DESTDIR)

run:
	ssh lantern sudo $(DESTDIR)/red-ink hello, from, make, land

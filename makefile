TARGET := arm-unknown-linux-musleabihf
BIN := target/$(TARGET)/release/red-ink

.PHONY: all upload run

all: upload run

$(BIN): src/*.rs
	cross build --target=$(TARGET) --release

upload: $(BIN)
	scp $(BIN) lantern:/home/admin/

run:
	ssh lantern sudo /home/admin/red-ink

.PHONY: all build upload run

all: build upload run

build:
	cross build --target=arm-unknown-linux-gnueabihf --release

upload:
	scp target/arm-unknown-linux-gnueabihf/release/red-ink lantern:/home/admin/

run:
	ssh lantern sudo /home/admin/red-ink

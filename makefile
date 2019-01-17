REDINK := red-ink/arm-unknown-linux-gnueabihf
.PHONY: all build upload run

all: image build upload run

image:
	docker build -t $(REDINK) .

build:
	cross build --target=arm-unknown-linux-gnueabihf --release

upload:
	scp target/arm-unknown-linux-gnueabihf/release/red-ink lantern:/home/admin/

run:
	ssh lantern sudo /home/admin/red-ink

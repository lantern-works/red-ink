TARGET := arm-unknown-linux-musleabihf
BUILDER := red-ink/$(TARGET)

.PHONY: all build upload run

all: image build upload run

image:
	docker build -t $(BUILDER) .

build:
	cross build --target=$(TARGET) --release

upload:
	scp target/$(TARGET)/release/red-ink lantern:/home/admin/

run:
	ssh lantern sudo /home/admin/red-ink

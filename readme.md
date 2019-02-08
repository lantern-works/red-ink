# red-ink

warning: this uses agressive timings for the eink display.
you should manually do a full refresh once in a while, or risk burnin

Open an interactive mode for sentences

    red-ink

Show four sentences

    red-ink some, comma, separated, sentences

Display a 212x140 image in black and white

    red-ink path/to/image.bmp


## development

run locally on the pi

    cargo run

cross compile from macos for pi zero

    make build

build, upload, and run on the lantern

    make


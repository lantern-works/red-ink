eink cli

warning: this uses agressive timings for the eink display.
you should manually do a full refresh once in a while, or risk burnin

usage

    red-ink # run in interactive mode

    red-ink some, comma, separated, sentences # display up to 4 lines of text from cli

run locally on the pi

    cargo run

cross compile from macos for pi zero

    make build

build, upload, and run on the lantern

    make

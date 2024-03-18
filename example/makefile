CC=emcc
CONFIGURE=emconfigure
EMMAKE=emmake
SOURCES=main.c protocol.c protocol.h
CFLAGS = -Oz --no-entry -sERROR_ON_UNDEFINED_SYMBOLS=0 -sFILESYSTEM=0 -sASSERTIONS=0 -sEXPORT_KEEPALIVE=1 -Wall -Wno-logical-op-parentheses

all: build

build: $(SOURCES)
	$(CC) $(CFLAGS) $(SOURCES) -o example.wasm
	wasi-stub --stub-function env:__syscall_unlinkat,env:__syscall_faccessat ./example.wasm

protocol.c protocol.h: protocol.prot
	cd ..\
	cargo run -- protocol.prot -o example
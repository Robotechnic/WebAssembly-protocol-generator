CC=emcc
CONFIGURE=emconfigure
EMMAKE=emmake
SOURCES=main.c protocol.c
CFLAGS = -Oz --no-entry -sERROR_ON_UNDEFINED_SYMBOLS=0 -sFILESYSTEM=0 -sASSERTIONS=0 -sEXPORT_KEEPALIVE=1 -Wall -Wno-logical-op-parentheses

all: build

build: $(SOURCES)
	$(CC) $(CFLAGS) $(SOURCES) -o example.wasm
	wasi-stub --stub-function env:__syscall_unlinkat,env:__syscall_faccessat ./example.wasm

protocol.c protocol.h: example.prot
	cd ..;\
	cargo run -- ./example/example.prot -o ./example

clean:
	rm example.wasm
	rm protocol.c
	rm protocol.h
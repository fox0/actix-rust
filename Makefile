all: build

build: src/main.rs
	cargo build --release

clean:
	rm -rf target/

all: target/x86_64-apple-darwin/release/libclibrespot.a clibrespot.h

clean:
	rm -rf target/

target/x86_64-apple-darwin/release/libclibrespot.a: src/lib.rs src/player.rs Cargo.toml cbindgen.toml
	RUSTFLAGS="-C link-arg=-mmacosx-version-min=10.7" MACOSX_DEPLOYMENT_TARGET=10.7 cargo build --release --target x86_64-apple-darwin

clibrespot.h: target/x86_64-apple-darwin/release/libclibrespot.a
	cbindgen --config cbindgen.toml --crate clibrespot --lang c --output clibrespot.h

.PHONY: all clean

all:
	@cargo build --release

run:
	@cargo run --release --quiet

clean:
	@cargo clean

lint:
	cargo clippy --all-targets --all-features

clean:
	cargo clean

build:
	cargo build && alias mlbh2h='./target/debug/mlbh2h'

test:
	cargo test

run:
	cargo run -- -d 2019-05-16 -l kbo -f pretty

#!make

APP=vs

.PHONY: build release run clippy test unit_tests clean doc


build:
	cargo build

clippy:
	cargo clippy

release:
	cargo build --release

run:
	which cargo && \
	    cargo run --bin $(APP)

# If TEST is not defined, all tests will be run.
# --nocapture allows output to be displayed
test:
	cargo test -- --nocapture $(TEST)


# If TEST is not defined, all tests will be run.
# --nocapture allows output to be displayed
unit_tests:
	cargo test --lib -- --nocapture

clean:
	cargo clean

doc:
	cargo doc

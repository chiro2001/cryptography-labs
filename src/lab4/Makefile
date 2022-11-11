ifdef BINARY
BINARY_OPTS := --binary
endif

test:
	cargo run -- $(BINARY_OPTS) -m generate
	sleep 1
	cargo run -- $(BINARY_OPTS) -m sign
	sleep 1
	cargo run -- $(BINARY_OPTS) -m test
	sleep 1
	cargo run -- $(BINARY_OPTS) -m check

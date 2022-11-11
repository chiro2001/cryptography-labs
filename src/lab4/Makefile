ifdef BINARY
BINARY_OPTS := --binary
endif
SOURCE_OPTS := -i data/lab4-message.txt

OPTS := $(BINARY_OPTS) $(SOURCE_OPTS)

lab:
	./scripts/lab4.sh

test:
	cargo run -- $(OPTS) -m generate
	sleep 1
	cargo run -- $(OPTS) -m sign
	sleep 1
	cargo run -- $(OPTS) -m test
	sleep 1
	cargo run -- $(OPTS) -m check

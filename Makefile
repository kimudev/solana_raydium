ENDPOINT ?= mainnet.sol.streamingfast.io:443

MANIFEST = "./substreams.yaml"

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	if [ -n "$(STOP)" ]; then \
		substreams run -e $(ENDPOINT) substreams.yaml block_kafka_output -s $(START) -t $(STOP); \
	elif [ -n "$(START)" ]; then \
		substreams run -e $(ENDPOINT) substreams.yaml block_kafka_output -s $(START); \
	else \
		substreams run -e $(ENDPOINT) substreams.yaml block_kafka_output; \
	fi

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: clean
clean:
	rm -rf target

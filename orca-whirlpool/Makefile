ENDPOINT ?= mainnet.sol.streamingfast.io:443

.PHONY: build
build:
	LDFLAGS="-Wl,-no_compact_unwind" cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml map_swaps -s 126272128 -t +1

.PHONY: gui
gui: build
	substreams gui substreams.yaml map_swaps -s 126272128 -t +1 --production-mode

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: pack
pack:
	substreams pack ./substreams.yaml

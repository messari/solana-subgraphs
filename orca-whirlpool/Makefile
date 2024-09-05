ENDPOINT ?= mainnet.sol.streamingfast.io:443

.PHONY: build
build:
	LDFLAGS="-Wl,-no_compact_unwind" cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml map_pools -s 124288454 -t +1000

.PHONY: gui
gui: build
	substreams gui substreams.yaml graph_out -s 124152351 --production-mode

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: pack
pack:
	substreams pack ./substreams.yaml

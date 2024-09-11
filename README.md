# Solana Subgraphs

This repository contains subgraphs for indexing and querying data from various Solana protocols. Currently, it includes a subgraph for the Orca Whirlpool protocol.

## Overview

Solana Subgraphs use [The Graph](https://thegraph.com/) protocol to index and query blockchain data efficiently. This project leverages Substreams technology to process Solana blockchain data and generate entities for Graph Protocol indexing.

## Subgraphs

### Orca Whirlpool

The Orca Whirlpool subgraph indexes data from the Orca Whirlpool protocol on the Solana blockchain. It processes transactions related to liquidity pools, swaps, deposits, and withdrawals.

Key features:
- Indexes pool creation and updates
- Tracks liquidity deposits and withdrawals
- Records swap events
- Maintains cumulative statistics

For more details on the Orca Whirlpool subgraph, refer to its specific README: [Orca Whirlpool README](./orca-whirlpool/README.md)


## Getting Started

To build and run a subgraph:

1. Navigate to the subgraph directory (e.g., `cd orca-whirlpool`)
2. Install dependencies: `cargo build`
3. Generate protocol buffers: `cargo protogen`
4. Build the subgraph: `cargo build --target wasm32-unknown-unknown --release`
5. Run the substream: `cargo stream`

## Development

To add support for new instructions or modify existing ones, refer to the "Adding Support for New Instructions" section in the Orca Whirlpool README.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
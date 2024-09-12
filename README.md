# Solana Subgraphs

This repository contains subgraphs for indexing and querying data from various Solana protocols. Currently, it includes a subgraph for the Orca Whirlpool protocol.

## Overview

Solana Subgraphs use [The Graph](https://thegraph.com/) protocol to index and query blockchain data efficiently. This project leverages Substreams technology from [StreamingFast](https://www.streamingfast.io/) to process Solana blockchain data and generate entities for Graph Protocol indexing.

## Getting Started

To build and run a subgraph:

1. Navigate to the subgraph directory (e.g., `cd orca-whirlpool`)
2. Generate protocol buffers: `make protogen`
3. Build the substream: `make build`
4. Pack the substream: `make pack`
5(a). If you want to run the substream locally: `make stream`
5(b). If you want to deploy to The Graph: `graph deploy --studio <location> --deploy-key <deployment key>`

Refer to [Substreams Docs](https://substreams.streamingfast.io/documentation) for more information on setting up and running a substream.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
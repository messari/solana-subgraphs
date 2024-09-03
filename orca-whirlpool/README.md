# Orca Whirlpool Substream

## Overview

This project is a Substream for the Orca Whirlpool protocol on the Solana blockchain. It processes and indexes data from Orca Whirlpool, providing structured and easily queryable information about liquidity pools, swaps, deposits, and withdrawals.

## Features

- Tracks initialization of new liquidity pools
- Monitors liquidity additions and removals
- Indexes swap events, including two-hop swaps
- Calculates daily usage metrics and pool statistics
- Provides cumulative user counts and total pool counts

## Project Structure

The project is primarily written in Rust and uses the Substreams framework. Here's an overview of the main components:

- `src/`: Contains the main Rust source code
  - `modules/`: Individual substream modules for different functionalities
  - `instructions/`: Definitions for Orca Whirlpool instructions
  - `pb/`: Protocol Buffer definitions
  - `db.rs`: Database handling functions
  - `key_store.rs`: Key management for data storage
  - `lib.rs`: Main library file
  - `orca_instructions.rs`: Orca-specific instruction handling
- `proto/`: Protocol Buffer definitions
- `substreams.yaml`: Substreams configuration file
- `Cargo.toml`: Rust package configuration

## Key Modules

1. `map_block`: Processes raw block data and extracts relevant events
2. `map_pools`: Tracks new pool initializations
3. `map_deposits` and `map_withdraws`: Processes liquidity additions and removals
4. `map_swaps`: Indexes swap events
5. `store_*` modules: Various modules for storing and updating metrics

## Setup and Usage

1. Ensure you have Rust and the `wasm32-unknown-unknown` target installed.
2. Install the Substreams CLI tool.
3. Build the project:
   ```
   cargo build --target wasm32-unknown-unknown --release
   ```
4. Pack the Substream:
   ```
   substreams pack ./substreams.yaml
   ```
5. Run the Substream:
   ```
   substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_pools -s 124288454 -t +1000
   ```

## Configuration

The main configuration file is `substreams.yaml`. It defines the modules, their dependencies, and the initial block for processing.

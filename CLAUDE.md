# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Artemis is a high-performance MEV (Maximal Extractable Value) bot framework written in Rust, designed for building arbitrage and MEV strategies on Ethereum. It implements a modular, event-driven architecture optimized for speed and reliability.

## Core Architecture

The framework follows a **three-stage pipeline**: **Collectors** → **Strategies** → **Executors**

- **Collectors**: Data sources that emit blockchain events (blocks, mempool, logs, orders)
- **Strategies**: Business logic that processes events and generates actions
- **Executors**: Submit transactions/bundles to various execution layers

The `Engine` in `artemis-core/src/engine.rs` orchestrates all components using Tokio broadcast channels for event distribution.

## Essential Commands

### Building and Testing
```bash
# Build entire project
cargo build --release

# Run all tests
cargo test

# Run contract tests (requires Foundry)
just test-contracts

# Generate contract bindings
just bind

# Format code
just fmt

# Run clippy linting
cargo clippy --all-targets --all-features
```

### Smart Contract Development
```bash
# Compile contracts
forge build

# Test contracts with gas reporting
forge test --gas-report

# Generate Rust bindings
forge bind --bindings-path crates/

# Update protocol sources
just update-protocols
```

### Development Workflow
```bash
# Generate new strategy boilerplate
cargo run --bin generator -- --name YourStrategy

# Run specific strategy example
cargo run --bin opensea-sudoswap-strategy

# Run with environment configuration
cargo run --bin artemis -- --config-path config.toml
```

## Project Structure

```
artemis/
├── bin/                    # Main executables
│   ├── artemis/           # Primary bot binary
│   └── cli/               # Command-line interface
├── crates/
│   ├── artemis-core/      # Core framework (Engine, traits)
│   ├── clients/           # External API clients
│   ├── strategies/        # MEV strategy implementations
│   └── generator/         # Code generation utilities
├── contracts/             # Solidity smart contracts
└── examples/              # Usage examples and strategies
```

## Key Traits and Components

All strategies must implement the `Strategy` trait:
```rust
trait Strategy<E, A> {
    async fn sync_state(&mut self) -> Result<()>;
    async fn process_event(&mut self, event: E) -> Vec<A>;
}
```

Core components are defined in `artemis-core/src/types.rs`. The framework provides collectors for blocks, mempool, logs, OpenSea orders, and MEV-Share events.

## Adding New Strategies

1. Use the generator: `cargo run --bin generator -- --name YourStrategy`  
2. Implement the `Strategy` trait in `crates/strategies/your-strategy/`
3. Define custom `Event` and `Action` types
4. Add any required smart contracts in `contracts/`
5. Generate bindings with `just bind`
6. Add configuration to main binary

## Smart Contract Integration

- Uses Foundry for Solidity development
- Generates Rust bindings via `forge bind`
- State override middleware for simulation
- Protocol source management via `protocols.json`

## Testing

- **Unit tests**: Standard Rust testing in each crate
- **Integration tests**: End-to-end strategy testing
- **Fork tests**: Mainnet state simulation with `--fork-url`
- **Contract tests**: Foundry-based testing in `contracts/`

## Environment Configuration

Required environment variables:
- `ETHEREUM_RPC_URL`: Ethereum node RPC endpoint
- `PRIVATE_KEY`: Wallet private key for transaction signing
- `FLASHBOTS_RELAY_URL`: Flashbots relay endpoint (optional)

## Performance Considerations

- All components use async/await for non-blocking I/O
- Broadcast channels distribute events efficiently
- State synchronization prevents stale data
- Batch operations reduce RPC calls
- Gas price strategies ensure profitable execution

## Available Strategies

- **OpenSea-Sudoswap NFT Arbitrage**: Cross-market NFT arbitrage between Seaport and Sudoswap
- **MEV-Share Uniswap Arbitrage**: Private mempool arbitrage using MEV-Share protocol

## External Integrations

- **OpenSea V2 API**: Client in `crates/clients/opensea-v2/`
- **Chainbound Fiber**: Low-latency data feeds
- **MEV-Share**: Private mempool and bundle submission
- **Flashbots**: MEV relay integration
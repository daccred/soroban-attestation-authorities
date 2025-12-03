# Soroban Attestation Resolvers

A collection of Soroban smart contract implementations for the Stellar attestation service resolver interface.

## Project Structure

| Project | Description |
|---------|-------------|
| `resolvers/` | Core interface library - defines `ResolverInterface` trait and types |
| `authority/` | Payment-gated authority resolver - organizations pay 100 XLM for attestation eligibility |
| `airdrop/` | Airdrop resolver - distributes token rewards for attestations |
| `taxcollector/` | Tax collector resolver - collects fees before allowing attestations |

```
soroban-attestation-authorities/
├── Cargo.toml                 # Workspace configuration
├── README.md
│
├── resolvers/                 # Interface library (no contract)
│   ├── Cargo.toml
│   ├── Makefile
│   └── src/
│       ├── lib.rs
│       └── interface.rs       # ResolverInterface trait & types
│
├── authority/                 # Payment-gated resolver
│   ├── Cargo.toml
│   ├── Makefile
│   ├── README.md
│   └── src/
│       ├── lib.rs
│       ├── state.rs
│       ├── errors.rs
│       ├── events.rs
│       ├── access_control.rs
│       ├── payment.rs
│       └── instructions/
│
├── airdrop/                   # Token reward resolver
│   ├── Cargo.toml
│   ├── Makefile
│   ├── README.md
│   └── src/
│       └── lib.rs
│
└── taxcollector/              # Fee collection resolver
    ├── Cargo.toml
    ├── Makefile
    ├── README.md
    └── src/
        └── lib.rs
```

## Quick Start

### Prerequisites

- Rust toolchain with `wasm32v1-none` target
- Stellar CLI (`stellar`)

```bash
# Install wasm target
rustup target add wasm32v1-none
```

### Build All Contracts

```bash
# Build all workspace members
cargo build --target wasm32v1-none --release
```

### Build Individual Contract

```bash
cd authority && make build
cd airdrop && make build
cd taxcollector && make build
```

### Run Tests

```bash
# Run all tests
cargo test

# Run tests for a specific project
cd authority && make test
```

### Deploy

```bash
cd authority && make deploy IDENTITY=alice NETWORK=testnet
```

### Generate TypeScript Bindings

```bash
cd authority && make bindings CONTRACT_ID=CABC... NETWORK=testnet
```

## Resolver Interface

All resolvers implement the `ResolverInterface` trait from the `resolvers` crate:

```rust
pub trait ResolverInterface {
    /// Called before attestation creation - validation/authorization
    fn onattest(env: Env, attestation: ResolverAttestationData) -> Result<bool, ResolverError>;

    /// Called before attestation revocation - validation
    fn onrevoke(env: Env, attestation: ResolverAttestationData) -> Result<bool, ResolverError>;

    /// Called after attestation/revocation - post-processing/side effects
    fn onresolve(env: Env, attestation_uid: BytesN<32>, attester: Address) -> Result<(), ResolverError>;

    /// Returns resolver metadata
    fn metadata(env: Env) -> ResolverMetadata;
}
```

### Execution Flow

```
Protocol Attestation Flow:
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│ 1. onattest()   │───▶│ 2. Create        │───▶│ 3. onresolve()  │
│   Validation    │    │    Attestation   │    │   Post-process  │
└─────────────────┘    └──────────────────┘    └─────────────────┘

Protocol Revocation Flow:
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│ 1. onrevoke()   │───▶│ 2. Mark Revoked  │───▶│ 3. onresolve()  │
│   Validation    │    │    - Update state│    │   Post-process  │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## Creating a New Resolver

1. Create a new directory: `my-resolver/`
2. Add to workspace members in root `Cargo.toml`
3. Create `Cargo.toml` with dependency on `resolvers` crate:
   ```toml
   [dependencies]
   resolvers = { path = "../resolvers", default-features = false }
   ```
4. Implement `ResolverInterface` trait
5. Copy Makefile template from existing resolver

## Available Resolvers

### Authority Resolver (`authority/`)
Payment-gated authority verification. Organizations pay 100 XLM to become eligible for attestation, enabling controlled access to the attestation system.

### Airdrop Resolver (`airdrop/`)
Permissionless attestation with token rewards. Anyone can create attestations and receive token rewards from a managed pool, incentivizing attestation creation.

### Tax Collector Resolver (`taxcollector/`)
Fee-based attestation gating. Collects configurable fees before allowing attestations, enabling monetization of attestation services.

## License

MIT

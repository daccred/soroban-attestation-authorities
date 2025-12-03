# Fee Collection Resolver

A Soroban smart contract that implements the `ResolverInterface` for fee-gated attestation with configurable fee collection.

## Overview

This resolver implements a fee-based access model where:
- Attestations require payment of a configurable fee
- Fees are collected before attestation is allowed
- Fee recipient can withdraw accumulated fees
- Enables monetization of attestation services

## Business Model

```
┌─────────────────────────────────────────────────────────────────────────┐
│  FeeCollectionResolver                                                  │
│                                                                         │
│  1. Admin configures fee token, amount, and recipient                  │
│  2. User attempts to create attestation                                │
│  3. onattest() collects fee from attester                              │
│  4. Fee is credited to recipient's balance                             │
│  5. Recipient can withdraw accumulated fees                            │
└─────────────────────────────────────────────────────────────────────────┘
```

## Interface

### Constructor / Initialize

```rust
pub fn __constructor(
    env: Env,
    admin: Address,
    fee_token: Address,
    attestation_fee: i128,
    fee_recipient: Address,
)
```

- `admin` - Address that can manage the resolver
- `fee_token` - Token contract for fee collection
- `attestation_fee` - Fee amount per attestation
- `fee_recipient` - Address that receives fees

### Admin Functions

- `set_attestation_fee(admin, new_fee)` - Update fee amount
- `set_fee_recipient(admin, new_recipient)` - Update fee recipient

### User Functions

- `withdraw_fees(recipient)` - Withdraw collected fees (recipient only)

### Query Functions

- `get_total_collected()` - Total fees collected
- `get_collected_fees(recipient)` - Fees available for withdrawal

## Usage

### Build

```bash
make build
```

### Deploy

```bash
make deploy IDENTITY=alice NETWORK=testnet
```

### Generate Bindings

```bash
make bindings CONTRACT_ID=CABC... NETWORK=testnet
```

## Security Considerations

- **Fee Validation**: Fees must be non-negative
- **Authorization**: Fee changes require admin auth, withdrawals require recipient auth
- **Token Validation**: Constructor/initialize validates token contract
- **TTL Management**: Storage TTLs are extended to prevent expiration

## License

MIT

# Token Reward Resolver

A Soroban smart contract that implements the `ResolverInterface` for permissionless attestation with automatic token reward distribution.

## Overview

This resolver implements an open, permissionless model where:
- Anyone can create attestations (no access control)
- Each successful attestation triggers automatic token reward distribution
- Rewards are distributed from a managed token pool
- Gas costs provide natural spam resistance

## Business Model

```
┌─────────────────────────────────────────────────────────────────────────┐
│  TokenRewardResolver                                                    │
│                                                                         │
│  1. Admin funds reward pool with tokens                                │
│  2. User creates attestation (permissionless)                          │
│  3. Protocol calls onresolve() after attestation                       │
│  4. Resolver distributes reward tokens to attester                     │
│  5. Tracking updates: total rewarded, user rewards                     │
└─────────────────────────────────────────────────────────────────────────┘
```

## Interface

### Constructor / Initialize

```rust
pub fn __constructor(
    env: Env,
    admin: Address,
    reward_token: Address,
    reward_amount: i128,
    protocol_contract: Address,
)
```

- `admin` - Address that can manage the resolver
- `reward_token` - Token contract for rewards
- `reward_amount` - Tokens per attestation
- `protocol_contract` - Authorized protocol that can call onresolve

### Admin Functions

- `set_reward_amount(admin, new_amount)` - Update reward per attestation
- `fund_reward_pool(admin, amount)` - Add tokens to reward pool

### Query Functions

- `get_total_rewarded()` - Total rewards distributed
- `get_user_rewards(user)` - Rewards earned by specific user
- `get_pool_balance()` - Current reward pool balance

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

- **Replay Protection**: Each attestation UID can only receive rewards once
- **Authorization**: Only the protocol contract can call `onresolve`
- **Balance Check**: Verifies sufficient pool balance before distribution
- **Reentrancy Safe**: State updated before token transfer

## License

MIT

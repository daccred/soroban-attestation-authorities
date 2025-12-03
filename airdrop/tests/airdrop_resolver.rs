extern crate std;

use soroban_sdk::{
    testutils::{Address as _, BytesN as _, Ledger, LedgerInfo},
    token, Address, Bytes, BytesN, Env,
};

use airdrop::{TokenRewardResolver, TokenRewardResolverClient};
use resolvers::{ResolverAttestationData, ResolverType};

const REWARD_AMOUNT: i128 = 10_0000000; // 10 tokens per attestation

struct TestEnv {
    env: Env,
    admin: Address,
    protocol: Address,
    contract_id: Address,
    reward_token: Address,
}

fn setup_env() -> TestEnv {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().set(LedgerInfo {
        timestamp: 1000,
        protocol_version: 22,
        sequence_number: 0,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 16 * 60 * 60 * 24,
        min_persistent_entry_ttl: 30 * 60 * 60 * 24,
        max_entry_ttl: 365 * 60 * 60 * 24,
    });

    let admin = Address::generate(&env);
    let protocol = Address::generate(&env);

    // Create reward token
    let reward_token_contract = env.register_stellar_asset_contract_v2(admin.clone());
    let reward_token = reward_token_contract.address();

    // Register resolver contract with constructor arguments
    let contract_id = env.register(
        TokenRewardResolver,
        (&admin, &reward_token, &REWARD_AMOUNT, &protocol),
    );

    TestEnv {
        env,
        admin,
        protocol,
        contract_id,
        reward_token,
    }
}

fn build_attestation(env: &Env, attester: &Address) -> ResolverAttestationData {
    ResolverAttestationData {
        uid: BytesN::random(env),
        schema_uid: BytesN::random(env),
        recipient: Address::generate(env),
        attester: attester.clone(),
        time: env.ledger().timestamp(),
        expiration_time: 0,
        revocation_time: 0,
        revocable: true,
        ref_uid: Bytes::new(env),
        data: Bytes::new(env),
        value: 0,
    }
}

// ============================================================================
// Initialization Tests
// ============================================================================

#[test]
fn test_initialize() {
    let setup = setup_env();
    let client = TokenRewardResolverClient::new(&setup.env, &setup.contract_id);

    // Verify initial state
    assert_eq!(client.get_total_rewarded(), 0);
    assert_eq!(client.get_pool_balance(), 0);
}

#[test]
fn test_double_initialize_fails() {
    let setup = setup_env();
    let client = TokenRewardResolverClient::new(&setup.env, &setup.contract_id);

    // Already initialized via constructor, try to initialize again
    let new_admin = Address::generate(&setup.env);
    let result = client.try_initialize(
        &new_admin,
        &setup.reward_token,
        &REWARD_AMOUNT,
        &setup.protocol,
    );

    assert!(result.is_err());
}

#[test]
#[should_panic(expected = "reward_amount must be non-negative")]
fn test_initialize_negative_reward_fails() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().set(LedgerInfo {
        timestamp: 1000,
        protocol_version: 22,
        sequence_number: 0,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 16 * 60 * 60 * 24,
        min_persistent_entry_ttl: 30 * 60 * 60 * 24,
        max_entry_ttl: 365 * 60 * 60 * 24,
    });

    let admin = Address::generate(&env);
    let protocol = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(admin.clone());
    let token = token_contract.address();

    // This should panic in the constructor
    let _contract_id = env.register(
        TokenRewardResolver,
        (&admin, &token, &-100i128, &protocol),
    );
}

// ============================================================================
// Admin Function Tests
// ============================================================================

#[test]
fn test_set_reward_amount() {
    let setup = setup_env();
    let client = TokenRewardResolverClient::new(&setup.env, &setup.contract_id);

    let new_amount: i128 = 20_0000000;
    client.set_reward_amount(&setup.admin, &new_amount);

    // Verify by checking metadata or through reward distribution
    // The amount is stored internally
}

#[test]
fn test_set_reward_amount_non_admin_fails() {
    let setup = setup_env();
    let client = TokenRewardResolverClient::new(&setup.env, &setup.contract_id);

    let non_admin = Address::generate(&setup.env);
    let result = client.try_set_reward_amount(&non_admin, &20_0000000);

    assert!(result.is_err());
}

#[test]
fn test_set_negative_reward_amount_fails() {
    let setup = setup_env();
    let client = TokenRewardResolverClient::new(&setup.env, &setup.contract_id);

    let result = client.try_set_reward_amount(&setup.admin, &-100);
    assert!(result.is_err());
}

#[test]
fn test_fund_reward_pool() {
    let setup = setup_env();
    let env = &setup.env;
    let client = TokenRewardResolverClient::new(env, &setup.contract_id);

    // Mint tokens to admin
    let token_admin = token::StellarAssetClient::new(env, &setup.reward_token);
    let fund_amount: i128 = 1000_0000000;
    token_admin.mint(&setup.admin, &fund_amount);

    // Fund the pool
    client.fund_reward_pool(&setup.admin, &fund_amount);

    // Verify pool balance
    assert_eq!(client.get_pool_balance(), fund_amount);
}

#[test]
fn test_fund_reward_pool_non_admin_fails() {
    let setup = setup_env();
    let env = &setup.env;
    let client = TokenRewardResolverClient::new(env, &setup.contract_id);

    let non_admin = Address::generate(env);
    let token_admin = token::StellarAssetClient::new(env, &setup.reward_token);
    token_admin.mint(&non_admin, &1000_0000000);

    let result = client.try_fund_reward_pool(&non_admin, &1000_0000000);
    assert!(result.is_err());
}

// ============================================================================
// Query Function Tests
// ============================================================================

#[test]
fn test_get_user_rewards_uninitialized() {
    let setup = setup_env();
    let client = TokenRewardResolverClient::new(&setup.env, &setup.contract_id);

    let user = Address::generate(&setup.env);
    assert_eq!(client.get_user_rewards(&user), 0);
}

#[test]
fn test_get_pool_balance_empty() {
    let setup = setup_env();
    let client = TokenRewardResolverClient::new(&setup.env, &setup.contract_id);

    assert_eq!(client.get_pool_balance(), 0);
}

// ============================================================================
// Resolver Interface Tests
// ============================================================================

#[test]
fn test_onattest_always_allows() {
    let setup = setup_env();
    let client = TokenRewardResolverClient::new(&setup.env, &setup.contract_id);

    let attester = Address::generate(&setup.env);
    let attestation = build_attestation(&setup.env, &attester);

    // onattest should always return true (permissionless)
    let result = client.onattest(&attestation);
    assert!(result);
}

#[test]
fn test_onrevoke_always_allows() {
    let setup = setup_env();
    let client = TokenRewardResolverClient::new(&setup.env, &setup.contract_id);

    let attester = Address::generate(&setup.env);
    let attestation = build_attestation(&setup.env, &attester);

    let result = client.onrevoke(&attestation);
    assert!(result);
}

#[test]
fn test_onresolve_distributes_rewards() {
    let setup = setup_env();
    let env = &setup.env;
    let client = TokenRewardResolverClient::new(env, &setup.contract_id);

    // Fund the pool
    let token_admin = token::StellarAssetClient::new(env, &setup.reward_token);
    let fund_amount: i128 = 1000_0000000;
    token_admin.mint(&setup.admin, &fund_amount);
    client.fund_reward_pool(&setup.admin, &fund_amount);

    // Create attestation
    let attester = Address::generate(env);
    let attestation_uid = BytesN::random(env);

    // Call onresolve (protocol must be caller)
    client.onresolve(&attestation_uid, &attester);

    // Verify rewards distributed
    let token_client = token::Client::new(env, &setup.reward_token);
    assert_eq!(token_client.balance(&attester), REWARD_AMOUNT);
    assert_eq!(client.get_user_rewards(&attester), REWARD_AMOUNT);
    assert_eq!(client.get_total_rewarded(), REWARD_AMOUNT);
    assert_eq!(client.get_pool_balance(), fund_amount - REWARD_AMOUNT);
}

#[test]
fn test_onresolve_insufficient_funds() {
    let setup = setup_env();
    let env = &setup.env;
    let client = TokenRewardResolverClient::new(env, &setup.contract_id);

    // Don't fund the pool
    let attester = Address::generate(env);
    let attestation_uid = BytesN::random(env);

    // Should fail due to insufficient funds
    let result = client.try_onresolve(&attestation_uid, &attester);
    assert!(result.is_err());
}

#[test]
fn test_onresolve_replay_protection() {
    let setup = setup_env();
    let env = &setup.env;
    let client = TokenRewardResolverClient::new(env, &setup.contract_id);

    // Fund the pool with enough for multiple rewards
    let token_admin = token::StellarAssetClient::new(env, &setup.reward_token);
    let fund_amount: i128 = 1000_0000000;
    token_admin.mint(&setup.admin, &fund_amount);
    client.fund_reward_pool(&setup.admin, &fund_amount);

    // Create attestation
    let attester = Address::generate(env);
    let attestation_uid = BytesN::random(env);

    // First call should succeed
    client.onresolve(&attestation_uid, &attester);
    assert_eq!(client.get_user_rewards(&attester), REWARD_AMOUNT);

    // Second call with same UID should silently succeed but not distribute
    client.onresolve(&attestation_uid, &attester);
    assert_eq!(client.get_user_rewards(&attester), REWARD_AMOUNT); // Still same amount
    assert_eq!(client.get_total_rewarded(), REWARD_AMOUNT); // Still same total
}

#[test]
fn test_onresolve_zero_reward_amount() {
    let setup = setup_env();
    let env = &setup.env;
    let client = TokenRewardResolverClient::new(env, &setup.contract_id);

    // Set reward amount to 0
    client.set_reward_amount(&setup.admin, &0);

    let attester = Address::generate(env);
    let attestation_uid = BytesN::random(env);

    // Should succeed silently with no distribution
    client.onresolve(&attestation_uid, &attester);
    assert_eq!(client.get_user_rewards(&attester), 0);
}

#[test]
fn test_metadata() {
    let setup = setup_env();
    let client = TokenRewardResolverClient::new(&setup.env, &setup.contract_id);

    let metadata = client.metadata();
    assert_eq!(metadata.resolver_type, ResolverType::TokenReward);
}

// ============================================================================
// Multiple Attestations Test
// ============================================================================

#[test]
fn test_multiple_attestations_multiple_users() {
    let setup = setup_env();
    let env = &setup.env;
    let client = TokenRewardResolverClient::new(env, &setup.contract_id);

    // Fund the pool
    let token_admin = token::StellarAssetClient::new(env, &setup.reward_token);
    let fund_amount: i128 = 1000_0000000;
    token_admin.mint(&setup.admin, &fund_amount);
    client.fund_reward_pool(&setup.admin, &fund_amount);

    // Multiple users get rewards
    let user1 = Address::generate(env);
    let user2 = Address::generate(env);
    let user3 = Address::generate(env);

    client.onresolve(&BytesN::random(env), &user1);
    client.onresolve(&BytesN::random(env), &user2);
    client.onresolve(&BytesN::random(env), &user3);

    // Verify each user got rewards
    assert_eq!(client.get_user_rewards(&user1), REWARD_AMOUNT);
    assert_eq!(client.get_user_rewards(&user2), REWARD_AMOUNT);
    assert_eq!(client.get_user_rewards(&user3), REWARD_AMOUNT);
    assert_eq!(client.get_total_rewarded(), REWARD_AMOUNT * 3);
}

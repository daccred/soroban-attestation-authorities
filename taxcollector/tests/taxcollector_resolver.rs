extern crate std;

use soroban_sdk::{
    testutils::{Address as _, BytesN as _, Ledger, LedgerInfo},
    token, Address, Bytes, BytesN, Env,
};

use resolvers::{ResolverAttestationData, ResolverType};
use taxcollector::{FeeCollectionResolver, FeeCollectionResolverClient};

const ATTESTATION_FEE: i128 = 5_0000000; // 5 tokens per attestation

struct TestEnv {
    env: Env,
    admin: Address,
    fee_recipient: Address,
    contract_id: Address,
    fee_token: Address,
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
    let fee_recipient = Address::generate(&env);

    // Create fee token
    let fee_token_contract = env.register_stellar_asset_contract_v2(admin.clone());
    let fee_token = fee_token_contract.address();

    // Register resolver contract with constructor arguments
    let contract_id = env.register(
        FeeCollectionResolver,
        (&admin, &fee_token, &ATTESTATION_FEE, &fee_recipient),
    );

    TestEnv {
        env,
        admin,
        fee_recipient,
        contract_id,
        fee_token,
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
    let client = FeeCollectionResolverClient::new(&setup.env, &setup.contract_id);

    // Verify initial state
    assert_eq!(client.get_total_collected(), 0);
    assert_eq!(client.get_collected_fees(&setup.fee_recipient), 0);
}

#[test]
fn test_double_initialize_fails() {
    let setup = setup_env();
    let client = FeeCollectionResolverClient::new(&setup.env, &setup.contract_id);

    // Already initialized via constructor, try to initialize again
    let new_admin = Address::generate(&setup.env);
    let new_recipient = Address::generate(&setup.env);
    let result = client.try_initialize(
        &new_admin,
        &setup.fee_token,
        &ATTESTATION_FEE,
        &new_recipient,
    );

    assert!(result.is_err());
}

#[test]
#[should_panic(expected = "attestation_fee must be non-negative")]
fn test_initialize_negative_fee_fails() {
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
    let recipient = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(admin.clone());
    let token = token_contract.address();

    // This should panic in the constructor
    let _contract_id = env.register(
        FeeCollectionResolver,
        (&admin, &token, &-100i128, &recipient),
    );
}

// ============================================================================
// Admin Function Tests
// ============================================================================

#[test]
fn test_set_attestation_fee() {
    let setup = setup_env();
    let client = FeeCollectionResolverClient::new(&setup.env, &setup.contract_id);

    let new_fee: i128 = 10_0000000;
    client.set_attestation_fee(&setup.admin, &new_fee);

    // Fee is stored internally - verify through attestation
}

#[test]
fn test_set_attestation_fee_non_admin_fails() {
    let setup = setup_env();
    let client = FeeCollectionResolverClient::new(&setup.env, &setup.contract_id);

    let non_admin = Address::generate(&setup.env);
    let result = client.try_set_attestation_fee(&non_admin, &10_0000000);

    assert!(result.is_err());
}

#[test]
fn test_set_negative_fee_fails() {
    let setup = setup_env();
    let client = FeeCollectionResolverClient::new(&setup.env, &setup.contract_id);

    let result = client.try_set_attestation_fee(&setup.admin, &-100);
    assert!(result.is_err());
}

#[test]
fn test_set_fee_recipient() {
    let setup = setup_env();
    let client = FeeCollectionResolverClient::new(&setup.env, &setup.contract_id);

    let new_recipient = Address::generate(&setup.env);
    client.set_fee_recipient(&setup.admin, &new_recipient);

    // Recipient is stored internally
}

#[test]
fn test_set_fee_recipient_non_admin_fails() {
    let setup = setup_env();
    let client = FeeCollectionResolverClient::new(&setup.env, &setup.contract_id);

    let non_admin = Address::generate(&setup.env);
    let new_recipient = Address::generate(&setup.env);
    let result = client.try_set_fee_recipient(&non_admin, &new_recipient);

    assert!(result.is_err());
}

// ============================================================================
// Fee Collection Tests
// ============================================================================

#[test]
fn test_onattest_collects_fee() {
    let setup = setup_env();
    let env = &setup.env;
    let client = FeeCollectionResolverClient::new(env, &setup.contract_id);

    // Mint tokens to attester
    let attester = Address::generate(env);
    let token_admin = token::StellarAssetClient::new(env, &setup.fee_token);
    token_admin.mint(&attester, &ATTESTATION_FEE);

    // Create attestation
    let attestation = build_attestation(env, &attester);

    // onattest should collect fee
    let result = client.onattest(&attestation);
    assert!(result);

    // Verify fee was collected
    let token_client = token::Client::new(env, &setup.fee_token);
    assert_eq!(token_client.balance(&attester), 0);
    assert_eq!(token_client.balance(&setup.contract_id), ATTESTATION_FEE);
    assert_eq!(client.get_total_collected(), ATTESTATION_FEE);
    assert_eq!(
        client.get_collected_fees(&setup.fee_recipient),
        ATTESTATION_FEE
    );
}

#[test]
fn test_onattest_insufficient_funds_fails() {
    let setup = setup_env();
    let env = &setup.env;
    let client = FeeCollectionResolverClient::new(env, &setup.contract_id);

    // Attester has no tokens
    let attester = Address::generate(env);
    let attestation = build_attestation(env, &attester);

    // Should fail due to insufficient funds
    let result = client.try_onattest(&attestation);
    assert!(result.is_err());
}

#[test]
fn test_onattest_zero_fee_no_transfer() {
    let setup = setup_env();
    let env = &setup.env;
    let client = FeeCollectionResolverClient::new(env, &setup.contract_id);

    // Set fee to 0
    client.set_attestation_fee(&setup.admin, &0);

    // Attester with no tokens should succeed
    let attester = Address::generate(env);
    let attestation = build_attestation(env, &attester);

    let result = client.onattest(&attestation);
    assert!(result);
    assert_eq!(client.get_total_collected(), 0);
}

#[test]
fn test_multiple_attestations_accumulate_fees() {
    let setup = setup_env();
    let env = &setup.env;
    let client = FeeCollectionResolverClient::new(env, &setup.contract_id);

    let token_admin = token::StellarAssetClient::new(env, &setup.fee_token);

    // Multiple attesters pay fees
    for _ in 0..3 {
        let attester = Address::generate(env);
        token_admin.mint(&attester, &ATTESTATION_FEE);
        let attestation = build_attestation(env, &attester);
        client.onattest(&attestation);
    }

    // Verify accumulated fees
    assert_eq!(client.get_total_collected(), ATTESTATION_FEE * 3);
    assert_eq!(
        client.get_collected_fees(&setup.fee_recipient),
        ATTESTATION_FEE * 3
    );
}

// ============================================================================
// Fee Withdrawal Tests
// ============================================================================

#[test]
fn test_withdraw_fees() {
    let setup = setup_env();
    let env = &setup.env;
    let client = FeeCollectionResolverClient::new(env, &setup.contract_id);

    // Collect some fees first
    let attester = Address::generate(env);
    let token_admin = token::StellarAssetClient::new(env, &setup.fee_token);
    token_admin.mint(&attester, &ATTESTATION_FEE);
    let attestation = build_attestation(env, &attester);
    client.onattest(&attestation);

    // Withdraw fees
    client.withdraw_fees(&setup.fee_recipient);

    // Verify withdrawal
    let token_client = token::Client::new(env, &setup.fee_token);
    assert_eq!(token_client.balance(&setup.fee_recipient), ATTESTATION_FEE);
    assert_eq!(token_client.balance(&setup.contract_id), 0);
    assert_eq!(client.get_collected_fees(&setup.fee_recipient), 0);
}

#[test]
fn test_withdraw_fees_non_recipient_fails() {
    let setup = setup_env();
    let env = &setup.env;
    let client = FeeCollectionResolverClient::new(env, &setup.contract_id);

    // Collect some fees first
    let attester = Address::generate(env);
    let token_admin = token::StellarAssetClient::new(env, &setup.fee_token);
    token_admin.mint(&attester, &ATTESTATION_FEE);
    let attestation = build_attestation(env, &attester);
    client.onattest(&attestation);

    // Non-recipient tries to withdraw
    let non_recipient = Address::generate(env);
    let result = client.try_withdraw_fees(&non_recipient);

    assert!(result.is_err());
}

#[test]
fn test_withdraw_fees_nothing_to_withdraw() {
    let setup = setup_env();
    let client = FeeCollectionResolverClient::new(&setup.env, &setup.contract_id);

    // Should succeed silently with nothing to withdraw
    client.withdraw_fees(&setup.fee_recipient);

    let token_client = token::Client::new(&setup.env, &setup.fee_token);
    assert_eq!(token_client.balance(&setup.fee_recipient), 0);
}

// ============================================================================
// Resolver Interface Tests
// ============================================================================

#[test]
fn test_onrevoke_always_allows() {
    let setup = setup_env();
    let client = FeeCollectionResolverClient::new(&setup.env, &setup.contract_id);

    let attester = Address::generate(&setup.env);
    let attestation = build_attestation(&setup.env, &attester);

    let result = client.onrevoke(&attestation);
    assert!(result);
}

#[test]
fn test_onresolve_no_op() {
    let setup = setup_env();
    let client = FeeCollectionResolverClient::new(&setup.env, &setup.contract_id);

    let attester = Address::generate(&setup.env);
    let attestation_uid = BytesN::random(&setup.env);

    // Should succeed silently (no-op)
    client.onresolve(&attestation_uid, &attester);
}

#[test]
fn test_metadata() {
    let setup = setup_env();
    let client = FeeCollectionResolverClient::new(&setup.env, &setup.contract_id);

    let metadata = client.metadata();
    assert_eq!(metadata.resolver_type, ResolverType::FeeCollection);
}

// ============================================================================
// Fee Recipient Change Tests
// ============================================================================

#[test]
fn test_change_recipient_mid_collection() {
    let setup = setup_env();
    let env = &setup.env;
    let client = FeeCollectionResolverClient::new(env, &setup.contract_id);

    let token_admin = token::StellarAssetClient::new(env, &setup.fee_token);

    // First attester pays to original recipient
    let attester1 = Address::generate(env);
    token_admin.mint(&attester1, &ATTESTATION_FEE);
    client.onattest(&build_attestation(env, &attester1));

    // Change recipient
    let new_recipient = Address::generate(env);
    client.set_fee_recipient(&setup.admin, &new_recipient);

    // Second attester pays to new recipient
    let attester2 = Address::generate(env);
    token_admin.mint(&attester2, &ATTESTATION_FEE);
    client.onattest(&build_attestation(env, &attester2));

    // Verify fees are tracked separately
    assert_eq!(
        client.get_collected_fees(&setup.fee_recipient),
        ATTESTATION_FEE
    );
    assert_eq!(client.get_collected_fees(&new_recipient), ATTESTATION_FEE);
    assert_eq!(client.get_total_collected(), ATTESTATION_FEE * 2);
}

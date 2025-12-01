import { Buffer } from "buffer";
import { AssembledTransaction, Client as ContractClient, ClientOptions as ContractClientOptions, MethodOptions, Result } from '@stellar/stellar-sdk/contract';
import type { u64, i128, Option } from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk';
export * as contract from '@stellar/stellar-sdk/contract';
export * as rpc from '@stellar/stellar-sdk/rpc';
export declare const networks: {
    readonly testnet: {
        readonly networkPassphrase: "Test SDF Network ; September 2015";
        readonly contractId: "CCMJGCRSQRZ56BDSLCAYV4BNS3SLIDPIP4CQYNT5X2VOPZIQ2ZM7GBVV";
    };
    readonly mainnet: {
        readonly networkPassphrase: "Public Global Stellar Network ; September 2015";
        readonly contractId: "CBKOB6XEEXYH5SEFQ4YSUEFJGYNBVISQBHQHVGCKB736A3JVGK7F77JG";
    };
};
export type DataKey = {
    tag: "Admin";
    values: void;
} | {
    tag: "Initialized";
    values: void;
} | {
    tag: "RegistrationFee";
    values: void;
} | {
    tag: "PaymentRecord";
    values: void;
} | {
    tag: "Authority";
    values: void;
} | {
    tag: "TokenId";
    values: void;
} | {
    tag: "TokenWasmHash";
    values: void;
} | {
    tag: "CollectedLevies";
    values: void;
} | {
    tag: "CollectedFees";
    values: void;
} | {
    tag: "RegAuthPrefix";
    values: void;
} | {
    tag: "CollLevyPrefix";
    values: void;
};
export interface Attestation {
    attester: string;
    data: Buffer;
    expiration_time: Option<u64>;
    recipient: string;
    ref_uid: Option<Buffer>;
    revocable: boolean;
    schema_uid: Buffer;
    time: u64;
    uid: Buffer;
    value: Option<i128>;
}
export interface PaymentRecord {
    amount_paid: i128;
    recipient: string;
    ref_id: string;
    timestamp: u64;
}
export interface RegisteredAuthorityData {
    address: string;
    metadata: string;
    ref_id: string;
    registration_time: u64;
}
export declare const Errors: {
    1: {
        message: string;
    };
    2: {
        message: string;
    };
    3: {
        message: string;
    };
    4: {
        message: string;
    };
    5: {
        message: string;
    };
    6: {
        message: string;
    };
    7: {
        message: string;
    };
    8: {
        message: string;
    };
    9: {
        message: string;
    };
    10: {
        message: string;
    };
    11: {
        message: string;
    };
    12: {
        message: string;
    };
    13: {
        message: string;
    };
    14: {
        message: string;
    };
    15: {
        message: string;
    };
    16: {
        message: string;
    };
    17: {
        message: string;
    };
};
export type ResolverType = {
    tag: "Default";
    values: void;
} | {
    tag: "Authority";
    values: void;
} | {
    tag: "TokenReward";
    values: void;
} | {
    tag: "FeeCollection";
    values: void;
} | {
    tag: "Hybrid";
    values: void;
} | {
    tag: "Staking";
    values: void;
} | {
    tag: "Custom";
    values: void;
};
export declare const ResolverError: {
    1: {
        message: string;
    };
    2: {
        message: string;
    };
    3: {
        message: string;
    };
    4: {
        message: string;
    };
    5: {
        message: string;
    };
    6: {
        message: string;
    };
    7: {
        message: string;
    };
    8: {
        message: string;
    };
};
export interface ResolverMetadata {
    description: string;
    name: string;
    resolver_type: ResolverType;
    version: string;
}
export interface ResolverAttestationData {
    attester: string;
    data: Buffer;
    expiration_time: u64;
    recipient: string;
    ref_uid: Buffer;
    revocable: boolean;
    revocation_time: u64;
    schema_uid: Buffer;
    time: u64;
    uid: Buffer;
    value: i128;
}
export interface Client {
    attest: ({ attestation }: {
        attestation: Attestation;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<boolean>>>;
    revoke: ({ attestation }: {
        attestation: Attestation;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<boolean>>>;
    is_owner: ({ address }: {
        address: string;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<boolean>>;
    onattest: ({ attestation }: {
        attestation: ResolverAttestationData;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<boolean>>>;
    get_owner: (options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<string>>>;
    onresolve: ({ attestation }: {
        attestation: ResolverAttestationData;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<void>>>;
    initialize: ({ admin, token_contract_id, token_wasm_hash }: {
        admin: string;
        token_contract_id: string;
        token_wasm_hash: Buffer;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<void>>>;
    get_token_id: (options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<string>>>;
    is_authority: ({ authority }: {
        authority: string;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<boolean>>>;
    withdraw_fees: ({ caller }: {
        caller: string;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<void>>>;
    withdraw_levies: ({ caller }: {
        caller: string;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<void>>>;
    get_admin_address: (options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<string>>>;
    get_collected_fees: ({ authority }: {
        authority: string;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<i128>>>;
    get_payment_record: ({ payer }: {
        payer: string;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Option<PaymentRecord>>>;
    register_authority: ({ caller, authority_to_reg, metadata }: {
        caller: string;
        authority_to_reg: string;
        metadata: string;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<void>>>;
    renounce_ownership: ({ current_owner }: {
        current_owner: string;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<void>>>;
    transfer_ownership: ({ current_owner, new_owner }: {
        current_owner: string;
        new_owner: string;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<void>>>;
    admin_withdraw_fees: ({ admin, token_address, amount }: {
        admin: string;
        token_address: string;
        amount: i128;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<void>>>;
    get_collected_levies: ({ authority }: {
        authority: string;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<i128>>>;
    pay_verification_fee: ({ payer, ref_id, token_address }: {
        payer: string;
        ref_id: string;
        token_address: string;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<void>>>;
    has_confirmed_payment: ({ payer }: {
        payer: string;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<boolean>>;
    admin_register_authority: ({ admin, auth_to_reg, metadata }: {
        admin: string;
        auth_to_reg: string;
        metadata: string;
    }, options?: {
        fee?: number;
        timeoutInSeconds?: number;
        simulate?: boolean;
    }) => Promise<AssembledTransaction<Result<void>>>;
}
export declare class Client extends ContractClient {
    readonly options: ContractClientOptions;
    static deploy<T = Client>(options: MethodOptions & Omit<ContractClientOptions, "contractId"> & {
        wasmHash: Buffer | string;
        salt?: Buffer | Uint8Array;
        format?: "hex" | "base64";
    }): Promise<AssembledTransaction<T>>;
    constructor(options: ContractClientOptions);
    readonly fromJSON: {
        attest: (json: string) => AssembledTransaction<Result<boolean, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        revoke: (json: string) => AssembledTransaction<Result<boolean, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        is_owner: (json: string) => AssembledTransaction<boolean>;
        onattest: (json: string) => AssembledTransaction<Result<boolean, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        get_owner: (json: string) => AssembledTransaction<Result<string, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        onresolve: (json: string) => AssembledTransaction<Result<void, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        initialize: (json: string) => AssembledTransaction<Result<void, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        get_token_id: (json: string) => AssembledTransaction<Result<string, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        is_authority: (json: string) => AssembledTransaction<Result<boolean, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        withdraw_fees: (json: string) => AssembledTransaction<Result<void, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        withdraw_levies: (json: string) => AssembledTransaction<Result<void, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        get_admin_address: (json: string) => AssembledTransaction<Result<string, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        get_collected_fees: (json: string) => AssembledTransaction<Result<bigint, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        get_payment_record: (json: string) => AssembledTransaction<Option<PaymentRecord>>;
        register_authority: (json: string) => AssembledTransaction<Result<void, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        renounce_ownership: (json: string) => AssembledTransaction<Result<void, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        transfer_ownership: (json: string) => AssembledTransaction<Result<void, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        admin_withdraw_fees: (json: string) => AssembledTransaction<Result<void, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        get_collected_levies: (json: string) => AssembledTransaction<Result<bigint, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        pay_verification_fee: (json: string) => AssembledTransaction<Result<void, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        has_confirmed_payment: (json: string) => AssembledTransaction<boolean>;
        admin_register_authority: (json: string) => AssembledTransaction<Result<void, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
    };
}
//# sourceMappingURL=authority.d.ts.map
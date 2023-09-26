use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{system_program, sysvar::SysvarId};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

pub fn lock_tokens(
    program_id: Pubkey,
    instruction_data: &InstructionLockToken,
    admin: Pubkey,
    token_mint: Pubkey,
    bridge_state: Pubkey,
    bridge_token_account: Pubkey,
    admin_token_account: Pubkey,
    gari_treasury_account: Pubkey,
) -> Instruction {
    Instruction::new_with_borsh(
        program_id,
        instruction_data,
        vec![
            // 0. `[]` admin's account
            AccountMeta::new(admin, true),
            // 1. `[]` token mint
            AccountMeta::new_readonly(token_mint, false),
            // 2. `[writable]` bridge state account
            // db / find_program_address
            AccountMeta::new(bridge_state, false),
            // 3. `[writable]` user_mandate_state account
            // db / find_program_address
            AccountMeta::new(bridge_token_account, false),
            // 4. '[writable]' admin_token_account
            AccountMeta::new(admin_token_account, false),
            // 5. '[writable]' gari_trassury_account
            AccountMeta::new_readonly(gari_treasury_account, false),
            // 6. `[]` Token Program
            AccountMeta::new_readonly(spl_token::id(), false),
            // 7. `[]` System program
            AccountMeta::new_readonly(system_program::id(), false),
        ],
    )
}

pub fn claim_locked_tokens(
    program_id: Pubkey,
    instruction_data: &InstructionUnlockToken,
    admin: Pubkey,
    token_mint: Pubkey,
    bridge_state: Pubkey,
    claimed_txn_state: Pubkey,
    bridge_token_account: Pubkey,
    admin_token_account: Pubkey,
) -> Instruction {
    Instruction::new_with_borsh(
        program_id,
        instruction_data,
        vec![
            // 0. `[]` admin's account
            AccountMeta::new(admin, true),
            // 1. `[]` token mint
            AccountMeta::new_readonly(token_mint, false),
            // 2. `[writable]` bridge state account
            // db / find_program_address
            AccountMeta::new(bridge_state, false),
            // 3. `[writable]` claimed transaction state
            // db / find_program_address
            AccountMeta::new(claimed_txn_state, false),
            // 4. `[writable]` user_mandate_state account
            // db / find_program_address
            AccountMeta::new(bridge_token_account, false),
            // 5. '[writable]' user_token_account
            AccountMeta::new(admin_token_account, false),
            // 6. `[]` Token Program
            AccountMeta::new_readonly(spl_token::id(), false),
            // 7. `[]` System program
            AccountMeta::new_readonly(system_program::id(), false),
        ],
    )
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InstructionUnlockToken {
    pub instruction_type: [u8; 8],
    pub amount: u64,
    pub transaction_nonce: u64,
    pub lock_transaction_hash: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InstructionLockToken {
    pub instruction_type: [u8; 8],
    pub amount: u64,
    pub mantle_gas_fee: u64,
    pub receiver_address: String,
}

// #[derive(BorshSerialize, BorshDeserialize, Debug)]
// pub struct InstructionCreateBridgeState{
//     pub instruction_type: [u8;8],
//     pub chain: String,
//     pub name: String,
// }

pub const SOL_ADDRESS: &str = "11111111111111111111111111111111";

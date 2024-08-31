use sha2::{Digest, Sha256};
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{
        ConfirmedTransaction, Message, Transaction, TransactionStatusMeta,
    },
};
use substreams_solana_program_instructions::token_instruction_2022::TokenInstruction;

pub fn _txn_pre_checks(
    confirmed_txn: &ConfirmedTransaction,
) -> Option<(&Transaction, &TransactionStatusMeta, &Message)> {
    let txn = match confirmed_txn.transaction.as_ref() {
        Some(txn) => txn,
        None => return None,
    };

    let txn_meta = match confirmed_txn.meta.as_ref() {
        Some(meta) => {
            if meta.err.is_some() {
                return None;
            }
            meta
        }
        None => return None,
    };

    let txn_messages = match txn.message.as_ref() {
        Some(msg) => msg,
        None => return None,
    };

    Some((txn, txn_meta, txn_messages))
}

pub fn _idl_discriminator(inst_name: &str) -> [u8; 8] {
    let preimage = format!("global:{}", inst_name);
    let hash = Sha256::digest(preimage.as_bytes());

    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(&hash[..8]);
    sighash
}

#[allow(deprecated)]
pub fn get_transfer_amount(instruction: Option<&InstructionView>) -> Option<String> {
    instruction?;

    let data = instruction.unwrap().data();

    match TokenInstruction::unpack(data).unwrap() {
        TokenInstruction::Transfer { amount } => Some(amount.to_string()),
        _ => None,
    }
}

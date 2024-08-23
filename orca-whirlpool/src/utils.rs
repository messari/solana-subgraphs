use sha2::{Digest, Sha256};
use substreams_solana::pb::sf::solana::r#type::v1::{
    ConfirmedTransaction, Message, Transaction, TransactionStatusMeta,
};

pub fn txn_pre_checks(
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

pub fn idl_discriminator(inst_name: &str) -> [u8; 8] {
    let preimage = format!("global:{}", inst_name);
    let hash = Sha256::digest(preimage.as_bytes());

    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(&hash[..8]);
    sighash
}

pub fn string_to_bigint(str: String) -> substreams::scalar::BigInt {
    str.parse::<substreams::scalar::BigInt>()
        .expect("failed to parse str")
}

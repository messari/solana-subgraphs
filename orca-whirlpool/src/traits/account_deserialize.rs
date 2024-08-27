use substreams_solana::pb::sf::solana::r#type::v1::{CompiledInstruction, ConfirmedTransaction};

pub trait AccountsDeserialize<'a> {
    fn deserialize(
        confirmed_txn: &'a ConfirmedTransaction,
        compiled_instr: &'a CompiledInstruction,
    ) -> Option<Self>
    where
        Self: Sized;
}

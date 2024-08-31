use substreams_solana::block_view::InstructionView;

pub trait AccountsDeserialize<'a> {
    fn deserialize(instruction_view: &'a InstructionView) -> Option<Self>
    where
        Self: Sized;
}

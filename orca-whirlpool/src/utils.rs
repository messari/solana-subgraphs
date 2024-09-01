use substreams_solana::block_view::InstructionView;
use substreams_solana_program_instructions::token_instruction_2022::TokenInstruction;

#[allow(deprecated)]
pub fn get_transfer_amount(instruction: Option<&InstructionView>) -> Option<String> {
    instruction?;

    let data = instruction.unwrap().data();

    match TokenInstruction::unpack(data).unwrap() {
        TokenInstruction::Transfer { amount } => Some(amount.to_string()),
        _ => None,
    }
}

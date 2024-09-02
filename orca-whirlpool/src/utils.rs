use substreams::scalar::BigInt;
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

pub fn balance_difference(
    pre_balance: Option<String>,
    post_balance: Option<String>,
) -> Option<String> {
    let pre_balance_value = pre_balance.unwrap_or("0".to_string());
    let post_balance_value = post_balance.unwrap_or("0".to_string());

    let pre_balance_bigint = BigInt::try_from(&pre_balance_value).unwrap();
    let post_balance_bigint = BigInt::try_from(&post_balance_value).unwrap();

    let balance_difference = post_balance_bigint - pre_balance_bigint;

    Some(balance_difference.to_string())
}

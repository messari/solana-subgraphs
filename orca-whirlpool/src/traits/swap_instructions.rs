use crate::{
    constants::ZERO_STRING,
    pb::messari::orca_whirlpool::v1::{OrcaSwap, OrcaSwapV2, TwoHopSwap, TwoHopSwapV2},
};

pub trait SwapInstruction {
    fn a_to_b(&self) -> bool;
    fn amount_a(&self) -> String;
    fn amount_b(&self) -> String;
    fn amount_a_post(&self) -> String;
    fn amount_b_post(&self) -> String;
    fn token_authority(&self) -> String;
    fn whirlpool(&self) -> String;
    fn is_two_hop(&self) -> bool;
    fn second_hop(&self) -> Option<Box<dyn SwapInstruction>>;
}

impl SwapInstruction for TwoHopSwap {
    fn a_to_b(&self) -> bool {
        self.instruction.as_ref().map_or(false, |i| i.a_to_b_one)
    }

    fn amount_a(&self) -> String {
        self.instruction
            .as_ref()
            .and_then(|i| i.amount_a_one.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn amount_b(&self) -> String {
        self.instruction
            .as_ref()
            .and_then(|i| i.amount_b_one.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn amount_a_post(&self) -> String {
        self.instruction
            .as_ref()
            .and_then(|i| i.amount_a_one_post.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn amount_b_post(&self) -> String {
        self.instruction
            .as_ref()
            .and_then(|i| i.amount_b_one_post.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn token_authority(&self) -> String {
        self.accounts
            .as_ref()
            .map(|a| a.token_authority.clone())
            .unwrap()
    }

    fn whirlpool(&self) -> String {
        self.accounts
            .as_ref()
            .map(|a| a.whirlpool_one.clone())
            .unwrap()
    }

    fn is_two_hop(&self) -> bool {
        true
    }

    fn second_hop(&self) -> Option<Box<dyn SwapInstruction>> {
        Some(Box::new(TwoHopSwapSecondHop(self.clone())))
    }
}

struct TwoHopSwapSecondHop(TwoHopSwap);

impl SwapInstruction for TwoHopSwapSecondHop {
    fn a_to_b(&self) -> bool {
        self.0.instruction.as_ref().map_or(false, |i| i.a_to_b_two)
    }

    fn amount_a(&self) -> String {
        self.0
            .instruction
            .as_ref()
            .and_then(|i| i.amount_a_two.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn amount_b(&self) -> String {
        self.0
            .instruction
            .as_ref()
            .and_then(|i| i.amount_b_two.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn amount_a_post(&self) -> String {
        self.0
            .instruction
            .as_ref()
            .and_then(|i| i.amount_a_two_post.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn amount_b_post(&self) -> String {
        self.0
            .instruction
            .as_ref()
            .and_then(|i| i.amount_b_two_post.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn token_authority(&self) -> String {
        self.0
            .accounts
            .as_ref()
            .map(|a| a.token_authority.clone())
            .unwrap()
    }

    fn whirlpool(&self) -> String {
        self.0
            .accounts
            .as_ref()
            .map(|a| a.whirlpool_two.clone())
            .unwrap()
    }

    fn is_two_hop(&self) -> bool {
        false
    }

    fn second_hop(&self) -> Option<Box<dyn SwapInstruction>> {
        None
    }
}

impl SwapInstruction for TwoHopSwapV2 {
    fn a_to_b(&self) -> bool {
        self.instruction.as_ref().map_or(false, |i| i.a_to_b_one)
    }

    fn amount_a(&self) -> String {
        self.instruction
            .as_ref()
            .and_then(|i| i.amount_a_one.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn amount_b(&self) -> String {
        self.instruction
            .as_ref()
            .and_then(|i| i.amount_b_one.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn amount_a_post(&self) -> String {
        self.instruction
            .as_ref()
            .and_then(|i| i.amount_a_one_post.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn amount_b_post(&self) -> String {
        self.instruction
            .as_ref()
            .and_then(|i| i.amount_b_one_post.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn token_authority(&self) -> String {
        self.accounts
            .as_ref()
            .map(|a| a.token_authority.clone())
            .unwrap()
    }

    fn whirlpool(&self) -> String {
        self.accounts
            .as_ref()
            .map(|a| a.whirlpool_one.clone())
            .unwrap()
    }

    fn is_two_hop(&self) -> bool {
        true
    }

    fn second_hop(&self) -> Option<Box<dyn SwapInstruction>> {
        Some(Box::new(TwoHopSwapV2SecondHop(self.clone())))
    }
}

struct TwoHopSwapV2SecondHop(TwoHopSwapV2);

impl SwapInstruction for TwoHopSwapV2SecondHop {
    fn a_to_b(&self) -> bool {
        self.0.instruction.as_ref().map_or(false, |i| i.a_to_b_two)
    }

    fn amount_a(&self) -> String {
        self.0
            .instruction
            .as_ref()
            .and_then(|i| i.amount_a_two.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn amount_b(&self) -> String {
        self.0
            .instruction
            .as_ref()
            .and_then(|i| i.amount_b_two.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn amount_a_post(&self) -> String {
        self.0
            .instruction
            .as_ref()
            .and_then(|i| i.amount_a_two_post.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn amount_b_post(&self) -> String {
        self.0
            .instruction
            .as_ref()
            .and_then(|i| i.amount_b_two_post.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn token_authority(&self) -> String {
        self.0
            .accounts
            .as_ref()
            .map(|a| a.token_authority.clone())
            .unwrap()
    }

    fn whirlpool(&self) -> String {
        self.0
            .accounts
            .as_ref()
            .map(|a| a.whirlpool_two.clone())
            .unwrap()
    }

    fn is_two_hop(&self) -> bool {
        false
    }

    fn second_hop(&self) -> Option<Box<dyn SwapInstruction>> {
        None
    }
}

impl SwapInstruction for OrcaSwap {
    fn a_to_b(&self) -> bool {
        self.instruction.as_ref().map_or(false, |i| i.a_to_b)
    }

    fn amount_a(&self) -> String {
        self.instruction
            .as_ref()
            .and_then(|i| i.amount_a.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn amount_b(&self) -> String {
        self.instruction
            .as_ref()
            .and_then(|i| i.amount_b.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn amount_a_post(&self) -> String {
        self.instruction
            .as_ref()
            .and_then(|i| i.amount_a_post.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn amount_b_post(&self) -> String {
        self.instruction
            .as_ref()
            .and_then(|i| i.amount_b_post.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn token_authority(&self) -> String {
        self.accounts
            .as_ref()
            .map(|a| a.token_authority.clone())
            .unwrap()
    }

    fn whirlpool(&self) -> String {
        self.accounts.as_ref().map(|a| a.whirlpool.clone()).unwrap()
    }

    fn is_two_hop(&self) -> bool {
        false
    }

    fn second_hop(&self) -> Option<Box<dyn SwapInstruction>> {
        None
    }
}

impl SwapInstruction for OrcaSwapV2 {
    fn a_to_b(&self) -> bool {
        self.instruction.as_ref().map_or(false, |i| i.a_to_b)
    }

    fn amount_a(&self) -> String {
        self.instruction
            .as_ref()
            .and_then(|i| i.amount_a.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn amount_b(&self) -> String {
        self.instruction
            .as_ref()
            .and_then(|i| i.amount_b.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn amount_a_post(&self) -> String {
        self.instruction
            .as_ref()
            .and_then(|i| i.amount_a_post.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn amount_b_post(&self) -> String {
        self.instruction
            .as_ref()
            .and_then(|i| i.amount_b_post.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }

    fn token_authority(&self) -> String {
        self.accounts
            .as_ref()
            .map(|a| a.token_authority.clone())
            .unwrap()
    }

    fn whirlpool(&self) -> String {
        self.accounts.as_ref().map(|a| a.whirlpool.clone()).unwrap()
    }

    fn is_two_hop(&self) -> bool {
        false
    }

    fn second_hop(&self) -> Option<Box<dyn SwapInstruction>> {
        None
    }
}

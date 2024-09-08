use crate::{
    constants::ZERO_STRING,
    pb::messari::orca_whirlpool::v1::{DecreaseLiquidity, DecreaseLiquidityV2},
};

pub trait WithdrawInstruction {
    fn whirlpool(&self) -> String;
    fn position_authority(&self) -> String;
    fn amount_a(&self) -> String;
    fn amount_b(&self) -> String;
    fn amount_a_post(&self) -> String;
    fn amount_b_post(&self) -> String;
    fn liquidity_amount(&self) -> String;
}

impl WithdrawInstruction for DecreaseLiquidity {
    fn whirlpool(&self) -> String {
        self.accounts
            .as_ref()
            .map(|a| a.whirlpool.clone())
            .unwrap_or_default()
    }

    fn position_authority(&self) -> String {
        self.accounts
            .as_ref()
            .map(|a| a.position_authority.clone())
            .unwrap_or_default()
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

    fn liquidity_amount(&self) -> String {
        self.instruction
            .as_ref()
            .map(|i| i.liquidity_amount.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }
}

impl WithdrawInstruction for DecreaseLiquidityV2 {
    fn whirlpool(&self) -> String {
        self.accounts
            .as_ref()
            .map(|a| a.whirlpool.clone())
            .unwrap_or_default()
    }

    fn position_authority(&self) -> String {
        self.accounts
            .as_ref()
            .map(|a| a.position_authority.clone())
            .unwrap_or_default()
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

    fn liquidity_amount(&self) -> String {
        self.instruction
            .as_ref()
            .map(|i| i.liquidity_amount.clone())
            .unwrap_or_else(|| ZERO_STRING.to_string())
    }
}

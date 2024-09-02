use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;
use substreams_solana::Address;

pub trait BalanceOf {
    fn balance_of(
        &self,
        pool_address: &Address,
        mint_address: &Address,
    ) -> (Option<String>, Option<String>);
    fn pre_balance_of(&self, pool_address: &Address, mint_address: &Address) -> Option<String>;
    fn post_balance_of(&self, pool_address: &Address, mint_address: &Address) -> Option<String>;
}

impl BalanceOf for &ConfirmedTransaction {
    fn pre_balance_of(&self, pool_address: &Address, mint_address: &Address) -> Option<String> {
        self.meta
            .as_ref()
            .unwrap()
            .pre_token_balances
            .iter()
            .find(
                |balance: &&substreams_solana::pb::sf::solana::r#type::v1::TokenBalance| {
                    balance.owner == pool_address.to_string()
                        && balance.account_index
                            == self
                                .resolved_accounts()
                                .iter()
                                .position(|address| address == mint_address)
                                .unwrap() as u32
                },
            )
            .map(|balance| balance.ui_token_amount.as_ref().unwrap().amount.to_string())
    }

    fn post_balance_of(&self, pool_address: &Address, mint_address: &Address) -> Option<String> {
        self.meta
            .as_ref()
            .unwrap()
            .post_token_balances
            .iter()
            .find(
                |balance: &&substreams_solana::pb::sf::solana::r#type::v1::TokenBalance| {
                    balance.owner == pool_address.to_string()
                        && balance.account_index
                            == self
                                .resolved_accounts()
                                .iter()
                                .position(|address| address == mint_address)
                                .unwrap() as u32
                },
            )
            .map(|balance| balance.ui_token_amount.as_ref().unwrap().amount.to_string())
    }

    fn balance_of(
        &self,
        pool_address: &Address,
        mint_address: &Address,
    ) -> (Option<String>, Option<String>) {
        let pre_balance = self.pre_balance_of(pool_address, mint_address);
        let post_balance = self.post_balance_of(pool_address, mint_address);

        (pre_balance, post_balance)
    }
}

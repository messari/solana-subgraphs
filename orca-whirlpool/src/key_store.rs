#[derive(Clone)]
pub enum StoreKey {
    Pool,
    Swap,
    User,
    Deposit,
    Withdraw,
    CumulativeUsers,
}

impl StoreKey {
    pub fn get_unique_key(&self, key: &str) -> String {
        format!("{}:{}", self.unique_id(), key)
    }

    pub fn unique_id(&self) -> String {
        match self {
            StoreKey::Pool => "POOL".to_string(),
            StoreKey::Swap => "SWAP".to_string(),
            StoreKey::User => "USER".to_string(),
            StoreKey::Deposit => "DEPOSIT".to_string(),
            StoreKey::Withdraw => "WITHDRAW".to_string(),
            StoreKey::CumulativeUsers => "CUMULATIVE_USERS".to_string(),
        }
    }
}

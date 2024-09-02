use std::fmt;

#[derive(Clone)]
pub enum StoreKey {
    Pool,
    Swap,
    User,
    Deposit,
    Withdraw,
    PoolBalance,
    PoolLiquidity,
    TotalPoolCount,
    CumulativeUsers,
    TxnCount,
    SwapCount,
    ActiveUsers,
    DepositCount,
    WithdrawCount,
    DailyVolumeByTokenAmount,
    PoolDailySnapshot(i64, Option<Box<StoreKey>>),
    UsageMetricsDailySnapshot(i64, Option<Box<StoreKey>>),
}

impl StoreKey {
    pub fn get_unique_key(&self, key: &str) -> String {
        format!("{}:{}", self.unique_id(), key)
    }

    pub fn get_unique_keys(&self, key1: &str, key2: &str) -> String {
        format!("{}:{}:{}", self.unique_id(), key1, key2)
    }

    pub fn get_snapshot_key(&self, key: Option<&str>) -> String {
        format!("{}:{}", self.unique_id(), key.unwrap_or(""))
    }

    pub fn unique_id(&self) -> String {
        match self {
            StoreKey::UsageMetricsDailySnapshot(day_id, field) => {
                let field_id = field.as_ref().map(|f| f.unique_id()).unwrap_or_default();
                format!("UsageMetricsDailySnapshot:{day_id}:{field_id}")
            }
            StoreKey::PoolDailySnapshot(day_id, field) => {
                let field_id = field.as_ref().map(|f| f.unique_id()).unwrap_or_default();
                format!("PoolDailySnapshot:{day_id}:{field_id}")
            }
            _ => format!("{}", self),
        }
    }
}

impl fmt::Display for StoreKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            StoreKey::Pool => "POOL",
            StoreKey::Swap => "SWAP",
            StoreKey::User => "USER",
            StoreKey::Deposit => "DEPOSIT",
            StoreKey::Withdraw => "WITHDRAW",
            StoreKey::PoolBalance => "POOL_BALANCE",
            StoreKey::PoolLiquidity => "POOL_LIQUIDITY",
            StoreKey::TotalPoolCount => "TOTAL_POOL_COUNT",
            StoreKey::CumulativeUsers => "CUMULATIVE_USERS",
            StoreKey::TxnCount => "TXN_COUNT",
            StoreKey::SwapCount => "SWAP_COUNT",
            StoreKey::ActiveUsers => "ACTIVE_USERS",
            StoreKey::DepositCount => "DEPOSIT_COUNT",
            StoreKey::WithdrawCount => "WITHDRAW_COUNT",
            StoreKey::DailyVolumeByTokenAmount => "DAILY_VOLUME_BY_TOKEN_AMOUNT",
            StoreKey::UsageMetricsDailySnapshot(_, _) => "USAGE_METRICS_DAILY_SNAPSHOT",
            StoreKey::PoolDailySnapshot(_, _) => "POOL_DAILY_SNAPSHOT",
        };
        write!(f, "{}", s)
    }
}

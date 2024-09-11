use substreams::scalar::BigInt;

use crate::constants::ZERO_STRING;

pub(crate) fn balance_difference(
    pre_balance: Option<String>,
    post_balance: Option<String>,
) -> Option<String> {
    let pre_balance_value = pre_balance.unwrap_or(ZERO_STRING.to_string());
    let post_balance_value = post_balance.unwrap_or(ZERO_STRING.to_string());

    let pre_balance_bigint = BigInt::try_from(&pre_balance_value).unwrap();
    let post_balance_bigint = BigInt::try_from(&post_balance_value).unwrap();

    let balance_difference = post_balance_bigint - pre_balance_bigint;

    Some(balance_difference.to_string())
}

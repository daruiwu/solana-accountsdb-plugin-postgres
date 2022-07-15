
const STEPN_ACCOUNT: &str = "STEPNq2UGeGSzCyGVr2nMQAzf8xuejwqebd84wcksCK";

pub fn is_stepn(transaction_info: &DbTransaction) -> bool {
    let legacy_message = transaction_info.legacy_message.as_ref().unwrap();
    let account_keys = legacy_message.account_keys.clone();
    for account_key in account_keys {
        let account_str: &str = str::from_utf8(&account_key).unwrap();
        let decoded = hex::decode(account_str).expect("Decoding failed");
        let account = bs58::encode(decoded).into_string();
        if account == STEPN_ACCOUNT {
            return true;
        }
    }
    let pre_token_balances = transaction_info.meta.pre_token_balances.as_ref().unwrap();
    for pre_token_balance in pre_token_balances {
        if pre_token_balance.owner.contains(STEPN_ACCOUNT) {
            return true;
        }
    }
    false
}
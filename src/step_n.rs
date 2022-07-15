use solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaTransactionInfo;
use solana_sdk::message::SanitizedMessage;
use std::str;

const STEPN_ACCOUNT: &str = "STEPNq2UGeGSzCyGVr2nMQAzf8xuejwqebd84wcksCK";

pub fn is_stepn(transaction_info: &ReplicaTransactionInfo) -> bool {
    let is_stepn = match transaction_info.transaction.message() {
        SanitizedMessage::Legacy(legacy_message) => {
            let account_keys: Vec<Vec<u8>> = legacy_message.account_keys
                .iter()
                .map(|key| key.as_ref().to_vec())
                .collect();

            for account_key in account_keys {
                let account_str: &str = str::from_utf8(&account_key).unwrap();
                let decoded = hex::decode(account_str).expect("Decoding failed");
                let account = bs58::encode(decoded).into_string();
                if account == STEPN_ACCOUNT {
                    return true;
                }
            }
            return false;
        }
        _ => false,
    };
    if is_stepn {
        return true;
    }

    return match transaction_info.transaction_status_meta.pre_token_balances.as_ref() {
        Some(pre_token_balances) => {
            for pre_token_balance in pre_token_balances {
                if pre_token_balance.owner.contains(STEPN_ACCOUNT) {
                    return true;
                }
            }
            false
        }
        _ => false
    };
}
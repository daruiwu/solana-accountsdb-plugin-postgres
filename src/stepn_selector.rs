use std::str;

use log::info;
use solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaTransactionInfo;
use solana_sdk::message::{Message, SanitizedMessage};
use solana_transaction_status::TransactionTokenBalance;

const STEPN_ACCOUNT: &str = "STEPNq2UGeGSzCyGVr2nMQAzf8xuejwqebd84wcksCK";

pub fn is_stepn_transaction(transaction_info: &ReplicaTransactionInfo) -> bool {
    return match transaction_info.transaction.message() {
        SanitizedMessage::Legacy(legacy_message) => {
            return legacy_account_has_stepn(legacy_message);
        }
        _ => false,
    } || {
        match transaction_info.transaction_status_meta.pre_token_balances.as_ref() {
            Some(pre_token_balances) => {
                pre_token_balance_owner_has_stepn(pre_token_balances)
            }
            _ => false
        }
    };
}

fn legacy_account_has_stepn(legacy_message: &Message) -> bool {
    let account_keys: Vec<Vec<u8>> = legacy_message.account_keys.iter()
        .map(|key_bytes| key_bytes.as_ref().to_vec())
        .collect();

    for account_key in account_keys {
        let account_str: &str = str::from_utf8(&account_key).unwrap();
        let decoded = hex::decode(account_str).expect("Decoding failed");
        let account = bs58::encode(decoded).into_string();
        info!("Legacy Account: {}", account);
        if str::eq(account.as_str(), STEPN_ACCOUNT) {
            return true;
        }
    }
    return false;
}

fn pre_token_balance_owner_has_stepn(pre_token_balances: &Vec<TransactionTokenBalance>) -> bool {
    for pre_token_balance in pre_token_balances {
        if str::eq(pre_token_balance.owner.as_str(), STEPN_ACCOUNT) {
            return true;
        }
    }
    false
}
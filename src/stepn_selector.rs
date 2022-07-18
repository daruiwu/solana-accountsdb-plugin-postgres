use std::str;
use lazy_static::lazy_static;

use solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaTransactionInfo;
use solana_sdk::message::{Message, SanitizedMessage};
use solana_transaction_status::TransactionTokenBalance;

const STEPN_ACCOUNT: &str = "STEPNq2UGeGSzCyGVr2nMQAzf8xuejwqebd84wcksCK";

lazy_static! {
    static ref STEPN_ACCOUNT_U8 : Vec<u8>= bs58::decode(STEPN_ACCOUNT).into_vec().unwrap();
}

pub fn is_stepn_transaction(transaction_info: &ReplicaTransactionInfo) -> bool {
    return match transaction_info.transaction.message() {
        SanitizedMessage::Legacy(legacy_message) => {
            legacy_account_has_stepn(legacy_message)
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
    return legacy_message.account_keys.iter()
        .any(|key| {
            // info!("legacy_message pubkey ({})",key);
            return key.as_ref().to_vec() == *STEPN_ACCOUNT_U8;
        });
}

fn pre_token_balance_owner_has_stepn(pre_token_balances: &Vec<TransactionTokenBalance>) -> bool {
    pre_token_balances.iter().any(|pre_token_balance| {
        str::eq(pre_token_balance.owner.as_str(), STEPN_ACCOUNT)
    })
}

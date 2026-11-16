#![allow(unused_variables)]

use std::collections::HashMap;

pub const SATS_PER_BTC: u64 = 100_000_000;
pub const GENESIS_HASH: &str = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f";
pub const GENESIS_TIMESTAMP: u64 = 1_231_006_505;
pub const GENESIS_REWARD_SATS: u64 = 50 * SATS_PER_BTC;
pub const GENESIS_MESSAGE: &str =
    "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MockTransaction {
    pub txid: String,
    pub sender: String,
    pub recipient: String,
    pub amount_sats: u64,
    pub confirmed: bool,
}

impl MockTransaction {
    /// Build a mock transaction by copying the borrowed string inputs into owned
    /// `String` fields and storing the amount and confirmation flag unchanged.
    pub fn new(
        txid: &str,
        sender: &str,
        recipient: &str,
        amount_sats: u64,
        confirmed: bool,
    ) -> Self {
        Self {
            txid: txid.to_string(),
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            amount_sats,
            confirmed,
        }
    }
}

/// Return the hardcoded Bitcoin genesis block hash.
pub fn genesis_hash() -> &'static str {
    GENESIS_HASH
}

/// Return the hardcoded Unix timestamp for the Bitcoin genesis block.
pub fn genesis_timestamp() -> u64 {
    GENESIS_TIMESTAMP
}

/// Return the genesis block reward in satoshis.
pub fn genesis_reward_sats() -> u64 {
    GENESIS_REWARD_SATS
}

/// Return the newspaper headline embedded in the genesis block coinbase data.
pub fn genesis_message() -> &'static str {
    GENESIS_MESSAGE
}

/// Build a human-readable summary string containing the genesis hash,
/// timestamp, reward, and message.
pub fn genesis_summary() -> String {
    format!(
        "Genesis block: hash={hash}, timestamp={timestamp}, reward_sats={reward}, message=\"{message}\"",
        hash = GENESIS_HASH,
        timestamp = GENESIS_TIMESTAMP,
        reward = GENESIS_REWARD_SATS,
        message = GENESIS_MESSAGE,
    )
}

/// Calculate the Bitcoin block subsidy for a height.
///
/// Start at 50 BTC, halve every 210,000 blocks, and return zero after
/// 64 or more halvings.
pub fn block_subsidy(height: u64) -> u64 {
    let halvings = height / 210_000;
    if halvings >= 64 {
        return 0;
    }

    GENESIS_REWARD_SATS / (2u64.pow(halvings as u32))
}

/// Format satoshis as a BTC string with exactly eight decimal places.
pub fn format_sats(sats: u64) -> String {
    let whole_btc = sats / SATS_PER_BTC;
    let remainder = sats % SATS_PER_BTC;
    format!("{whole_btc}.{remainder:08} BTC")
}

/// Count transactions where `confirmed` is true.
pub fn count_confirmed(transactions: &[MockTransaction]) -> usize {
    transactions.iter().filter(|tx| tx.confirmed).count()
}

/// Count transactions where `confirmed` is false.
pub fn count_unconfirmed(transactions: &[MockTransaction]) -> usize {
    transactions.iter().filter(|tx| !tx.confirmed).count()
}

/// Sum the amount of every transaction, confirmed and unconfirmed.
pub fn total_amount(transactions: &[MockTransaction]) -> u64 {
    transactions.iter().map(|tx| tx.amount_sats).sum()
}

/// Return the integer average transaction amount.
///
/// Return zero when the input slice is empty.
pub fn average_amount(transactions: &[MockTransaction]) -> u64 {
    if transactions.is_empty() {
        0
    } else {
        total_amount(transactions) / transactions.len() as u64
    }
}

/// Return cloned transactions whose sender exactly matches `sender`.
///
/// Preserve the original order.
pub fn filter_by_sender(transactions: &[MockTransaction], sender: &str) -> Vec<MockTransaction> {
    let mut matches = Vec::new();
    for tx in transactions {
        if tx.sender == sender {
            matches.push(tx.clone());
        }
    }
    matches
}

/// Return cloned transactions whose recipient exactly matches `recipient`.
///
/// Preserve the original order.
pub fn filter_by_recipient(
    transactions: &[MockTransaction],
    recipient: &str,
) -> Vec<MockTransaction> {
    let mut matches = Vec::new();
    for tx in transactions {
        if tx.recipient == recipient {
            matches.push(tx.clone());
        }
    }
    matches
}

/// Return cloned transactions that are confirmed.
///
/// Preserve the original order.
pub fn filter_confirmed(transactions: &[MockTransaction]) -> Vec<MockTransaction> {
    let mut matches = Vec::new();
    for tx in transactions {
        if tx.confirmed {
            matches.push(tx.clone());
        }
    }
    matches
}

/// Return all transaction ids as owned strings in their original order.
pub fn transaction_ids(transactions: &[MockTransaction]) -> Vec<String> {
    transactions.iter().map(|tx| tx.txid.clone()).collect()
}

/// Find the first transaction with a matching txid and return an owned clone.
///
/// Return `None` when no transaction matches.
pub fn find_transaction(transactions: &[MockTransaction], txid: &str) -> Option<MockTransaction> {
    for tx in transactions {
        if tx.txid == txid {
            return Some(tx.clone());
        }
    }
    None
}

/// Return amounts that are strictly greater than `minimum_sats`.
///
/// Preserve the original order.
pub fn amounts_over(transactions: &[MockTransaction], minimum_sats: u64) -> Vec<u64> {
    let mut matches = Vec::new();
    for tx in transactions {
        if tx.amount_sats > minimum_sats {
            matches.push(tx.amount_sats);
        }
    }
    matches
}

/// Build a balance map from confirmed transactions only.
///
/// Subtract each confirmed amount from the sender and add it to the recipient.
pub fn build_balances(transactions: &[MockTransaction]) -> HashMap<String, i64> {
    let mut balances = HashMap::new();
    for tx in transactions {
        if !tx.confirmed {
            continue;
        }

        let amount = tx.amount_sats as i64;
        *balances.entry(tx.sender.clone()).or_insert(0) -= amount;
        *balances.entry(tx.recipient.clone()).or_insert(0) += amount;
    }
    balances
}

/// Sum confirmed amounts received by `address`.
pub fn address_received_total(transactions: &[MockTransaction], address: &str) -> u64 {
    let mut total = 0;
    for tx in transactions {
        if tx.confirmed && tx.recipient == address {
            total += tx.amount_sats;
        }
    }
    total
}

/// Sum confirmed amounts sent by `address`.
pub fn address_sent_total(transactions: &[MockTransaction], address: &str) -> u64 {
    let mut total = 0;
    for tx in transactions {
        if tx.confirmed && tx.sender == address {
            total += tx.amount_sats;
        }
    }
    total
}

/// Return confirmed received total minus confirmed sent total for `address`.
pub fn net_balance_change(transactions: &[MockTransaction], address: &str) -> i64 {
    let received = address_received_total(transactions, address) as i64;
    let sent = address_sent_total(transactions, address) as i64;
    received - sent
}

/// Return true when the transaction sender is exactly `"coinbase"`.
pub fn is_coinbase(transaction: &MockTransaction) -> bool {
    transaction.sender == "coinbase"
}

/// Classify an amount as `"dust"`, `"micro"`, `"standard"`, or `"large"`.
///
/// Use the thresholds described in this week's README.
pub fn classify_amount(sats: u64) -> &'static str {
    match sats {
        s if s < 546 => "dust",
        s if s < 100_000 => "micro",
        s if s < 100_000_000 => "standard",
        _ => "large",
    }
}

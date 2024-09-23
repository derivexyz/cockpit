use crate::types::shared::RPCId;
use bigdecimal;
use serde::{Deserialize, Serialize};
use uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiquidationParams {
    ///Subaccount ID owned by wallet, that will be doing the bidding.
    pub subaccount_id: i64,
    ///Subaccount ID of the account to be liquidated.
    pub liquidated_subaccount_id: i64,
    ///Amount of cash to transfer to a newly created subaccount for bidding. Must be non-negative.
    pub cash_transfer: bigdecimal::BigDecimal,
    ///Percent of the liquidated position to bid for. Will bid for the maximum possible percent of the position if set to 1
    pub percent_bid: bigdecimal::BigDecimal,
    ///Maximum amount of cash to be paid from bidder to liquidated account (supports negative amounts for insolvent auctions). Not checked if set to 0.
    pub price_limit: bigdecimal::BigDecimal,
    ///Last seen trade ID for account being liquidated. Not checked if set to 0.
    pub last_seen_trade_id: i64,
    ///Unix timestamp in seconds. Order signature becomes invalid after this time, and the system will cancel the order. Expiry MUST be at least 5 min from now.
    pub signature_expiry_sec: i64,
    ///Unique nonce defined as <UTC_timestamp in ms><random_number_up_to_6_digits> (e.g. 1695836058725001, where 001 is the random number)
    pub nonce: i64,
    ///Owner wallet address or registered session key that signed order
    pub signer: String,
    ///Ethereum signature of the order
    pub signature: String,
}

use bigdecimal::BigDecimal;

/// Performs market making on the spot market and hedges via perps
/// The basic logic involves:

/// hedge costs -> this is a conservative estimate for how much the hedge would cost
/// picks "WORST" between perp + fees + slippage risk ("spread") and some index-based guard rails 
/// (in case perp trades very far away from spot)
/// perp            ------|--|-----
/// perp + spread   ----|------|---
/// index min       ---|-----|-----
/// pick worst:
/// hedge proxy     ---|-------|---

/// index max vs spot - dime -> pick "BEST" between the two, meant to represent true spot liquidity
/// this is used as "edge competitive bid / ask" - i.e. no reason to quote better than this 
/// index max       |-----------|--
/// spot - dime     ----|----------
/// pick best:
/// spot proxy:     ----|-------|--

/// finally, the algo quotes AT MOST as tight as spot proxy and AT LEAST as wide as hedge proxy
/// i.e. bid = min(spot bid, hedge bid), ask = max(spot ask, hedge ask)
/// hedge proxy     ---|-------|---
/// spot proxy:     ----|-------|--
/// quotes:         ---|--------|--


pub struct AlgoParams {
    /// the quoted spot bid / ask will always be at least this far away from index,
    /// even if the perp bid / ask spread is very tight
    pub min_index_spread: BigDecimal,
    /// if spot is not quoted by other MMs and only perps are available, will be assuming that
    /// spot should be traded around max_index_spread
    pub max_index_spread: BigDecimal,
    /// spread to apply to perp prices (after fees) to account for slippage, risk and profit 
    pub hedge_spread: BigDecimal,
    /// try to quote this much size on both bid and ask (subject to hedge liquidity)
    /// in calculating perp bid / ask, will "walk the book" and compute avg fill based on this size
    pub size: BigDecimal,
    /// target delta to maintain (e.g. if ETH is borrowed, can set this to debt amount)
    pub target_delta: BigDecimal,
    /// attempt to execute the hedge over twap_ms milliseconds
    pub twap_ms: u64
    // todo asymmetric params (allow more bid size or different spread to bid or ask)
}
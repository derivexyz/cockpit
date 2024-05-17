pub enum ExecutorState {
    SpotOnly, // process withdrawals here
    OptionAuction,
    AwaitSettlement,
    SpotAuction,
}

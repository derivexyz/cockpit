# CLI examples by type group

Run from `cockpit_v3` after env/session keys are set (see `README.md`).

```bash
CLI="cargo run -p derive-rs --"
SUBACCOUNT_ID=1
INSTRUMENT=ETH-PERP
OPTION=ETH-20260925-4000-C
WALLET=0x0000000000000000000000000000000000000000
VAULT_ID=2
```

Private methods log in automatically. The CLI **auto-signs** `private/order`, `private/replace`, `private/deposit`, `private/withdraw`, `private/send_quote`, `private/execute_quote`, and `private/liquidate`. Other signed methods (`private/replace_quote`, vault writes) need a full signed params JSON, or use `auctions` for liquidations.

---

## Liquidations

Types: `LiquidationParams`, `StartAuctionParams`, `GetLiquidationHistoryParams`, `AuctionsWatchResultSchema` / `AuctionDetailsSchema`, `SendLiquidateResult`, `StartAuctionResult`, `LiquidationHistoryResult`, `AuctionHistory`, `AuctionBidEvent`, `AuctionState`, `AuctionType`.

### `AuctionsWatchResultSchema` — live auction feed

Interactive table (pause with Enter, type a subaccount id, `exec` to bid using `LiquidationParams` signed by the CLI):

```bash
$CLI auctions --subaccount-id "$SUBACCOUNT_ID"
# or: DERIVE_SUBACCOUNT_ID=$SUBACCOUNT_ID $CLI auctions
```

Raw `auctions.watch` notifications (`AuctionState` `ongoing` / `ended`):

```bash
$CLI sub '["auctions.watch"]'
```

### `StartAuctionParams` → `public/start_auction` (`StartAuctionResult`)

```bash
$CLI rpc -m public/start_auction -i "{\"subaccount_id\": $SUBACCOUNT_ID}"
```

### `GetLiquidationHistoryParams` → `public/get_liquidation_history` (`LiquidationHistoryResult`)

All accounts, first page:

```bash
$CLI rpc -m public/get_liquidation_history -i '{}'
```

Filter by subaccount and window (unix ms). Result `auction_type` is `solvent` or `insolvent` (`AuctionType`).

```bash
$CLI rpc -m public/get_liquidation_history -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"start_timestamp\": 1700000000000,
  \"end_timestamp\": 1800000000000,
  \"page\": 1,
  \"page_size\": 50
}"
```

### `LiquidationParams` → `private/liquidate` (`SendLiquidateResult`)

Prefer `auctions` + `exec`. CLI auto-signs (`percent_of_acc` multiple of `0.01`; `"0"` price limit opts out):

```bash
$CLI rpc -m private/liquidate -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"liquidate_subaccount_id\": 99,
  \"percent_of_acc\": \"1.0\",
  \"price_limit\": \"0\"
}"
```

---

## Orders

Types: `OrderParams` / `OrderArgs`, `ReplaceParams`, `GetTradesParams`, `OrderResponse`, `TradeResponse`, `SendOrderResult`, `ReplaceResult`, `GetTradesResult`, plus enums `Direction`, `OrderType`, `TimeInForce`, `OrderStatus`, `CancelReason`, `LiquidityRole`, `AlgoType`, `TriggerType`, `TriggerPriceType`, `BatchStatus`.

CLI `private/order` JSON is **`OrderArgs` plus** `instrument_name` and `subaccount_id` (signature/nonce filled in by the client).

### `OrderParams` / `OrderArgs` → `private/order` (`SendOrderResult`)

Limit GTC buy:

```bash
$CLI rpc -m private/order -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"instrument_name\": \"$INSTRUMENT\",
  \"amount\": \"0.1\",
  \"limit_price\": \"2000\",
  \"direction\": \"buy\",
  \"time_in_force\": \"gtc\",
  \"order_type\": \"limit\",
  \"label\": \"cli-limit-gtc\",
  \"mmp\": false
}"
```

Limit sell `post_only`:

```bash
$CLI rpc -m private/order -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"instrument_name\": \"$INSTRUMENT\",
  \"amount\": \"0.1\",
  \"limit_price\": \"5000\",
  \"direction\": \"sell\",
  \"time_in_force\": \"post_only\",
  \"order_type\": \"limit\",
  \"label\": \"cli-post-only\",
  \"mmp\": false
}"
```

IOC / FOK / market (`limit_price` still required for the signature):

```bash
$CLI rpc -m private/order -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"instrument_name\": \"$INSTRUMENT\",
  \"amount\": \"0.1\",
  \"limit_price\": \"3000\",
  \"direction\": \"buy\",
  \"time_in_force\": \"ioc\",
  \"order_type\": \"limit\",
  \"label\": \"cli-ioc\",
  \"mmp\": false
}"
```

```bash
$CLI rpc -m private/order -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"instrument_name\": \"$INSTRUMENT\",
  \"amount\": \"0.1\",
  \"limit_price\": \"3000\",
  \"direction\": \"buy\",
  \"time_in_force\": \"fok\",
  \"order_type\": \"limit\",
  \"label\": \"cli-fok\",
  \"mmp\": false
}"
```

```bash
$CLI rpc -m private/order -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"instrument_name\": \"$INSTRUMENT\",
  \"amount\": \"0.1\",
  \"limit_price\": \"3000\",
  \"direction\": \"buy\",
  \"time_in_force\": \"ioc\",
  \"order_type\": \"market\",
  \"label\": \"cli-market\",
  \"mmp\": false
}"
```

Option instrument (`InstrumentType::option`):

```bash
$CLI rpc -m private/order -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"instrument_name\": \"$OPTION\",
  \"amount\": \"1\",
  \"limit_price\": \"50\",
  \"direction\": \"buy\",
  \"time_in_force\": \"gtc\",
  \"order_type\": \"limit\",
  \"label\": \"cli-option\",
  \"mmp\": false
}"
```

MMP-tagged:

```bash
$CLI rpc -m private/order -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"instrument_name\": \"$INSTRUMENT\",
  \"amount\": \"0.1\",
  \"limit_price\": \"2000\",
  \"direction\": \"buy\",
  \"time_in_force\": \"gtc\",
  \"order_type\": \"limit\",
  \"label\": \"cli-mmp\",
  \"mmp\": true
}"
```

### `ReplaceParams` → `private/replace` (`ReplaceResult`)

CLI auto-signs. Include `order_id_to_cancel` (or use the SDK `send_replace` helpers for `nonce_to_cancel`). JSON is `OrderArgs` plus `instrument_name`, `subaccount_id`, and `order_id_to_cancel`.

```bash
$CLI rpc -m private/replace -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"instrument_name\": \"$INSTRUMENT\",
  \"order_id_to_cancel\": \"00000000-0000-0000-0000-000000000001\",
  \"amount\": \"0.1\",
  \"limit_price\": \"2100\",
  \"direction\": \"buy\",
  \"time_in_force\": \"gtc\",
  \"order_type\": \"limit\",
  \"label\": \"cli-replace\",
  \"mmp\": false
}"
```

### `GetTradesParams` → `private/get_trade_history` (`GetTradesResult` / `TradeResponse`)

```bash
$CLI rpc -m private/get_trade_history -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"page\": 1,
  \"page_size\": 100
}"
```

By instrument / order / quote / time:

```bash
$CLI rpc -m private/get_trade_history -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"instrument_name\": \"$INSTRUMENT\",
  \"from_timestamp\": 1700000000000,
  \"to_timestamp\": 1800000000000,
  \"page\": 1,
  \"page_size\": 50
}"
```

```bash
$CLI rpc -m private/get_trade_history -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"order_id\": \"00000000-0000-0000-0000-000000000001\"
}"
```

```bash
$CLI rpc -m private/get_trade_history -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"quote_id\": \"00000000-0000-0000-0000-000000000002\"
}"
```

```bash
$CLI rpc -m private/get_trade_history -i "{
  \"wallet\": \"$WALLET\",
  \"page\": 1,
  \"page_size\": 50
}"
```

### Order / trade notifications (`OrderNotificationData`, `TradeNotificationData`)

```bash
$CLI sub "[\"$SUBACCOUNT_ID.orders\"]"
$CLI sub "[\"$SUBACCOUNT_ID.trades\"]"
```

Related cancel RPCs (not in `types/`, useful after placing test orders):

```bash
$CLI rpc -m private/cancel_all -i "{\"subaccount_id\": $SUBACCOUNT_ID}"
$CLI rpc -m private/cancel_by_instrument -i "{\"subaccount_id\": $SUBACCOUNT_ID, \"instrument_name\": \"$INSTRUMENT\"}"
$CLI rpc -m private/get_open_orders -i "{\"subaccount_id\": $SUBACCOUNT_ID}"
$CLI rpc -m private/get_order -i "{\"subaccount_id\": $SUBACCOUNT_ID, \"order_id\": \"00000000-0000-0000-0000-000000000001\"}"
```

COMMENTED OUT FOR NOW:

Spot deposit / withdraw (CLI auto-signs; `amount` is a **string**. Withdraw sends `amount_in_underlying` plus default `max_fee_usd` `"1"` and `force_batch` false):


```bash
$CLI rpc -m private/deposit -i "{\"subaccount_id\": $SUBACCOUNT_ID, \"amount\": \"10\", \"asset_name\": \"USDC\"}"
$CLI rpc -m private/withdraw -i "{\"subaccount_id\": $SUBACCOUNT_ID, \"amount\": \"1\", \"asset_name\": \"USDC\"}"
```

---

## RFQs

Types: `RfqParams`, `QuoteParams` / `QuoteArgs`, `ExecuteQuoteParams`, `ReplaceQuoteParams`, `PollRfqsParams`, `GetQuotesParams`, `LegUnpriced`, `LegPriced`, `RFQResultPrivate`, `RfqResultPublicSchema`, `QuoteResultPublic`, `QuoteResultSchema`, `PollRfqsResult`, `GetQuotesResult`, `PollQuotesResult`, `ReplaceQuoteResult`, `RFQStatus`, RFQ `CancelReason`.

### `RfqParams` / `LegUnpriced` → `private/send_rfq` (`RFQResultPrivate`)

No signature on this method.

```bash
$CLI rpc -m private/send_rfq -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"label\": \"cli-rfq\",
  \"legs\": [
    {\"instrument_name\": \"$INSTRUMENT\", \"direction\": \"buy\", \"amount\": \"0.1\"}
  ]
}"
```

Two-leg, cost bounds, counterparties, extra fee, partial fill:

```bash
$CLI rpc -m private/send_rfq -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"label\": \"cli-rfq-multi\",
  \"legs\": [
    {\"instrument_name\": \"$OPTION\", \"direction\": \"buy\", \"amount\": \"1\"},
    {\"instrument_name\": \"$INSTRUMENT\", \"direction\": \"sell\", \"amount\": \"0.1\"}
  ],
  \"max_total_cost\": \"5000\",
  \"min_total_cost\": \"-5000\",
  \"counterparties\": [\"0xMARKETMAKER\"],
  \"extra_fee\": \"0.01\",
  \"partial_fill_step\": \"0.25\",
  \"referral_code\": \"\",
  \"client\": \"cli-examples\"
}"
```

### `QuoteParams` / `QuoteArgs` / `LegPriced` → `private/send_quote` (`QuoteResultSchema`)

CLI auto-signs. JSON is `QuoteArgs` plus `subaccount_id`.

```bash
$CLI rpc -m private/send_quote -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"rfq_id\": \"00000000-0000-0000-0000-000000000010\",
  \"direction\": \"sell\",
  \"label\": \"cli-quote\",
  \"legs\": [
    {\"instrument_name\": \"$INSTRUMENT\", \"direction\": \"buy\", \"amount\": \"0.1\", \"price\": \"3000\"}
  ]
}"
```

Opposite maker direction:

```bash
$CLI rpc -m private/send_quote -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"rfq_id\": \"00000000-0000-0000-0000-000000000010\",
  \"direction\": \"buy\",
  \"label\": \"cli-quote-buy\",
  \"legs\": [
    {\"instrument_name\": \"$INSTRUMENT\", \"direction\": \"buy\", \"amount\": \"0.1\", \"price\": \"3000\"}
  ]
}"
```

### `ExecuteQuoteParams` → `private/execute_quote`

CLI polls `private/poll_quotes` then signs execute. Pass `quote_id` + `subaccount_id`:

```bash
$CLI rpc -m private/execute_quote -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"quote_id\": \"00000000-0000-0000-0000-000000000011\"
}"
```

### `ReplaceQuoteParams` → `private/replace_quote` (`ReplaceQuoteResult`)

Not auto-signed.

```bash
$CLI rpc -m private/replace_quote -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"rfq_id\": \"00000000-0000-0000-0000-000000000010\",
  \"quote_id_to_cancel\": \"00000000-0000-0000-0000-000000000011\",
  \"direction\": \"sell\",
  \"label\": \"cli-replace-quote\",
  \"legs\": [
    {\"instrument_name\": \"$INSTRUMENT\", \"direction\": \"buy\", \"amount\": \"0.1\", \"price\": \"3010\"}
  ],
  \"max_fee\": \"10\",
  \"mmp\": false
}"
```

### `PollRfqsParams` → `private/poll_rfqs` (`PollRfqsResult` / `RfqResultPublicSchema`)

Open RFQs visible to a maker:

```bash
$CLI rpc -m private/poll_rfqs -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"status\": \"open\",
  \"page\": 1,
  \"page_size\": 100
}"
```

`RFQStatus` variants: `open`, `filled`, `cancelled`, `expired`.

```bash
$CLI rpc -m private/poll_rfqs -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"status\": \"filled\",
  \"from_timestamp\": 1700000000000,
  \"to_timestamp\": 1800000000000,
  \"rfq_id\": \"00000000-0000-0000-0000-000000000010\",
  \"rfq_subaccount_id\": $SUBACCOUNT_ID,
  \"page\": 1,
  \"page_size\": 20
}"
```

Taker history (`GetRFQsResult` / `RFQResultPrivate`):

```bash
$CLI rpc -m private/get_rfqs -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"status\": \"open\",
  \"page\": 1,
  \"page_size\": 100
}"
```

### `GetQuotesParams` → `private/get_quotes` (`GetQuotesResult`) and `private/poll_quotes` (`PollQuotesResult` / `QuoteResultPublic`)

Maker quotes:

```bash
$CLI rpc -m private/get_quotes -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"status\": \"open\",
  \"page\": 1,
  \"page_size\": 100
}"
```

Taker poll (used internally by `private/execute_quote`):

```bash
$CLI rpc -m private/poll_quotes -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"rfq_id\": \"00000000-0000-0000-0000-000000000010\",
  \"quote_id\": \"00000000-0000-0000-0000-000000000011\",
  \"status\": \"open\",
  \"from_timestamp\": 1700000000000,
  \"to_timestamp\": 1800000000000,
  \"page\": 1,
  \"page_size\": 50
}"
```

Cancel helpers:

```bash
$CLI rpc -m private/cancel_rfq -i "{\"subaccount_id\": $SUBACCOUNT_ID, \"rfq_id\": \"00000000-0000-0000-0000-000000000010\"}"
$CLI rpc -m private/cancel_quote -i "{\"subaccount_id\": $SUBACCOUNT_ID, \"quote_id\": \"00000000-0000-0000-0000-000000000011\"}"
$CLI rpc -m private/cancel_batch_rfqs -i "{\"subaccount_id\": $SUBACCOUNT_ID}"
$CLI rpc -m private/cancel_batch_quotes -i "{\"subaccount_id\": $SUBACCOUNT_ID}"
```

### RFQ / quote notifications

```bash
$CLI sub "[\"$WALLET.rfqs\"]"
$CLI sub "[\"$SUBACCOUNT_ID.quotes\"]"
$CLI sub "[\"$SUBACCOUNT_ID.best.quotes\"]"
```

---

## Tickers

Types: `InstrumentTicker`, `InstrumentSlimTicker`, `InstrumentData`, `TickerResponse`, `InstrumentResponse`, `InstrumentsResponse`, `TickerNotificationData`, `TickerSlimNotificationData`, `AggregateTradingStatsSchema` / `Slim`, `OptionPricingSchema` / `Slim`, `OptionPublicDetailsSchema`, `PerpPublicDetailsSchema`, `SpotPublicDetailsSchema`, `InstrumentType`, `TickerInterval`, `OptionType`.

There are no ticker *params* structs in `types/`; these RPCs exercise the result types.

### `InstrumentTicker` / `TickerResponse` → `public/get_ticker`

Perp:

```bash
$CLI rpc -m public/get_ticker -i "{\"instrument_name\": \"$INSTRUMENT\"}"
```

Option (`OptionType` `C`/`P` in `option_details`):

```bash
$CLI rpc -m public/get_ticker -i "{\"instrument_name\": \"$OPTION\"}"
```

Spot / erc20:

```bash
$CLI rpc -m public/get_ticker -i '{"instrument_name": "ETH-USDC"}'
```

### `InstrumentData` / `InstrumentResponse` → `public/get_instrument`

```bash
$CLI rpc -m public/get_instrument -i "{\"instrument_name\": \"$INSTRUMENT\"}"
$CLI rpc -m public/get_instrument -i "{\"instrument_name\": \"$OPTION\"}"
$CLI rpc -m public/get_instrument -i '{"instrument_name": "ETH-USDC"}'
```

### `InstrumentsResponse` → `public/get_all_instruments`

`instrument_type`: `erc20` | `option` | `perp`.

```bash
$CLI rpc -m public/get_all_instruments -i '{"instrument_type": "perp", "expired": false, "page": 1, "page_size": 100}'
$CLI rpc -m public/get_all_instruments -i '{"instrument_type": "erc20", "expired": false, "currency": "ETH"}'
$CLI rpc -m public/get_all_instruments -i '{"instrument_type": "option", "expired": false, "currency": "ETH", "page": 1, "page_size": 100}'
$CLI rpc -m public/get_all_instruments -i '{"instrument_type": "option", "expired": true, "currency": "ETH"}'
```

Live names only:

```bash
$CLI rpc -m public/get_all_live_instruments -i '{}'
```

### `InstrumentSlimTicker` → `public/get_tickers`

```bash
$CLI rpc -m public/get_tickers -i '{"instrument_type": "perp"}'
$CLI rpc -m public/get_tickers -i '{"instrument_type": "perp", "currency": "ETH"}'
$CLI rpc -m public/get_tickers -i '{"instrument_type": "erc20", "currency": "ETH"}'
$CLI rpc -m public/get_tickers -i '{"instrument_type": "option", "currency": "ETH", "expiry_date": 20260925}'
```

### Ticker channels (`TickerInterval` `100` / `1000`)

Full ticker (`TickerNotificationData`) is deprecated on the server; use `ticker_slim`:

```bash
$CLI sub "[\"ticker_slim.$INSTRUMENT.100\"]"
$CLI sub "[\"ticker_slim.$INSTRUMENT.1000\"]"
```

Slim (`TickerSlimNotificationData`):

```bash
$CLI sub "[\"ticker_slim.$INSTRUMENT.100\"]"
$CLI sub "[\"ticker_slim.$INSTRUMENT.1000\"]"
$CLI sub "[\"ticker_slim.$OPTION.1000\"]"
```

Orderbook helper (not a ticker type, same CLI):

```bash
$CLI orderbook --instrument "$INSTRUMENT"
```

---

## Vaults

Types: `GetVaultParams`, `GetVaultsParams`, `GetVaultActionHistoryParams`, `GetVaultPerformanceHistoryParams`, `WalletVaultParams`, `GetVaultRequestHistoryParams`, `GetLiveQueueParams`, `RequestVaultDepositParams`, `RequestVaultWithdrawParams`, `CancelAllVaultRequestsParams`, `CreateVaultParams`, `MintSharesParams`, `BurnSharesParams`, `UpdateVaultInfoParams`, `RejectDepositRequestParams`, `ForceBurnParams`, `VaultRequestId`, plus results `Vault`, `VaultsResult`, `PublicVaultAction`, `VaultPerformancePoint`, `VaultRequest`, `VaultOpResult`, `VaultCancelResult`, `OffchainAck`, enum `PerformanceResolution`.

Write methods are **not** auto-signed by the CLI except where noted; unsigned reads work as-is.

### `GetVaultParams` → `public/get_vault` (`VaultResponse`)

```bash
$CLI rpc -m public/get_vault -i "{\"subaccount_id\": $VAULT_ID}"
```

### `GetVaultsParams` → `public/get_vaults` (`VaultsResponse`)

```bash
$CLI rpc -m public/get_vaults -i '{}'
$CLI rpc -m public/get_vaults -i '{"page": 1, "page_size": 50}'
```

### `GetVaultActionHistoryParams` → `public/get_vault_action_history`

```bash
$CLI rpc -m public/get_vault_action_history -i "{\"subaccount_id\": $VAULT_ID, \"page\": 1, \"page_size\": 50}"
$CLI rpc -m public/get_vault_action_history -i "{\"subaccount_id\": $VAULT_ID, \"event_type\": \"Deposit\"}"
```

### `GetVaultPerformanceHistoryParams` → `public/get_vault_performance_history`

`resolution`: `1h` | `8h` | `24h` | `1wk` (`PerformanceResolution`). JSON fields `from` / `to` (not `from_timestamp`).

```bash
$CLI rpc -m public/get_vault_performance_history -i "{\"subaccount_id\": $VAULT_ID, \"resolution\": \"1h\"}"
$CLI rpc -m public/get_vault_performance_history -i "{\"subaccount_id\": $VAULT_ID, \"resolution\": \"8h\"}"
$CLI rpc -m public/get_vault_performance_history -i "{\"subaccount_id\": $VAULT_ID, \"resolution\": \"24h\"}"
$CLI rpc -m public/get_vault_performance_history -i "{
  \"subaccount_id\": $VAULT_ID,
  \"resolution\": \"1wk\",
  \"from\": 1700000000000,
  \"to\": 1800000000000,
  \"limit\": 100
}"
```

### `WalletVaultParams`

```bash
$CLI rpc -m private/get_curated_vaults -i "{\"wallet\": \"$WALLET\"}"
$CLI rpc -m private/get_shareholder_vaults -i "{\"wallet\": \"$WALLET\"}"
$CLI rpc -m private/get_vault_shares -i "{\"wallet\": \"$WALLET\"}"
$CLI rpc -m private/get_live_vault_requests -i "{\"wallet\": \"$WALLET\"}"
```

### `GetVaultRequestHistoryParams` → `private/get_vault_request_history`

```bash
$CLI rpc -m private/get_vault_request_history -i "{\"wallet\": \"$WALLET\", \"page\": 1, \"page_size\": 50}"
```

### `GetLiveQueueParams`

```bash
$CLI rpc -m private/get_live_mint_requests -i "{\"subaccount_id\": $VAULT_ID, \"limit\": 50}"
$CLI rpc -m private/get_live_burn_requests -i "{\"subaccount_id\": $VAULT_ID, \"limit\": 50}"
```

### `UpdateVaultInfoParams` → `private/update_vault_info` (`OffchainAck`)

Unsigned off-chain metadata:

```bash
$CLI rpc -m private/update_vault_info -i "{
  \"subaccount_id\": $VAULT_ID,
  \"name\": \"cli-vault\",
  \"description\": \"updated from cli.examples.md\",
  \"mtm_cap\": \"1000000\",
  \"whitelist_only\": false
}"
```

### `RejectDepositRequestParams` → `private/reject_deposit_request` (`VaultRequestAck`)

```bash
$CLI rpc -m private/reject_deposit_request -i "{
  \"request_id\": {
    \"vault_nonce\": \"1\",
    \"vault_subaccount_id\": $VAULT_ID,
    \"wallet\": \"$WALLET\"
  },
  \"reason\": \"cli test reject\"
}"
```

### `ForceBurnParams` → `private/force_burn` (`VaultOpResult`)

```bash
$CLI rpc -m private/force_burn -i "{\"subaccount_id\": $VAULT_ID, \"holder\": \"$WALLET\"}"
```

### Signed vault writes (`CreateVaultParams`, deposit/withdraw/cancel/mint/burn)

CLI forwards the JSON as-is. Fill `nonce`, `signer`, `signature`, `signature_expiry_sec` from a real action signature (see `json_rpc.rs` vault helpers).

`CreateVaultParams` → `private/create_vault`:

```bash
$CLI rpc -m private/create_vault -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"manager_id\": $SUBACCOUNT_ID,
  \"deposit_spot_asset\": \"USDC\",
  \"initial_deposit\": \"1000\",
  \"initial_share_price_usd\": \"1\",
  \"management_fee_bps\": 100,
  \"performance_fee_bps\": 1000,
  \"max_slippage_bps\": 50,
  \"cooldown_sec\": 86400,
  \"max_fee_usd\": \"10\",
  \"benchmark_asset\": \"ETH\"
}"
```

`RequestVaultDepositParams` → `private/request_vault_deposit`:

```bash
$CLI rpc -m private/request_vault_deposit -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"vault_subaccount_id\": $VAULT_ID,
  \"deposit_spot_asset\": \"USDC\",
  \"amount\": \"100\"
}"
```

`RequestVaultWithdrawParams` → `private/request_vault_withdraw`:

```bash
$CLI rpc -m private/request_vault_withdraw -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"vault_subaccount_id\": $VAULT_ID,
  \"shares_to_burn\": \"10\"
}"
```

`CancelAllVaultRequestsParams` → `private/cancel_all_vault_requests` (`VaultCancelResult`):

```bash
$CLI rpc -m private/cancel_all_vault_requests -i "{
  \"subaccount_id\": $SUBACCOUNT_ID,
  \"vault_subaccount_id\": $VAULT_ID
}"
```

`MintSharesParams` → `private/mint_vault_shares`:

```bash
$CLI rpc -m private/mint_vault_shares -i "{
  \"subaccount_id\": $VAULT_ID,
  \"request_id\": {
    \"vault_nonce\": \"1\",
    \"vault_subaccount_id\": $VAULT_ID,
    \"wallet\": \"$WALLET\"
  },
  \"share_price\": \"1.05\",
  \"deposit_hash\": \"0xHASH\"
}"
```

`BurnSharesParams` → `private/burn_vault_shares`:

```bash
$CLI rpc -m private/burn_vault_shares -i "{
  \"subaccount_id\": $VAULT_ID,
  \"request_id\": {
    \"vault_nonce\": \"1\",
    \"vault_subaccount_id\": $VAULT_ID,
    \"wallet\": \"$WALLET\"
  },
  \"share_price\": \"1.05\",
  \"withdraw_hash\": \"0xHASH\"
}"
```

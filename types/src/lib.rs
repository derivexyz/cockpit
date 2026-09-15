pub mod generated;
pub mod types;

#[cfg(test)]
mod v3_wire_tests {
    use crate::generated::private_get_subaccount::PrivateGetSubaccountResponseSchema;

    #[test]
    fn private_get_subaccount_v3_body_parses() {
        let body = r#"{
            "id":"ef412cc5-665e-44ed-8e34-922afb9b3420",
            "result":{
                "subaccount_id":78649,
                "failed_to_fetch":false,
                "manager_id":5,
                "risk_universe_id":1,
                "label":"Test MM",
                "currency":["BTC","ETH"],
                "margin_type":"PM2",
                "is_under_liquidation":false,
                "positions_value":"0",
                "collaterals_value":"47899.000191362549",
                "subaccount_value":"47899.000191362549",
                "mm_credits":"0",
                "positions_maintenance_margin":"0",
                "positions_initial_margin":"0",
                "collaterals_maintenance_margin":"47899.000191362549",
                "collaterals_initial_margin":"47899.000191362549",
                "maintenance_margin":"47899.000191362549",
                "initial_margin":"47899.000191362549",
                "open_orders_margin":"0",
                "projected_margin_change":"0",
                "open_orders":[],
                "positions":[],
                "collaterals":[{
                    "asset_type":"erc20",
                    "asset_name":"USDC",
                    "amount":"47899.000191362549",
                    "average_price":"1",
                    "average_price_excl_fees":"1",
                    "mark_price":"1",
                    "mark_value":"47899.000191362549",
                    "amount_step":"0",
                    "currency":"USDC",
                    "creation_timestamp":0,
                    "cumulative_interest":"0.000307455173",
                    "pending_interest":"0",
                    "initial_margin":"47899.000191362549",
                    "maintenance_margin":"47899.000191362549",
                    "open_orders_margin":"0",
                    "realized_pnl":"0",
                    "realized_pnl_excl_fees":"0",
                    "total_fees":"0",
                    "unrealized_pnl":"0",
                    "unrealized_pnl_excl_fees":"0",
                    "delta_currency":"USDC",
                    "delta":"1"
                }],
                "vault_deposit_holds":[]
            }
        }"#;
        let parsed: PrivateGetSubaccountResponseSchema =
            serde_json::from_str(body).expect("v3 get_subaccount body should parse");
        assert_eq!(parsed.result.subaccount_id, 78649);
        assert_eq!(parsed.result.currency, vec!["BTC", "ETH"]);
        assert_eq!(parsed.result.collaterals.len(), 1);
        assert_eq!(parsed.result.collaterals[0].asset_name, "USDC");
    }
}

// auto-generates a module called types using stubs from the types directory
// use automod::dir;
// pub mod generated {
//     automod::dir!(pub "./src/generated");
// }

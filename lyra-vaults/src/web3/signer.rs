use crate::web3::actions::{sign_execute_quote, sign_order};
use crate::web3::contracts::{
    get_tsa_contract, maybe_tsa_address, ProviderWithSigner, TSA,
};
use anyhow::{Error, Result};
use ethers::prelude::{LocalWallet, Middleware};
use log::info;
use lyra_client::actions::{
    new_execute_params, new_order_params, ExecuteQuoteParams, OrderArgs, OrderParams,
    QuoteResultPublic,
};
use lyra_client::auth::load_signer_by_name;
use orderbook_types::types::tickers::InstrumentTicker;
use std::collections::HashMap;

/// Signs orderbook actions on behalf of a vault subaccount.
///
/// - [VaultSigner::Tsa] signs every action onchain via the vault's TSA contract, which is both
///   the owner and the signer of the subaccount. The session key only pays for the gas and
///   signs the API request envelope.
/// - [VaultSigner::Session] signs actions with the session key directly, i.e. the same flow any
///   regular API user follows. Selected when `{VAULT_NAME}_TSA_ADDRESS` is not set, in which case
///   `OWNER_PUBLIC_KEY` is expected to be the EOA owner of the subaccount.
#[derive(Debug)]
pub enum VaultSigner {
    Tsa(TSA<ProviderWithSigner>),
    Session(LocalWallet),
}

impl VaultSigner {
    pub async fn new(vault_name: &str) -> Result<Self> {
        match maybe_tsa_address(vault_name) {
            Some(address) => {
                info!("VaultSigner signing via the {} TSA at {}", vault_name, address);
                Ok(Self::Tsa(get_tsa_contract(vault_name, "SESSION").await?))
            }
            None => {
                info!("No TSA address for {}, VaultSigner signing via the session key", vault_name);
                Ok(Self::Session(load_signer_by_name("SESSION").await))
            }
        }
    }

    /// The wallet signing the API requests. For TSA vaults the action itself is signed onchain,
    /// and this wallet is the session key registered as the TSA's onchain signer.
    pub fn wallet(&self) -> &LocalWallet {
        match self {
            Self::Tsa(tsa) => tsa.client_ref().inner().signer(),
            Self::Session(wallet) => wallet,
        }
    }

    fn subaccount_id() -> Result<i64> {
        let subaccount_id = std::env::var("SUBACCOUNT_ID")
            .map_err(|_| Error::msg("SUBACCOUNT_ID is not set"))?;
        Ok(subaccount_id.parse()?)
    }

    /// Returns signed `private/order` params, signing onchain first if the vault has a TSA.
    pub async fn order_params(
        &self,
        ticker: &InstrumentTicker,
        args: OrderArgs,
    ) -> Result<OrderParams> {
        match self {
            Self::Tsa(tsa) => {
                let action_data = sign_order(tsa, ticker, &args).await?;
                action_data.to_order_params(self.wallet(), ticker, args)
            }
            Self::Session(wallet) => new_order_params(wallet, ticker, Self::subaccount_id()?, args),
        }
    }

    /// Returns signed `private/execute_quote` params, signing onchain first if the vault has a TSA.
    pub async fn execute_quote_params(
        &self,
        tickers: &HashMap<String, InstrumentTicker>,
        quote: &QuoteResultPublic,
    ) -> Result<ExecuteQuoteParams> {
        match self {
            Self::Tsa(tsa) => {
                let action_data = sign_execute_quote(tsa, tickers, quote).await?;
                action_data.to_execute_params(self.wallet(), tickers, quote)
            }
            Self::Session(wallet) => {
                new_execute_params(wallet, tickers, Self::subaccount_id()?, quote)
            }
        }
    }
}

#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///An RPC error object that will be returned if the RPC call fails
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "RPCError",
  "description": "An RPC error object that will be returned if the RPC call fails",
  "type": "object",
  "required": [
    "error",
    "id"
  ],
  "properties": {
    "error": {
      "$ref": "#/definitions/RPCErrorCode"
    },
    "id": {
      "type": [
        "integer",
        "string"
      ]
    }
  }
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RpcError {
    pub error: RpcErrorCode,
    pub id: RpcErrorId,
}
impl From<&RpcError> for RpcError {
    fn from(value: &RpcError) -> Self {
        value.clone()
    }
}
///An enum type for RPC error codes
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "RPCErrorCode",
  "description": "An enum type for RPC error codes",
  "type": "object",
  "oneOf": [
    {
      "title": "Ok",
      "description": "No error (typically omitted from a successful response)",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 0
        },
        "data": {},
        "message": {
          "type": "string",
          "const": ""
        }
      }
    },
    {
      "title": "RateLimitExceeded",
      "description": "Check per IP rate limits for non-auth requests or your account details for auth requests",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": -32000
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Rate limit exceeded"
        }
      }
    },
    {
      "title": "ConcurrentWsClientExceeded",
      "description": "Check per IP max concurrent clients for non-auth requests or your account details for auth requests",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": -32100
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Number of concurrent websocket clients limit exceeded"
        }
      }
    },
    {
      "title": "ParseError",
      "description": "Invalid JSON was received by the server.<br />An error occurred on the server while parsing the JSON text.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": -32700
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Parse error"
        }
      }
    },
    {
      "title": "InvalidRequest",
      "description": "The JSON sent is not a valid Request object.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": -32600
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Invalid Request"
        }
      }
    },
    {
      "title": "MethodNotFound",
      "description": "The method does not exist / is not available.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": -32601
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Method not found"
        }
      }
    },
    {
      "title": "InvalidParams",
      "description": "Invalid method parameter(s).",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": -32602
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Invalid params"
        }
      }
    },
    {
      "title": "InternalError",
      "description": "Internal JSON-RPC error.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": -32603
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Internal error"
        }
      }
    },
    {
      "title": "DatabaseError",
      "description": "Database error, see data for details.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 8000
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Database error"
        }
      }
    },
    {
      "title": "DjangoError",
      "description": "Django error, see data for details.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 8001
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Django error"
        }
      }
    },
    {
      "title": "DuplicateCashAsset",
      "description": "Can only create 1 cash asset in the db.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 8002
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Duplicate cash asset"
        }
      }
    },
    {
      "title": "OptionBalanceForSettlementNotFound",
      "description": "Quote delta was debited but option balance was not adjusted",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 8003
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "No open option balance for the settlement event was not found"
        }
      }
    },
    {
      "title": "MoreThanOneOptionBalanceForSettlementFound",
      "description": "Quote delta was debited but option balance was not adjusted",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 8004
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "More than one option balance for the settlement event was found"
        }
      }
    },
    {
      "title": "NoVacantInstruments",
      "description": "Latching failed because no vacant instruments were found.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 8100
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "No vacant instruments"
        }
      }
    },
    {
      "title": "InvalidServiceType",
      "description": "Provided service type is invalid.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 8101
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Invalid service type"
        }
      }
    },
    {
      "title": "LatchNotRetained",
      "description": "Latch was not retained by the pod.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 8102
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Latch not retained"
        }
      }
    },
    {
      "title": "FeedsNotFound",
      "description": "Error querying feeds from the database, see data for details",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 8200
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Feeds not found"
        }
      }
    },
    {
      "title": "InvalidOptionInstrumentDeactivation",
      "description": "Option instrument deactivation must be at-least MAX_HEARTBEAT_INTERVAL before option_details.expiry",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 8300
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Scheduled deactivation too late"
        }
      }
    },
    {
      "title": "InvalidHeartbeatInterval",
      "description": "Must be within MIN_HEARTBEAT_INTERVAL and MAX_HEARTBEAT_INTERVAL",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 8301
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Engine or publisher heartbeat inervals is invalid"
        }
      }
    },
    {
      "title": "InvalidMakerOrTakerFee",
      "description": "Maker fee must be less than or equal to taker fee and both must be < 0.05",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 8303
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "The maker and/or taker fees are invalid"
        }
      }
    },
    {
      "title": "InvalidInstrumentName",
      "description": "Instrument name for options must be in the format `underlying-expiry-strike(with _ for decimals)-type`",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 8304
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Instrument name is invalid"
        }
      }
    },
    {
      "title": "OptionSettlementPriceCouldNotBeSaved",
      "description": "During option expiry listener, settlement price could not be retreived",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 8402
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Option settlement price could not be saved"
        }
      }
    },
    {
      "title": "CannotSaveSettlementPriceToNonOptionAsset",
      "description": "You should not be attempting to save settlement price to Option asset details",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 8403
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Option settlement price cannot be saved to a non-option asset"
        }
      }
    },
    {
      "title": "CounterpartyInsufficientFunds",
      "description": "Counterparty had insufficient funds to fill the order and order re-submission was not possible.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 8500
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Counterparty insufficient funds"
        }
      }
    },
    {
      "title": "CounterpartyMaxFeeTooLow",
      "description": "Max fee for one or more counterparties is too low",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 8501
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Max fee for one or more counterparties is too low"
        }
      }
    },
    {
      "title": "OrderConfirmationTimeout",
      "description": "Order confirmation timed out but order status is unknown. Please check status of open orders.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 9000
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Order confirmation timeout"
        }
      }
    },
    {
      "title": "ManagerNotFound",
      "description": "The requested margin type does not exist.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 10000
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Manager not found"
        }
      }
    },
    {
      "title": "AssetNotErc20",
      "description": "The requested asset is not an ERC20 token and therefore cannot be deposited/withdrawn/transferred.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 10001
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Asset is not an ERC20 token"
        }
      }
    },
    {
      "title": "TransferWalletMismatch",
      "description": "Transfers can only be made to subaccounts under the same wallet.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 10002
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Sender and recipient wallet do not match"
        }
      }
    },
    {
      "title": "SameAccountTransfer",
      "description": "Transfers can only be made to a different subaccount id.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 10003
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Sender and recipient subaccount IDs are the same"
        }
      }
    },
    {
      "title": "MultipleCurrenciesNotSupported",
      "description": "Only standard margin accounts may hold assets of multiple currencies.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 10004
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Multiple currencies not supported"
        }
      }
    },
    {
      "title": "MaxSubaccountsPerWallet",
      "description": "Withdraw any unused subaccounts to add new ones.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 10005
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Maximum number of subaccounts per wallet reached"
        }
      }
    },
    {
      "title": "MaxSessionKeysPerWallet",
      "description": "Deactivate any unused session keys to add new ones.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 10006
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Maximum number of session keys per wallet reached"
        }
      }
    },
    {
      "title": "MaxAssetsPerSubaccount",
      "description": "Number of assets in a subaccount is limited by the on-chain constraints.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 10007
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Maximum number of assets per subaccount reached"
        }
      }
    },
    {
      "title": "MaxExpiriesPerSubaccount",
      "description": "Number of expiries in a portfolio margin subaccount is limited by the on-chain constraints.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 10008
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Maximum number of expiries per subaccount reached"
        }
      }
    },
    {
      "title": "TransferMustBeToNonZeroSubaccount",
      "description": "Transfers must be made to registerred non-zero subaccounts.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 10009
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Recipient subaccount ID of the transfer cannot be 0"
        }
      }
    },
    {
      "title": "InsufficientFunds",
      "description": "Insufficient funds to place order.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11000
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Insufficient funds"
        }
      }
    },
    {
      "title": "OrderRejectedFromQueue",
      "description": "Order did not reach the queue, matching engine might be down, or order was in the queue for too long.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11002
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Order rejected from queue"
        }
      }
    },
    {
      "title": "OrderAlreadyCancelled",
      "description": "Order is already cancelled.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11003
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Already cancelled"
        }
      }
    },
    {
      "title": "OrderAlreadyFilled",
      "description": "Order is already filled.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11004
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Already filled"
        }
      }
    },
    {
      "title": "OrderAlreadyExpired",
      "description": "Order is already expired.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11005
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Already expired"
        }
      }
    },
    {
      "title": "OrderDoesNotExist",
      "description": "Order does not exist.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11006
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Does not exist"
        }
      }
    },
    {
      "title": "SelfCrossingDisallowed",
      "description": "Order was rejected because it crossed with another order placed by the same user.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11007
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Self-crossing disallowed"
        }
      }
    },
    {
      "title": "PostOnlyReject",
      "description": "A post-only order was rejected because it would have matched with an existing order.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11008
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Post-only reject"
        }
      }
    },
    {
      "title": "ZeroLiquidityForTakerOrder",
      "description": "A market or an IOC/FOK order was rejected because there was no liquidity within the provided limit price.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11009
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Zero liquidity for market or IOC/FOK order"
        }
      }
    },
    {
      "title": "PostOnlyInvalidOrderType",
      "description": "A post-only order was rejected because it was not a limit order.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11010
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Post-only invalid order type"
        }
      }
    },
    {
      "title": "OrderInvalidSignatureExpiry",
      "description": "Order expiry is too short or is beyond expiry (for options only).",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11011
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Invalid signature expiry"
        }
      }
    },
    {
      "title": "OrderInvalidAmount",
      "description": "Order amount is invalid, see data for details.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11012
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Invalid amount"
        }
      }
    },
    {
      "title": "OrderInvalidLimitPrice",
      "description": "Order limit price is invalid, see data for details.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11013
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Invalid limit price"
        }
      }
    },
    {
      "title": "FokNotFilled",
      "description": "A fill-or-kill order was not filled.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11014
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Fill-or-kill not filled"
        }
      }
    },
    {
      "title": "MmpFrozen",
      "description": "An order was rejected because the market maker protections were triggered.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11015
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "MMP frozen"
        }
      }
    },
    {
      "title": "OrderAlreadyConsumed",
      "description": "Order is already consumed (Filled/expired/rejected).",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11016
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Already consumed"
        }
      }
    },
    {
      "title": "OrderNonUniqueNonce",
      "description": "This nonce has already been used, please use a new nonce.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11017
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Non unique nonce"
        }
      }
    },
    {
      "title": "OrderInvalidNonceDate",
      "description": "First 10 digits of nonce must be a UTC sec timestamp within 1 hour of the true UTC timestamp.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11018
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Invalid nonce date"
        }
      }
    },
    {
      "title": "TooManyOrders",
      "description": "Too many open orders for this subaccount.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11019
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Open orders limit exceeded"
        }
      }
    },
    {
      "title": "NegativeErc20Balance",
      "description": "Wrapped ERC20 balances cannot be negative.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11020
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Negative ERC20 balance"
        }
      }
    },
    {
      "title": "InstrumentNotLive",
      "description": "Instrument has either been deactivated or expired (if an Option)",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11021
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Instrument is not live"
        }
      }
    },
    {
      "title": "RejectTimestampExceeded",
      "description": "Order was rejected because it reached the engine after the supplied `reject_timestamp`.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11022
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Reject timestamp exceeded"
        }
      }
    },
    {
      "title": "MaxFeeTooLow",
      "description": "Max fee order param must always be >= 2 x max(taker, maker) x spot_price. If the order crosses the book, it must be >= 2 x max(taker, maker) x spot_price + base_fee / fill_amount.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11023
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Max fee order param is too low"
        }
      }
    },
    {
      "title": "ReduceOnlyNotSupported",
      "description": "Reduce only orders have to be market orders or non-resting limit orders (IOC or FOK).",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11024
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Reduce only not supported with this time in force"
        }
      }
    },
    {
      "title": "ReduceOnlyReject",
      "description": "A reduce-only order was rejected because it would have increased position size.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11025
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Reduce only reject"
        }
      }
    },
    {
      "title": "TransferReject",
      "description": "A transfer was rejected because it would have increased maker subaccount position size.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11026
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Transfer reject"
        }
      }
    },
    {
      "title": "SubaccountUnderLiquidation",
      "description": "A trade or order was rejected because the subaccount is undergoing a liquidation.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11027
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Subaccount undergoing liquidation"
        }
      }
    },
    {
      "title": "ReplaceOrderFilledAmountMismatch",
      "description": "New create order was reverted as the filled amount of the old order does not match the expected filled amount.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11028
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Replaced order filled amount does not match expected state."
        }
      }
    },
    {
      "title": "LegInstrumentsNotUnique",
      "description": "Leg provided in the RFQ or Quote parameter must not have repeated instrument names.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11100
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Leg instruments are not unique"
        }
      }
    },
    {
      "title": "RfqNotFound",
      "description": "RFQ query or cancellation failed because nothing was found with the given filters.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11101
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "RFQ not found"
        }
      }
    },
    {
      "title": "QuoteNotFound",
      "description": "Quote query or cancellation failed because nothing was found with the given filters.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11102
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Quote not found"
        }
      }
    },
    {
      "title": "QuoteLegMismatchVsRfq",
      "description": "Legs provided in quote parameters must match the legs in the RFQ.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11103
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Quote leg does not match RFQ leg"
        }
      }
    },
    {
      "title": "QuoteRfqNotOpen",
      "description": "Quote submission failed because the RFQ is either expired, filled or cancelled.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11104
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Requested quote or RFQ is not open"
        }
      }
    },
    {
      "title": "QuoteRfqIdMismatch",
      "description": "Quote execution failed because the RFQ ID parameter does not match the RFQ ID in the quote.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 11105
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Requested quote ID references a different RFQ ID"
        }
      }
    },
    {
      "title": "AssetNotFound",
      "description": "Requested asset does not exist.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 12000
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Asset not found"
        }
      }
    },
    {
      "title": "InstrumentNotFound",
      "description": "Requested instrument does not exist.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 12001
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Instrument not found"
        }
      }
    },
    {
      "title": "CurrencyNotFound",
      "description": "Requested currency does not exist.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 12002
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Currency not found"
        }
      }
    },
    {
      "title": "InvalidChannels",
      "description": "All channels in the subscribe request were invalid.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 13000
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Invalid channels"
        }
      }
    },
    {
      "title": "AccountNotFound",
      "description": "Requested wallet does not exist.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14000
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Account not found"
        }
      }
    },
    {
      "title": "SubaccountNotFound",
      "description": "Requested subaccount does not belong to a registered wallet.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14001
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Subaccount not found"
        }
      }
    },
    {
      "title": "SubaccountWithdrawn",
      "description": "Requested subaccount was withdrawn. Please re-deposit subaccount.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14002
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Subaccount was withdrawn"
        }
      }
    },
    {
      "title": "UseDeregisterSessionKey",
      "description": "Must use the deregisterSessionKey RPC route to reduce expiry.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14008
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Cannot reduce expiry using registerSessionKey RPC route"
        }
      }
    },
    {
      "title": "SessionKeyExpiryTooSoon",
      "description": "Increase the expiry_sec.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14009
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Session key expiry must be > utc_now + 10 min"
        }
      }
    },
    {
      "title": "SessionKeyAlreadyRegistered",
      "description": "Requested session key is already registered for this account.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14010
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Session key already registered for this account"
        }
      }
    },
    {
      "title": "SessionKeyTaken",
      "description": "Requested session key is already registered with another account.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14011
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Session key already registered with another account"
        }
      }
    },
    {
      "title": "AddressIsNotChecksum",
      "description": "Requested address must be checksumed",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14012
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Address must be checksummed"
        }
      }
    },
    {
      "title": "StringIsNotEthereumAddress",
      "description": "String must be a valid ethereum address: e.g. 0xd3cda913deb6f67967b99d67acdfa1712c293601",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14013
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "String is not a valid ethereum address"
        }
      }
    },
    {
      "title": "InvalidSignature",
      "description": "Address recovered from message and signature does not match the signer",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14014
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Signature invalid for message or transaction"
        }
      }
    },
    {
      "title": "InvalidNonce",
      "description": "Ensure the nonce is set correctly",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14015
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Transaction count for given wallet does not match provided nonce"
        }
      }
    },
    {
      "title": "SignedTxAndFunctionNameDoNotMatch",
      "description": "Ensure that the right contract abi was used when generating the transaction",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14016
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "The provided signed raw transaction contains function name that does not match the expected function name"
        }
      }
    },
    {
      "title": "SignedTxAndContractAddressDoNotMatch",
      "description": "Ensure that the right contract address was used when generating the transaction",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14017
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "The provided signed raw transaction contains contract address that does not match the expected contract address"
        }
      }
    },
    {
      "title": "SignedTxAndFunctionParamNamesDoNotMatch",
      "description": "Ensure that the right contract abi was used when generating the transaction",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14018
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "The provided signed raw transaction contains function params that do not match any expected function params"
        }
      }
    },
    {
      "title": "SignedTxAndFunctionParamValuesDoNotMatch",
      "description": "Ensure that the signed function inputs match the ones provided in the request",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14019
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "The provided signed raw transaction contains function param values that do not match the expected values"
        }
      }
    },
    {
      "title": "AuthHeaderMismatch",
      "description": "Ensure the X-LyraWallet header is set to match the requested subaccount_id or wallet",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14020
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "The X-LyraWallet header does not match the requested subaccount_id or wallet"
        }
      }
    },
    {
      "title": "AuthMissingWalletHeader",
      "description": "Ensure the X-LyraWallet header is included in the request",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14021
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "The X-LyraWallet header not provided"
        }
      }
    },
    {
      "title": "ChannelNotAuthorized",
      "description": "A private channel is not authorized for this websocket connection, or it was requested over a public method",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14022
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Subscription to a private channel failed"
        }
      }
    },
    {
      "title": "InvalidSigner",
      "description": "Address of the signer must be a wallet owner or registered session key",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14023
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Signer in on-chain related request is not wallet owner or registered session key"
        }
      }
    },
    {
      "title": "InvalidChainId",
      "description": "Refer to v2-docs for chain id for each deployment",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14024
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Chain ID must match the current roll up chain id"
        }
      }
    },
    {
      "title": "PrivateRequestMissingWalletOrSubaccount",
      "description": "Ensure request params include a subaccount_id or wallet",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14025
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "The private request is missing a wallet or subaccount_id param"
        }
      }
    },
    {
      "title": "SessionKeyNotFound",
      "description": "Requested session key does not exist.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14026
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Session key not found"
        }
      }
    },
    {
      "title": "UnauthorizedAsRfqMaker",
      "description": "The account is not authorized to act as an RFQ maker.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14027
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Unauthorized as RFQ maker"
        }
      }
    },
    {
      "title": "CrossCurrencyRfqNotSupported",
      "description": "RFQs only support same-currency legs at this moment.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 14028
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Cross currency RFQ not supported"
        }
      }
    },
    {
      "title": "RegionRestricted",
      "description": "You are in a restricted region that violates our terms of service. You may withdraw funds any time but deposits, transfers, orders are blocked",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 16000
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "You are in a restricted region that violates our terms of service."
        }
      }
    },
    {
      "title": "AccountDisabled",
      "description": "Account is disabled due to compliance violations, please contact support to enable it.",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 16001
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Account is disabled due to compliance violations, please contact support to enable it."
        }
      }
    },
    {
      "title": "InvalidSentinelAuthorization",
      "description": "The sentinel authorization is invalid",
      "type": "object",
      "required": [
        "code",
        "message"
      ],
      "properties": {
        "code": {
          "type": "integer",
          "const": 16100
        },
        "data": {},
        "message": {
          "type": "string",
          "const": "Sentinel authorization is invalid"
        }
      }
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RpcErrorCode {
    Ok {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    RateLimitExceeded {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    ConcurrentWsClientExceeded {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    ParseError {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    InvalidRequest {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    MethodNotFound {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    InvalidParams {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    InternalError {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    DatabaseError {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    DjangoError {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    DuplicateCashAsset {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    OptionBalanceForSettlementNotFound {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    MoreThanOneOptionBalanceForSettlementFound {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    NoVacantInstruments {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    InvalidServiceType {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    LatchNotRetained {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    FeedsNotFound {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    InvalidOptionInstrumentDeactivation {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    InvalidHeartbeatInterval {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    InvalidMakerOrTakerFee {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    InvalidInstrumentName {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    OptionSettlementPriceCouldNotBeSaved {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    CannotSaveSettlementPriceToNonOptionAsset {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    CounterpartyInsufficientFunds {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    CounterpartyMaxFeeTooLow {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    OrderConfirmationTimeout {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    ManagerNotFound {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    AssetNotErc20 {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    TransferWalletMismatch {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    SameAccountTransfer {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    MultipleCurrenciesNotSupported {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    MaxSubaccountsPerWallet {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    MaxSessionKeysPerWallet {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    MaxAssetsPerSubaccount {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    MaxExpiriesPerSubaccount {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    TransferMustBeToNonZeroSubaccount {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    InsufficientFunds {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    OrderRejectedFromQueue {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    OrderAlreadyCancelled {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    OrderAlreadyFilled {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    OrderAlreadyExpired {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    OrderDoesNotExist {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    SelfCrossingDisallowed {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    PostOnlyReject {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    ZeroLiquidityForTakerOrder {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    PostOnlyInvalidOrderType {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    OrderInvalidSignatureExpiry {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    OrderInvalidAmount {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    OrderInvalidLimitPrice {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    FokNotFilled {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    MmpFrozen {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    OrderAlreadyConsumed {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    OrderNonUniqueNonce {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    OrderInvalidNonceDate {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    TooManyOrders {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    NegativeErc20Balance {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    InstrumentNotLive {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    RejectTimestampExceeded {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    MaxFeeTooLow {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    ReduceOnlyNotSupported {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    ReduceOnlyReject {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    TransferReject {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    SubaccountUnderLiquidation {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    ReplaceOrderFilledAmountMismatch {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    LegInstrumentsNotUnique {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    RfqNotFound {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    QuoteNotFound {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    QuoteLegMismatchVsRfq {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    QuoteRfqNotOpen {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    QuoteRfqIdMismatch {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    AssetNotFound {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    InstrumentNotFound {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    CurrencyNotFound {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    InvalidChannels {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    AccountNotFound {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    SubaccountNotFound {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    SubaccountWithdrawn {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    UseDeregisterSessionKey {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    SessionKeyExpiryTooSoon {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    SessionKeyAlreadyRegistered {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    SessionKeyTaken {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    AddressIsNotChecksum {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    StringIsNotEthereumAddress {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    InvalidSignature {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    InvalidNonce {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    SignedTxAndFunctionNameDoNotMatch {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    SignedTxAndContractAddressDoNotMatch {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    SignedTxAndFunctionParamNamesDoNotMatch {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    SignedTxAndFunctionParamValuesDoNotMatch {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    AuthHeaderMismatch {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    AuthMissingWalletHeader {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    ChannelNotAuthorized {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    InvalidSigner {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    InvalidChainId {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    PrivateRequestMissingWalletOrSubaccount {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    SessionKeyNotFound {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    UnauthorizedAsRfqMaker {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    CrossCurrencyRfqNotSupported {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    RegionRestricted {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    AccountDisabled {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
    InvalidSentinelAuthorization {
        code: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        message: String,
    },
}
impl From<&RpcErrorCode> for RpcErrorCode {
    fn from(value: &RpcErrorCode) -> Self {
        value.clone()
    }
}
///RpcErrorId
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": [
    "integer",
    "string"
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RpcErrorId {
    String(String),
    Integer(i64),
}
impl From<&RpcErrorId> for RpcErrorId {
    fn from(value: &RpcErrorId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for RpcErrorId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for RpcErrorId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RpcErrorId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RpcErrorId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for RpcErrorId {
    fn to_string(&self) -> String {
        match self {
            Self::String(x) => x.to_string(),
            Self::Integer(x) => x.to_string(),
        }
    }
}
impl From<i64> for RpcErrorId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
///RpcErrorResponse
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "RPCErrorResponse",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/RPCError"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RpcErrorResponse(pub RpcError);
impl std::ops::Deref for RpcErrorResponse {
    type Target = RpcError;
    fn deref(&self) -> &RpcError {
        &self.0
    }
}
impl From<RpcErrorResponse> for RpcError {
    fn from(value: RpcErrorResponse) -> Self {
        value.0
    }
}
impl From<&RpcErrorResponse> for RpcErrorResponse {
    fn from(value: &RpcErrorResponse) -> Self {
        value.clone()
    }
}
impl From<RpcError> for RpcErrorResponse {
    fn from(value: RpcError) -> Self {
        Self(value)
    }
}

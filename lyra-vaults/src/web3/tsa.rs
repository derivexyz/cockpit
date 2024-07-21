pub use tsa::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod tsa {
    const _: () = {
        ::core::include_bytes!("../../abi/tsa.json");
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("acceptOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("acceptOwnership"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allowance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("spender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("approve"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("spender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approveModule"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("approveModule"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("module"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("collectFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("collectFee"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("decimals"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAccountValue"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAccountValue"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("includePending"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getActionTypedDataHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getActionTypedDataHash",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("action"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IActionVerifier.Action",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBasePrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getBasePrice"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBaseTSAAddresses"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getBaseTSAAddresses",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct BaseTSA.BaseTSAAddresses",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCollateralManagementParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCollateralManagementParams",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct CollateralManagementTSA.CollateralManagementParams",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNumShares"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNumShares"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("depositAmount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPPTSAAddresses"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPPTSAAddresses"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract ISpotFeed"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IDepositModule"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IWithdrawalModule",),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IRfqModule"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IOptionAsset"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPPTSAParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPPTSAParams"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct PrincipalProtectedTSA.PPTSAParams",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSharesValue"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSharesValue"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("numShares"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSubAccountStats"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSubAccountStats"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("openSpreads"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("baseBalance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("cashBalance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTSAParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTSAParams"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct BaseTSA.TSAParams"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("initialOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("initParams"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::String,
                                    ::ethers::core::abi::ethabi::ParamType::String,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct BaseTSA.BaseTSAInitParams",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("ppInitParams"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct PrincipalProtectedTSA.PPTSAInitParams",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initiateDeposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initiateDeposit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("recipient"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("depositId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isActionSigned"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isActionSigned"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("action"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IActionVerifier.Action",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isBlocked"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isBlocked"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isSigner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isSigner"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("signer"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isValidSignature"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isValidSignature"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_hash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_signature"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("magicValue"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes4"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lastFeeCollected"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lastFeeCollected"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lastSeenHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lastSeenHash"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("name"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pendingOwner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pendingOwner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("processDeposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("processDeposit"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("depositId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("processDeposits"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("processDeposits"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("depositIds"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("processWithdrawalRequests"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("processWithdrawalRequests",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("limit"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("queuedDeposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("queuedDeposit"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("depositId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct BaseTSA.DepositRequest",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("queuedWithdrawal"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("queuedWithdrawal"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("withdrawalId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct BaseTSA.WithdrawalRequest",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("requestWithdrawal"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requestWithdrawal"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("withdrawalId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revokeActionSignature"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("revokeActionSignature",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("action"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IActionVerifier.Action",),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revokeSignature"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("revokeSignature"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("typedDataHash"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPPTSAParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setPPTSAParams"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newCollateralMgmtParams",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct CollateralManagementTSA.CollateralManagementParams",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pptsaParams"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct PrincipalProtectedTSA.PPTSAParams",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setShareKeeper"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setShareKeeper"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("keeper"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("isKeeper"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setSignaturesDisabled"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setSignaturesDisabled",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("disabled"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setSigner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setSigner"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signer"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_isSigner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setTSAParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setTSAParams"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_params"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct BaseTSA.TSAParams"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("shareKeeper"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("shareKeeper"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("keeper"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signActionData"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signActionData"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("action"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IActionVerifier.Action",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("extraData"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signaturesDisabled"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signaturesDisabled"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedData"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedData"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("hash"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("subAccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("subAccount"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("symbol"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalPendingDeposits"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalPendingDeposits",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalPendingWithdrawals"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalPendingWithdrawals",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalSupply"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transfer"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOwner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ActionSigned"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ActionSigned"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("signer"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("hash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("action"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ],),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Approval"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("spender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DepositInitiated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("DepositInitiated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("depositId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("recipient"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DepositProcessed"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("DepositProcessed"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("depositId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("recipient"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("success"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("shares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeeCollected"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("FeeCollected"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("recipient"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("totalSupply"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Initialized"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ModuleApproved"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ModuleApproved"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("module"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferStarted"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferStarted",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferred",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPTSAParamsSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PPTSAParamsSet"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "collateralManagementParams",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ShareKeeperUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ShareKeeperUpdated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("keeper"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("isKeeper"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SignatureRevoked"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SignatureRevoked"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("signer"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("hash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SignaturesDisabledUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SignaturesDisabledUpdated",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("enabled"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SignerUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SignerUpdated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("signer"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("isSigner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TSAParamsUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("TSAParamsUpdated"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("params"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Transfer"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawalProcessed"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("WithdrawalProcessed",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("withdrawalId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("beneficiary"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("complete"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sharesProcessed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amountReceived"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawalRequested"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("WithdrawalRequested",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("withdrawalId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("beneficiary"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BOCST_InvalidAction"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BOCST_InvalidAction",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BOCST_OnlySigner"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BOCST_OnlySigner"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BTSA_Blocked"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BTSA_Blocked"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BTSA_DepositAlreadyProcessed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BTSA_DepositAlreadyProcessed",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BTSA_DepositBelowMinimum"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BTSA_DepositBelowMinimum",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BTSA_DepositCapExceeded"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BTSA_DepositCapExceeded",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BTSA_InsufficientBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BTSA_InsufficientBalance",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BTSA_InvalidParams"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BTSA_InvalidParams"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BTSA_InvalidWithdrawalAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BTSA_InvalidWithdrawalAmount",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BTSA_ModuleNotPartOfMatching"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BTSA_ModuleNotPartOfMatching",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BTSA_MustReceiveShares"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BTSA_MustReceiveShares",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BTSA_OnlyShareKeeper"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BTSA_OnlyShareKeeper",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CMTSA_BuyingTooMuchCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CMTSA_BuyingTooMuchCollateral",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CMTSA_DepositingTooMuch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CMTSA_DepositingTooMuch",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CMTSA_FeeTooHigh"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CMTSA_FeeTooHigh"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CMTSA_InvalidAsset"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CMTSA_InvalidAsset"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CMTSA_MustHaveNegativeCash"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CMTSA_MustHaveNegativeCash",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CMTSA_MustHavePositiveCash"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CMTSA_MustHavePositiveCash",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CMTSA_PositionInsolvent"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CMTSA_PositionInsolvent",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CMTSA_SellingTooMuchCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CMTSA_SellingTooMuchCollateral",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CMTSA_SpotLimitPriceTooHigh"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CMTSA_SpotLimitPriceTooHigh",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CMTSA_SpotLimitPriceTooLow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CMTSA_SpotLimitPriceTooLow",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InsufficientAllowance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC20InsufficientAllowance",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("spender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("allowance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("needed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InsufficientBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC20InsufficientBalance",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("needed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidApprover"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC20InvalidApprover",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("approver"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidReceiver"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC20InvalidReceiver",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("receiver"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidSender"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC20InvalidSender"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("sender"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidSpender"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC20InvalidSpender",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("spender"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidInitialization"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidInitialization",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotInitializing"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotInitializing"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableInvalidOwner"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OwnableInvalidOwner",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableUnauthorizedAccount"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OwnableUnauthorizedAccount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPT_InvalidActionExpiry"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PPT_InvalidActionExpiry",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPT_InvalidAsset"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PPT_InvalidAsset"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPT_InvalidBaseBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PPT_InvalidBaseBalance",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPT_InvalidDesiredAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PPT_InvalidDesiredAmount",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPT_InvalidHighStrikeAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PPT_InvalidHighStrikeAmount",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPT_InvalidModule"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PPT_InvalidModule"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPT_InvalidParams"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PPT_InvalidParams"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPT_InvalidTradeAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PPT_InvalidTradeAmount",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPT_MarkValueNotWithinBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PPT_MarkValueNotWithinBounds",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPT_OptionExpiryOutOfBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PPT_OptionExpiryOutOfBounds",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPT_StrikePriceOutsideOfDiff"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PPT_StrikePriceOutsideOfDiff",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPT_TotalCostBelowTolerance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PPT_TotalCostBelowTolerance",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPT_TotalCostOverTolerance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PPT_TotalCostOverTolerance",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPT_TradeDataDoesNotMatchOrderHash"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "PPT_TradeDataDoesNotMatchOrderHash",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPT_TradeTooLarge"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PPT_TradeTooLarge"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPT_WithdrawingUtilisedCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PPT_WithdrawingUtilisedCollateral",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPT_WithdrawingWithOpenTrades"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PPT_WithdrawingWithOpenTrades",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PPT_WrongInputSpread"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PPT_WrongInputSpread",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReentrancyGuardReentrantCall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ReentrancyGuardReentrantCall",),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static TSA_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct TSA<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TSA<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TSA<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TSA<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TSA<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TSA)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TSA<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(address.into(), TSA_ABI.clone(), client))
        }
        ///Calls the contract's `acceptOwnership` (0x79ba5097) function
        pub fn accept_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approveModule` (0x5c69d2d3) function
        pub fn approve_module(
            &self,
            module: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 105, 210, 211], (module, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `collectFee` (0xd4d5d32a) function
        pub fn collect_fee(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 213, 211, 42], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAccountValue` (0xbd4dd391) function
        pub fn get_account_value(
            &self,
            include_pending: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([189, 77, 211, 145], include_pending)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getActionTypedDataHash` (0x1ee037f4) function
        pub fn get_action_typed_data_hash(
            &self,
            action: Action,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([30, 224, 55, 244], (action,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBasePrice` (0xb49f4afd) function
        pub fn get_base_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([180, 159, 74, 253], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBaseTSAAddresses` (0x3d43038c) function
        pub fn get_base_tsa_addresses(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, BaseTSAAddresses> {
            self.0
                .method_hash([61, 67, 3, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCollateralManagementParams` (0x805230f5) function
        pub fn get_collateral_management_params(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, CollateralManagementParams> {
            self.0
                .method_hash([128, 82, 48, 245], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNumShares` (0x855cb21b) function
        pub fn get_num_shares(
            &self,
            deposit_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([133, 92, 178, 27], deposit_amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPPTSAAddresses` (0x88466ce4) function
        pub fn get_pptsa_addresses(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([136, 70, 108, 228], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPPTSAParams` (0x5ae7dd4e) function
        pub fn get_pptsa_params(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, Pptsaparams> {
            self.0
                .method_hash([90, 231, 221, 78], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSharesValue` (0xdea0e746) function
        pub fn get_shares_value(
            &self,
            num_shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([222, 160, 231, 70], num_shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubAccountStats` (0xc3d5ce56) function
        pub fn get_sub_account_stats(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256, ::ethers::core::types::I256),
        > {
            self.0
                .method_hash([195, 213, 206, 86], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTSAParams` (0xadd9d6af) function
        pub fn get_tsa_params(&self) -> ::ethers::contract::builders::ContractCall<M, Tsaparams> {
            self.0
                .method_hash([173, 217, 214, 175], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x86d58cd5) function
        pub fn initialize(
            &self,
            initial_owner: ::ethers::core::types::Address,
            init_params: BaseTSAInitParams,
            pp_init_params: PptsainitParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([134, 213, 140, 213], (initial_owner, init_params, pp_init_params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initiateDeposit` (0x4b9b708b) function
        pub fn initiate_deposit(
            &self,
            amount: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([75, 155, 112, 139], (amount, recipient))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isActionSigned` (0x5e17c823) function
        pub fn is_action_signed(
            &self,
            action: Action,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([94, 23, 200, 35], (action,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isBlocked` (0xd75f0da7) function
        pub fn is_blocked(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([215, 95, 13, 167], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSigner` (0x7df73e27) function
        pub fn is_signer(
            &self,
            signer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([125, 247, 62, 39], signer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isValidSignature` (0x1626ba7e) function
        pub fn is_valid_signature(
            &self,
            hash: [u8; 32],
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([22, 38, 186, 126], (hash, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastFeeCollected` (0x750290da) function
        pub fn last_fee_collected(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([117, 2, 144, 218], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastSeenHash` (0x8043cc18) function
        pub fn last_seen_hash(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([128, 67, 204, 24], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingOwner` (0xe30c3978) function
        pub fn pending_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([227, 12, 57, 120], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processDeposit` (0x0cc2055a) function
        pub fn process_deposit(
            &self,
            deposit_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([12, 194, 5, 90], deposit_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processDeposits` (0x2dd2f137) function
        pub fn process_deposits(
            &self,
            deposit_ids: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 210, 241, 55], deposit_ids)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processWithdrawalRequests` (0xe1bece85) function
        pub fn process_withdrawal_requests(
            &self,
            limit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 190, 206, 133], limit)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queuedDeposit` (0x8210d616) function
        pub fn queued_deposit(
            &self,
            deposit_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, DepositRequest> {
            self.0
                .method_hash([130, 16, 214, 22], deposit_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queuedWithdrawal` (0xdc82bcbf) function
        pub fn queued_withdrawal(
            &self,
            withdrawal_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, WithdrawalRequest> {
            self.0
                .method_hash([220, 130, 188, 191], withdrawal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestWithdrawal` (0x9ee679e8) function
        pub fn request_withdrawal(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([158, 230, 121, 232], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeActionSignature` (0x6adeac95) function
        pub fn revoke_action_signature(
            &self,
            action: Action,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([106, 222, 172, 149], (action,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeSignature` (0xc9fd8ee8) function
        pub fn revoke_signature(
            &self,
            typed_data_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 253, 142, 232], typed_data_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPPTSAParams` (0xe3fc7570) function
        pub fn set_pptsa_params(
            &self,
            new_collateral_mgmt_params: CollateralManagementParams,
            pptsa_params: Pptsaparams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 252, 117, 112], (new_collateral_mgmt_params, pptsa_params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setShareKeeper` (0x66ee2a1e) function
        pub fn set_share_keeper(
            &self,
            keeper: ::ethers::core::types::Address,
            is_keeper: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([102, 238, 42, 30], (keeper, is_keeper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSignaturesDisabled` (0x802e1d6d) function
        pub fn set_signatures_disabled(
            &self,
            disabled: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([128, 46, 29, 109], disabled)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSigner` (0x31cb6105) function
        pub fn set_signer(
            &self,
            signer: ::ethers::core::types::Address,
            is_signer: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 203, 97, 5], (signer, is_signer))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTSAParams` (0xd80895ab) function
        pub fn set_tsa_params(
            &self,
            params: Tsaparams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([216, 8, 149, 171], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `shareKeeper` (0xf34edc47) function
        pub fn share_keeper(
            &self,
            keeper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([243, 78, 220, 71], keeper)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signActionData` (0xa39cc91b) function
        pub fn sign_action_data(
            &self,
            action: Action,
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 156, 201, 27], (action, extra_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signaturesDisabled` (0x4e3ab0dc) function
        pub fn signatures_disabled(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([78, 58, 176, 220], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedData` (0x34d5966a) function
        pub fn signed_data(
            &self,
            hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([52, 213, 150, 106], hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `subAccount` (0x377ea3f5) function
        pub fn sub_account(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([55, 126, 163, 245], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalPendingDeposits` (0x0a881082) function
        pub fn total_pending_deposits(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([10, 136, 16, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalPendingWithdrawals` (0xa4563e03) function
        pub fn total_pending_withdrawals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([164, 86, 62, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ActionSigned` event
        pub fn action_signed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ActionSignedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApprovalFilter> {
            self.0.event()
        }
        ///Gets the contract's `DepositInitiated` event
        pub fn deposit_initiated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositInitiatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `DepositProcessed` event
        pub fn deposit_processed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositProcessedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `FeeCollected` event
        pub fn fee_collected_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FeeCollectedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ModuleApproved` event
        pub fn module_approved_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ModuleApprovedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferStarted` event
        pub fn ownership_transfer_started_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferStartedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `PPTSAParamsSet` event
        pub fn pptsa_params_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PptsaparamsSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ShareKeeperUpdated` event
        pub fn share_keeper_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ShareKeeperUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SignatureRevoked` event
        pub fn signature_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SignatureRevokedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SignaturesDisabledUpdated` event
        pub fn signatures_disabled_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SignaturesDisabledUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SignerUpdated` event
        pub fn signer_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SignerUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `TSAParamsUpdated` event
        pub fn tsa_params_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TsaparamsUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransferFilter> {
            self.0.event()
        }
        ///Gets the contract's `WithdrawalProcessed` event
        pub fn withdrawal_processed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WithdrawalProcessedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `WithdrawalRequested` event
        pub fn withdrawal_requested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WithdrawalRequestedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TSAEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for TSA<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BOCST_InvalidAction` with signature `BOCST_InvalidAction()` and selector `0x77cabbfb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BOCST_InvalidAction", abi = "BOCST_InvalidAction()")]
    pub struct BOCST_InvalidAction;
    ///Custom Error type `BOCST_OnlySigner` with signature `BOCST_OnlySigner()` and selector `0x9504b993`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BOCST_OnlySigner", abi = "BOCST_OnlySigner()")]
    pub struct BOCST_OnlySigner;
    ///Custom Error type `BTSA_Blocked` with signature `BTSA_Blocked()` and selector `0x0645006b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BTSA_Blocked", abi = "BTSA_Blocked()")]
    pub struct BTSA_Blocked;
    ///Custom Error type `BTSA_DepositAlreadyProcessed` with signature `BTSA_DepositAlreadyProcessed()` and selector `0x51791086`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BTSA_DepositAlreadyProcessed", abi = "BTSA_DepositAlreadyProcessed()")]
    pub struct BTSA_DepositAlreadyProcessed;
    ///Custom Error type `BTSA_DepositBelowMinimum` with signature `BTSA_DepositBelowMinimum()` and selector `0x27ef3c96`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BTSA_DepositBelowMinimum", abi = "BTSA_DepositBelowMinimum()")]
    pub struct BTSA_DepositBelowMinimum;
    ///Custom Error type `BTSA_DepositCapExceeded` with signature `BTSA_DepositCapExceeded()` and selector `0x5521d243`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BTSA_DepositCapExceeded", abi = "BTSA_DepositCapExceeded()")]
    pub struct BTSA_DepositCapExceeded;
    ///Custom Error type `BTSA_InsufficientBalance` with signature `BTSA_InsufficientBalance()` and selector `0x48643c07`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BTSA_InsufficientBalance", abi = "BTSA_InsufficientBalance()")]
    pub struct BTSA_InsufficientBalance;
    ///Custom Error type `BTSA_InvalidParams` with signature `BTSA_InvalidParams()` and selector `0x713e6629`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BTSA_InvalidParams", abi = "BTSA_InvalidParams()")]
    pub struct BTSA_InvalidParams;
    ///Custom Error type `BTSA_InvalidWithdrawalAmount` with signature `BTSA_InvalidWithdrawalAmount()` and selector `0x1bf552f5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BTSA_InvalidWithdrawalAmount", abi = "BTSA_InvalidWithdrawalAmount()")]
    pub struct BTSA_InvalidWithdrawalAmount;
    ///Custom Error type `BTSA_ModuleNotPartOfMatching` with signature `BTSA_ModuleNotPartOfMatching()` and selector `0x3ccf5ae4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BTSA_ModuleNotPartOfMatching", abi = "BTSA_ModuleNotPartOfMatching()")]
    pub struct BTSA_ModuleNotPartOfMatching;
    ///Custom Error type `BTSA_MustReceiveShares` with signature `BTSA_MustReceiveShares()` and selector `0x63375958`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BTSA_MustReceiveShares", abi = "BTSA_MustReceiveShares()")]
    pub struct BTSA_MustReceiveShares;
    ///Custom Error type `BTSA_OnlyShareKeeper` with signature `BTSA_OnlyShareKeeper()` and selector `0x33f49bc9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BTSA_OnlyShareKeeper", abi = "BTSA_OnlyShareKeeper()")]
    pub struct BTSA_OnlyShareKeeper;
    ///Custom Error type `CMTSA_BuyingTooMuchCollateral` with signature `CMTSA_BuyingTooMuchCollateral()` and selector `0xea76a3eb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "CMTSA_BuyingTooMuchCollateral", abi = "CMTSA_BuyingTooMuchCollateral()")]
    pub struct CMTSA_BuyingTooMuchCollateral;
    ///Custom Error type `CMTSA_DepositingTooMuch` with signature `CMTSA_DepositingTooMuch()` and selector `0xccbb69e5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "CMTSA_DepositingTooMuch", abi = "CMTSA_DepositingTooMuch()")]
    pub struct CMTSA_DepositingTooMuch;
    ///Custom Error type `CMTSA_FeeTooHigh` with signature `CMTSA_FeeTooHigh()` and selector `0x4a223f29`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "CMTSA_FeeTooHigh", abi = "CMTSA_FeeTooHigh()")]
    pub struct CMTSA_FeeTooHigh;
    ///Custom Error type `CMTSA_InvalidAsset` with signature `CMTSA_InvalidAsset()` and selector `0xda729355`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "CMTSA_InvalidAsset", abi = "CMTSA_InvalidAsset()")]
    pub struct CMTSA_InvalidAsset;
    ///Custom Error type `CMTSA_MustHaveNegativeCash` with signature `CMTSA_MustHaveNegativeCash()` and selector `0x31dc9e31`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "CMTSA_MustHaveNegativeCash", abi = "CMTSA_MustHaveNegativeCash()")]
    pub struct CMTSA_MustHaveNegativeCash;
    ///Custom Error type `CMTSA_MustHavePositiveCash` with signature `CMTSA_MustHavePositiveCash()` and selector `0x6fd318c0`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "CMTSA_MustHavePositiveCash", abi = "CMTSA_MustHavePositiveCash()")]
    pub struct CMTSA_MustHavePositiveCash;
    ///Custom Error type `CMTSA_PositionInsolvent` with signature `CMTSA_PositionInsolvent()` and selector `0x60380cfb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "CMTSA_PositionInsolvent", abi = "CMTSA_PositionInsolvent()")]
    pub struct CMTSA_PositionInsolvent;
    ///Custom Error type `CMTSA_SellingTooMuchCollateral` with signature `CMTSA_SellingTooMuchCollateral()` and selector `0xe47f7257`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "CMTSA_SellingTooMuchCollateral", abi = "CMTSA_SellingTooMuchCollateral()")]
    pub struct CMTSA_SellingTooMuchCollateral;
    ///Custom Error type `CMTSA_SpotLimitPriceTooHigh` with signature `CMTSA_SpotLimitPriceTooHigh()` and selector `0x59f38125`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "CMTSA_SpotLimitPriceTooHigh", abi = "CMTSA_SpotLimitPriceTooHigh()")]
    pub struct CMTSA_SpotLimitPriceTooHigh;
    ///Custom Error type `CMTSA_SpotLimitPriceTooLow` with signature `CMTSA_SpotLimitPriceTooLow()` and selector `0x713182f5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "CMTSA_SpotLimitPriceTooLow", abi = "CMTSA_SpotLimitPriceTooLow()")]
    pub struct CMTSA_SpotLimitPriceTooLow;
    ///Custom Error type `ERC20InsufficientAllowance` with signature `ERC20InsufficientAllowance(address,uint256,uint256)` and selector `0xfb8f41b2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "ERC20InsufficientAllowance",
        abi = "ERC20InsufficientAllowance(address,uint256,uint256)"
    )]
    pub struct ERC20InsufficientAllowance {
        pub spender: ::ethers::core::types::Address,
        pub allowance: ::ethers::core::types::U256,
        pub needed: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC20InsufficientBalance` with signature `ERC20InsufficientBalance(address,uint256,uint256)` and selector `0xe450d38c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "ERC20InsufficientBalance",
        abi = "ERC20InsufficientBalance(address,uint256,uint256)"
    )]
    pub struct ERC20InsufficientBalance {
        pub sender: ::ethers::core::types::Address,
        pub balance: ::ethers::core::types::U256,
        pub needed: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC20InvalidApprover` with signature `ERC20InvalidApprover(address)` and selector `0xe602df05`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ERC20InvalidApprover", abi = "ERC20InvalidApprover(address)")]
    pub struct ERC20InvalidApprover {
        pub approver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidReceiver` with signature `ERC20InvalidReceiver(address)` and selector `0xec442f05`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ERC20InvalidReceiver", abi = "ERC20InvalidReceiver(address)")]
    pub struct ERC20InvalidReceiver {
        pub receiver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidSender` with signature `ERC20InvalidSender(address)` and selector `0x96c6fd1e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ERC20InvalidSender", abi = "ERC20InvalidSender(address)")]
    pub struct ERC20InvalidSender {
        pub sender: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidSpender` with signature `ERC20InvalidSpender(address)` and selector `0x94280d62`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ERC20InvalidSpender", abi = "ERC20InvalidSpender(address)")]
    pub struct ERC20InvalidSpender {
        pub spender: ::ethers::core::types::Address,
    }
    ///Custom Error type `InvalidInitialization` with signature `InvalidInitialization()` and selector `0xf92ee8a9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidInitialization", abi = "InvalidInitialization()")]
    pub struct InvalidInitialization;
    ///Custom Error type `NotInitializing` with signature `NotInitializing()` and selector `0xd7e6bcf8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotInitializing", abi = "NotInitializing()")]
    pub struct NotInitializing;
    ///Custom Error type `OwnableInvalidOwner` with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "OwnableInvalidOwner", abi = "OwnableInvalidOwner(address)")]
    pub struct OwnableInvalidOwner {
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `OwnableUnauthorizedAccount` with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "OwnableUnauthorizedAccount", abi = "OwnableUnauthorizedAccount(address)")]
    pub struct OwnableUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `PPT_InvalidActionExpiry` with signature `PPT_InvalidActionExpiry()` and selector `0xf9aec3bc`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PPT_InvalidActionExpiry", abi = "PPT_InvalidActionExpiry()")]
    pub struct PPT_InvalidActionExpiry;
    ///Custom Error type `PPT_InvalidAsset` with signature `PPT_InvalidAsset()` and selector `0x0041d7aa`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PPT_InvalidAsset", abi = "PPT_InvalidAsset()")]
    pub struct PPT_InvalidAsset;
    ///Custom Error type `PPT_InvalidBaseBalance` with signature `PPT_InvalidBaseBalance()` and selector `0xc2f3647f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PPT_InvalidBaseBalance", abi = "PPT_InvalidBaseBalance()")]
    pub struct PPT_InvalidBaseBalance;
    ///Custom Error type `PPT_InvalidDesiredAmount` with signature `PPT_InvalidDesiredAmount()` and selector `0x96bb7948`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PPT_InvalidDesiredAmount", abi = "PPT_InvalidDesiredAmount()")]
    pub struct PPT_InvalidDesiredAmount;
    ///Custom Error type `PPT_InvalidHighStrikeAmount` with signature `PPT_InvalidHighStrikeAmount()` and selector `0x50979787`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PPT_InvalidHighStrikeAmount", abi = "PPT_InvalidHighStrikeAmount()")]
    pub struct PPT_InvalidHighStrikeAmount;
    ///Custom Error type `PPT_InvalidModule` with signature `PPT_InvalidModule()` and selector `0x135502c6`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PPT_InvalidModule", abi = "PPT_InvalidModule()")]
    pub struct PPT_InvalidModule;
    ///Custom Error type `PPT_InvalidParams` with signature `PPT_InvalidParams()` and selector `0xae4b4c94`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PPT_InvalidParams", abi = "PPT_InvalidParams()")]
    pub struct PPT_InvalidParams;
    ///Custom Error type `PPT_InvalidTradeAmount` with signature `PPT_InvalidTradeAmount()` and selector `0xe52f7fe0`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PPT_InvalidTradeAmount", abi = "PPT_InvalidTradeAmount()")]
    pub struct PPT_InvalidTradeAmount;
    ///Custom Error type `PPT_MarkValueNotWithinBounds` with signature `PPT_MarkValueNotWithinBounds()` and selector `0x25453e77`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PPT_MarkValueNotWithinBounds", abi = "PPT_MarkValueNotWithinBounds()")]
    pub struct PPT_MarkValueNotWithinBounds;
    ///Custom Error type `PPT_OptionExpiryOutOfBounds` with signature `PPT_OptionExpiryOutOfBounds()` and selector `0x188b1d46`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PPT_OptionExpiryOutOfBounds", abi = "PPT_OptionExpiryOutOfBounds()")]
    pub struct PPT_OptionExpiryOutOfBounds;
    ///Custom Error type `PPT_StrikePriceOutsideOfDiff` with signature `PPT_StrikePriceOutsideOfDiff()` and selector `0xcc475da9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PPT_StrikePriceOutsideOfDiff", abi = "PPT_StrikePriceOutsideOfDiff()")]
    pub struct PPT_StrikePriceOutsideOfDiff;
    ///Custom Error type `PPT_TotalCostBelowTolerance` with signature `PPT_TotalCostBelowTolerance()` and selector `0x9da78498`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PPT_TotalCostBelowTolerance", abi = "PPT_TotalCostBelowTolerance()")]
    pub struct PPT_TotalCostBelowTolerance;
    ///Custom Error type `PPT_TotalCostOverTolerance` with signature `PPT_TotalCostOverTolerance()` and selector `0xbbff2731`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PPT_TotalCostOverTolerance", abi = "PPT_TotalCostOverTolerance()")]
    pub struct PPT_TotalCostOverTolerance;
    ///Custom Error type `PPT_TradeDataDoesNotMatchOrderHash` with signature `PPT_TradeDataDoesNotMatchOrderHash()` and selector `0x16480281`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "PPT_TradeDataDoesNotMatchOrderHash",
        abi = "PPT_TradeDataDoesNotMatchOrderHash()"
    )]
    pub struct PPT_TradeDataDoesNotMatchOrderHash;
    ///Custom Error type `PPT_TradeTooLarge` with signature `PPT_TradeTooLarge()` and selector `0x7aa96c7e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PPT_TradeTooLarge", abi = "PPT_TradeTooLarge()")]
    pub struct PPT_TradeTooLarge;
    ///Custom Error type `PPT_WithdrawingUtilisedCollateral` with signature `PPT_WithdrawingUtilisedCollateral()` and selector `0x12601845`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "PPT_WithdrawingUtilisedCollateral",
        abi = "PPT_WithdrawingUtilisedCollateral()"
    )]
    pub struct PPT_WithdrawingUtilisedCollateral;
    ///Custom Error type `PPT_WithdrawingWithOpenTrades` with signature `PPT_WithdrawingWithOpenTrades()` and selector `0x51a9e7f2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PPT_WithdrawingWithOpenTrades", abi = "PPT_WithdrawingWithOpenTrades()")]
    pub struct PPT_WithdrawingWithOpenTrades;
    ///Custom Error type `PPT_WrongInputSpread` with signature `PPT_WrongInputSpread()` and selector `0x5293e86a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PPT_WrongInputSpread", abi = "PPT_WrongInputSpread()")]
    pub struct PPT_WrongInputSpread;
    ///Custom Error type `ReentrancyGuardReentrantCall` with signature `ReentrancyGuardReentrantCall()` and selector `0x3ee5aeb5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ReentrancyGuardReentrantCall", abi = "ReentrancyGuardReentrantCall()")]
    pub struct ReentrancyGuardReentrantCall;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TSAErrors {
        BOCST_InvalidAction(BOCST_InvalidAction),
        BOCST_OnlySigner(BOCST_OnlySigner),
        BTSA_Blocked(BTSA_Blocked),
        BTSA_DepositAlreadyProcessed(BTSA_DepositAlreadyProcessed),
        BTSA_DepositBelowMinimum(BTSA_DepositBelowMinimum),
        BTSA_DepositCapExceeded(BTSA_DepositCapExceeded),
        BTSA_InsufficientBalance(BTSA_InsufficientBalance),
        BTSA_InvalidParams(BTSA_InvalidParams),
        BTSA_InvalidWithdrawalAmount(BTSA_InvalidWithdrawalAmount),
        BTSA_ModuleNotPartOfMatching(BTSA_ModuleNotPartOfMatching),
        BTSA_MustReceiveShares(BTSA_MustReceiveShares),
        BTSA_OnlyShareKeeper(BTSA_OnlyShareKeeper),
        CMTSA_BuyingTooMuchCollateral(CMTSA_BuyingTooMuchCollateral),
        CMTSA_DepositingTooMuch(CMTSA_DepositingTooMuch),
        CMTSA_FeeTooHigh(CMTSA_FeeTooHigh),
        CMTSA_InvalidAsset(CMTSA_InvalidAsset),
        CMTSA_MustHaveNegativeCash(CMTSA_MustHaveNegativeCash),
        CMTSA_MustHavePositiveCash(CMTSA_MustHavePositiveCash),
        CMTSA_PositionInsolvent(CMTSA_PositionInsolvent),
        CMTSA_SellingTooMuchCollateral(CMTSA_SellingTooMuchCollateral),
        CMTSA_SpotLimitPriceTooHigh(CMTSA_SpotLimitPriceTooHigh),
        CMTSA_SpotLimitPriceTooLow(CMTSA_SpotLimitPriceTooLow),
        ERC20InsufficientAllowance(ERC20InsufficientAllowance),
        ERC20InsufficientBalance(ERC20InsufficientBalance),
        ERC20InvalidApprover(ERC20InvalidApprover),
        ERC20InvalidReceiver(ERC20InvalidReceiver),
        ERC20InvalidSender(ERC20InvalidSender),
        ERC20InvalidSpender(ERC20InvalidSpender),
        InvalidInitialization(InvalidInitialization),
        NotInitializing(NotInitializing),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        PPT_InvalidActionExpiry(PPT_InvalidActionExpiry),
        PPT_InvalidAsset(PPT_InvalidAsset),
        PPT_InvalidBaseBalance(PPT_InvalidBaseBalance),
        PPT_InvalidDesiredAmount(PPT_InvalidDesiredAmount),
        PPT_InvalidHighStrikeAmount(PPT_InvalidHighStrikeAmount),
        PPT_InvalidModule(PPT_InvalidModule),
        PPT_InvalidParams(PPT_InvalidParams),
        PPT_InvalidTradeAmount(PPT_InvalidTradeAmount),
        PPT_MarkValueNotWithinBounds(PPT_MarkValueNotWithinBounds),
        PPT_OptionExpiryOutOfBounds(PPT_OptionExpiryOutOfBounds),
        PPT_StrikePriceOutsideOfDiff(PPT_StrikePriceOutsideOfDiff),
        PPT_TotalCostBelowTolerance(PPT_TotalCostBelowTolerance),
        PPT_TotalCostOverTolerance(PPT_TotalCostOverTolerance),
        PPT_TradeDataDoesNotMatchOrderHash(PPT_TradeDataDoesNotMatchOrderHash),
        PPT_TradeTooLarge(PPT_TradeTooLarge),
        PPT_WithdrawingUtilisedCollateral(PPT_WithdrawingUtilisedCollateral),
        PPT_WithdrawingWithOpenTrades(PPT_WithdrawingWithOpenTrades),
        PPT_WrongInputSpread(PPT_WrongInputSpread),
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for TSAErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <BOCST_InvalidAction as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BOCST_InvalidAction(decoded));
            }
            if let Ok(decoded) = <BOCST_OnlySigner as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BOCST_OnlySigner(decoded));
            }
            if let Ok(decoded) = <BTSA_Blocked as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BTSA_Blocked(decoded));
            }
            if let Ok(decoded) =
                <BTSA_DepositAlreadyProcessed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BTSA_DepositAlreadyProcessed(decoded));
            }
            if let Ok(decoded) =
                <BTSA_DepositBelowMinimum as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BTSA_DepositBelowMinimum(decoded));
            }
            if let Ok(decoded) =
                <BTSA_DepositCapExceeded as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BTSA_DepositCapExceeded(decoded));
            }
            if let Ok(decoded) =
                <BTSA_InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BTSA_InsufficientBalance(decoded));
            }
            if let Ok(decoded) =
                <BTSA_InvalidParams as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BTSA_InvalidParams(decoded));
            }
            if let Ok(decoded) =
                <BTSA_InvalidWithdrawalAmount as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BTSA_InvalidWithdrawalAmount(decoded));
            }
            if let Ok(decoded) =
                <BTSA_ModuleNotPartOfMatching as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BTSA_ModuleNotPartOfMatching(decoded));
            }
            if let Ok(decoded) =
                <BTSA_MustReceiveShares as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BTSA_MustReceiveShares(decoded));
            }
            if let Ok(decoded) =
                <BTSA_OnlyShareKeeper as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BTSA_OnlyShareKeeper(decoded));
            }
            if let Ok(decoded) =
                <CMTSA_BuyingTooMuchCollateral as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CMTSA_BuyingTooMuchCollateral(decoded));
            }
            if let Ok(decoded) =
                <CMTSA_DepositingTooMuch as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CMTSA_DepositingTooMuch(decoded));
            }
            if let Ok(decoded) = <CMTSA_FeeTooHigh as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CMTSA_FeeTooHigh(decoded));
            }
            if let Ok(decoded) =
                <CMTSA_InvalidAsset as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CMTSA_InvalidAsset(decoded));
            }
            if let Ok(decoded) =
                <CMTSA_MustHaveNegativeCash as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CMTSA_MustHaveNegativeCash(decoded));
            }
            if let Ok(decoded) =
                <CMTSA_MustHavePositiveCash as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CMTSA_MustHavePositiveCash(decoded));
            }
            if let Ok(decoded) =
                <CMTSA_PositionInsolvent as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CMTSA_PositionInsolvent(decoded));
            }
            if let Ok(decoded) =
                <CMTSA_SellingTooMuchCollateral as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CMTSA_SellingTooMuchCollateral(decoded));
            }
            if let Ok(decoded) =
                <CMTSA_SpotLimitPriceTooHigh as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CMTSA_SpotLimitPriceTooHigh(decoded));
            }
            if let Ok(decoded) =
                <CMTSA_SpotLimitPriceTooLow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CMTSA_SpotLimitPriceTooLow(decoded));
            }
            if let Ok(decoded) =
                <ERC20InsufficientAllowance as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC20InsufficientAllowance(decoded));
            }
            if let Ok(decoded) =
                <ERC20InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC20InsufficientBalance(decoded));
            }
            if let Ok(decoded) =
                <ERC20InvalidApprover as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC20InvalidApprover(decoded));
            }
            if let Ok(decoded) =
                <ERC20InvalidReceiver as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC20InvalidReceiver(decoded));
            }
            if let Ok(decoded) =
                <ERC20InvalidSender as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC20InvalidSender(decoded));
            }
            if let Ok(decoded) =
                <ERC20InvalidSpender as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC20InvalidSpender(decoded));
            }
            if let Ok(decoded) =
                <InvalidInitialization as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidInitialization(decoded));
            }
            if let Ok(decoded) = <NotInitializing as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotInitializing(decoded));
            }
            if let Ok(decoded) =
                <OwnableInvalidOwner as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OwnableInvalidOwner(decoded));
            }
            if let Ok(decoded) =
                <OwnableUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OwnableUnauthorizedAccount(decoded));
            }
            if let Ok(decoded) =
                <PPT_InvalidActionExpiry as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PPT_InvalidActionExpiry(decoded));
            }
            if let Ok(decoded) = <PPT_InvalidAsset as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PPT_InvalidAsset(decoded));
            }
            if let Ok(decoded) =
                <PPT_InvalidBaseBalance as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PPT_InvalidBaseBalance(decoded));
            }
            if let Ok(decoded) =
                <PPT_InvalidDesiredAmount as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PPT_InvalidDesiredAmount(decoded));
            }
            if let Ok(decoded) =
                <PPT_InvalidHighStrikeAmount as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PPT_InvalidHighStrikeAmount(decoded));
            }
            if let Ok(decoded) = <PPT_InvalidModule as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PPT_InvalidModule(decoded));
            }
            if let Ok(decoded) = <PPT_InvalidParams as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PPT_InvalidParams(decoded));
            }
            if let Ok(decoded) =
                <PPT_InvalidTradeAmount as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PPT_InvalidTradeAmount(decoded));
            }
            if let Ok(decoded) =
                <PPT_MarkValueNotWithinBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PPT_MarkValueNotWithinBounds(decoded));
            }
            if let Ok(decoded) =
                <PPT_OptionExpiryOutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PPT_OptionExpiryOutOfBounds(decoded));
            }
            if let Ok(decoded) =
                <PPT_StrikePriceOutsideOfDiff as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PPT_StrikePriceOutsideOfDiff(decoded));
            }
            if let Ok(decoded) =
                <PPT_TotalCostBelowTolerance as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PPT_TotalCostBelowTolerance(decoded));
            }
            if let Ok(decoded) =
                <PPT_TotalCostOverTolerance as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PPT_TotalCostOverTolerance(decoded));
            }
            if let Ok(decoded) =
                <PPT_TradeDataDoesNotMatchOrderHash as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PPT_TradeDataDoesNotMatchOrderHash(decoded));
            }
            if let Ok(decoded) = <PPT_TradeTooLarge as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PPT_TradeTooLarge(decoded));
            }
            if let Ok(decoded) =
                <PPT_WithdrawingUtilisedCollateral as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PPT_WithdrawingUtilisedCollateral(decoded));
            }
            if let Ok(decoded) =
                <PPT_WithdrawingWithOpenTrades as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PPT_WithdrawingWithOpenTrades(decoded));
            }
            if let Ok(decoded) =
                <PPT_WrongInputSpread as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PPT_WrongInputSpread(decoded));
            }
            if let Ok(decoded) =
                <ReentrancyGuardReentrantCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReentrancyGuardReentrantCall(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TSAErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BOCST_InvalidAction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BOCST_OnlySigner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BTSA_Blocked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BTSA_DepositAlreadyProcessed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BTSA_DepositBelowMinimum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BTSA_DepositCapExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BTSA_InsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BTSA_InvalidParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BTSA_InvalidWithdrawalAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BTSA_ModuleNotPartOfMatching(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BTSA_MustReceiveShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BTSA_OnlyShareKeeper(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CMTSA_BuyingTooMuchCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CMTSA_DepositingTooMuch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CMTSA_FeeTooHigh(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CMTSA_InvalidAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CMTSA_MustHaveNegativeCash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CMTSA_MustHavePositiveCash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CMTSA_PositionInsolvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CMTSA_SellingTooMuchCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CMTSA_SpotLimitPriceTooHigh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CMTSA_SpotLimitPriceTooLow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InsufficientAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidApprover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidSpender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInitialization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotInitializing(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PPT_InvalidActionExpiry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PPT_InvalidAsset(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PPT_InvalidBaseBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PPT_InvalidDesiredAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PPT_InvalidHighStrikeAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PPT_InvalidModule(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PPT_InvalidParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PPT_InvalidTradeAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PPT_MarkValueNotWithinBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PPT_OptionExpiryOutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PPT_StrikePriceOutsideOfDiff(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PPT_TotalCostBelowTolerance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PPT_TotalCostOverTolerance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PPT_TradeDataDoesNotMatchOrderHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PPT_TradeTooLarge(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PPT_WithdrawingUtilisedCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PPT_WithdrawingWithOpenTrades(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PPT_WrongInputSpread(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for TSAErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BOCST_InvalidAction as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BOCST_OnlySigner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BTSA_Blocked as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <BTSA_DepositAlreadyProcessed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BTSA_DepositBelowMinimum as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BTSA_DepositCapExceeded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BTSA_InsufficientBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BTSA_InvalidParams as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BTSA_InvalidWithdrawalAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BTSA_ModuleNotPartOfMatching as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BTSA_MustReceiveShares as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BTSA_OnlyShareKeeper as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CMTSA_BuyingTooMuchCollateral as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CMTSA_DepositingTooMuch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CMTSA_FeeTooHigh as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CMTSA_InvalidAsset as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CMTSA_MustHaveNegativeCash as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CMTSA_MustHavePositiveCash as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CMTSA_PositionInsolvent as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CMTSA_SellingTooMuchCollateral as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CMTSA_SpotLimitPriceTooHigh as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CMTSA_SpotLimitPriceTooLow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InsufficientAllowance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InsufficientBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidApprover as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidReceiver as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidSpender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidInitialization as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotInitializing as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableInvalidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PPT_InvalidActionExpiry as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PPT_InvalidAsset as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PPT_InvalidBaseBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PPT_InvalidDesiredAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PPT_InvalidHighStrikeAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PPT_InvalidModule as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PPT_InvalidParams as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PPT_InvalidTradeAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PPT_MarkValueNotWithinBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PPT_OptionExpiryOutOfBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PPT_StrikePriceOutsideOfDiff as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PPT_TotalCostBelowTolerance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PPT_TotalCostOverTolerance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PPT_TradeDataDoesNotMatchOrderHash as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PPT_TradeTooLarge as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PPT_WithdrawingUtilisedCollateral as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PPT_WithdrawingWithOpenTrades as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PPT_WrongInputSpread as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ReentrancyGuardReentrantCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for TSAErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BOCST_InvalidAction(element) => ::core::fmt::Display::fmt(element, f),
                Self::BOCST_OnlySigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::BTSA_Blocked(element) => ::core::fmt::Display::fmt(element, f),
                Self::BTSA_DepositAlreadyProcessed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BTSA_DepositBelowMinimum(element) => ::core::fmt::Display::fmt(element, f),
                Self::BTSA_DepositCapExceeded(element) => ::core::fmt::Display::fmt(element, f),
                Self::BTSA_InsufficientBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::BTSA_InvalidParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::BTSA_InvalidWithdrawalAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BTSA_ModuleNotPartOfMatching(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BTSA_MustReceiveShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::BTSA_OnlyShareKeeper(element) => ::core::fmt::Display::fmt(element, f),
                Self::CMTSA_BuyingTooMuchCollateral(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CMTSA_DepositingTooMuch(element) => ::core::fmt::Display::fmt(element, f),
                Self::CMTSA_FeeTooHigh(element) => ::core::fmt::Display::fmt(element, f),
                Self::CMTSA_InvalidAsset(element) => ::core::fmt::Display::fmt(element, f),
                Self::CMTSA_MustHaveNegativeCash(element) => ::core::fmt::Display::fmt(element, f),
                Self::CMTSA_MustHavePositiveCash(element) => ::core::fmt::Display::fmt(element, f),
                Self::CMTSA_PositionInsolvent(element) => ::core::fmt::Display::fmt(element, f),
                Self::CMTSA_SellingTooMuchCollateral(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CMTSA_SpotLimitPriceTooHigh(element) => ::core::fmt::Display::fmt(element, f),
                Self::CMTSA_SpotLimitPriceTooLow(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InsufficientAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InsufficientBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InvalidApprover(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InvalidReceiver(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InvalidSpender(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidInitialization(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotInitializing(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableInvalidOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableUnauthorizedAccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::PPT_InvalidActionExpiry(element) => ::core::fmt::Display::fmt(element, f),
                Self::PPT_InvalidAsset(element) => ::core::fmt::Display::fmt(element, f),
                Self::PPT_InvalidBaseBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::PPT_InvalidDesiredAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::PPT_InvalidHighStrikeAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::PPT_InvalidModule(element) => ::core::fmt::Display::fmt(element, f),
                Self::PPT_InvalidParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::PPT_InvalidTradeAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::PPT_MarkValueNotWithinBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PPT_OptionExpiryOutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::PPT_StrikePriceOutsideOfDiff(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PPT_TotalCostBelowTolerance(element) => ::core::fmt::Display::fmt(element, f),
                Self::PPT_TotalCostOverTolerance(element) => ::core::fmt::Display::fmt(element, f),
                Self::PPT_TradeDataDoesNotMatchOrderHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PPT_TradeTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::PPT_WithdrawingUtilisedCollateral(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PPT_WithdrawingWithOpenTrades(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PPT_WrongInputSpread(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for TSAErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BOCST_InvalidAction> for TSAErrors {
        fn from(value: BOCST_InvalidAction) -> Self {
            Self::BOCST_InvalidAction(value)
        }
    }
    impl ::core::convert::From<BOCST_OnlySigner> for TSAErrors {
        fn from(value: BOCST_OnlySigner) -> Self {
            Self::BOCST_OnlySigner(value)
        }
    }
    impl ::core::convert::From<BTSA_Blocked> for TSAErrors {
        fn from(value: BTSA_Blocked) -> Self {
            Self::BTSA_Blocked(value)
        }
    }
    impl ::core::convert::From<BTSA_DepositAlreadyProcessed> for TSAErrors {
        fn from(value: BTSA_DepositAlreadyProcessed) -> Self {
            Self::BTSA_DepositAlreadyProcessed(value)
        }
    }
    impl ::core::convert::From<BTSA_DepositBelowMinimum> for TSAErrors {
        fn from(value: BTSA_DepositBelowMinimum) -> Self {
            Self::BTSA_DepositBelowMinimum(value)
        }
    }
    impl ::core::convert::From<BTSA_DepositCapExceeded> for TSAErrors {
        fn from(value: BTSA_DepositCapExceeded) -> Self {
            Self::BTSA_DepositCapExceeded(value)
        }
    }
    impl ::core::convert::From<BTSA_InsufficientBalance> for TSAErrors {
        fn from(value: BTSA_InsufficientBalance) -> Self {
            Self::BTSA_InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<BTSA_InvalidParams> for TSAErrors {
        fn from(value: BTSA_InvalidParams) -> Self {
            Self::BTSA_InvalidParams(value)
        }
    }
    impl ::core::convert::From<BTSA_InvalidWithdrawalAmount> for TSAErrors {
        fn from(value: BTSA_InvalidWithdrawalAmount) -> Self {
            Self::BTSA_InvalidWithdrawalAmount(value)
        }
    }
    impl ::core::convert::From<BTSA_ModuleNotPartOfMatching> for TSAErrors {
        fn from(value: BTSA_ModuleNotPartOfMatching) -> Self {
            Self::BTSA_ModuleNotPartOfMatching(value)
        }
    }
    impl ::core::convert::From<BTSA_MustReceiveShares> for TSAErrors {
        fn from(value: BTSA_MustReceiveShares) -> Self {
            Self::BTSA_MustReceiveShares(value)
        }
    }
    impl ::core::convert::From<BTSA_OnlyShareKeeper> for TSAErrors {
        fn from(value: BTSA_OnlyShareKeeper) -> Self {
            Self::BTSA_OnlyShareKeeper(value)
        }
    }
    impl ::core::convert::From<CMTSA_BuyingTooMuchCollateral> for TSAErrors {
        fn from(value: CMTSA_BuyingTooMuchCollateral) -> Self {
            Self::CMTSA_BuyingTooMuchCollateral(value)
        }
    }
    impl ::core::convert::From<CMTSA_DepositingTooMuch> for TSAErrors {
        fn from(value: CMTSA_DepositingTooMuch) -> Self {
            Self::CMTSA_DepositingTooMuch(value)
        }
    }
    impl ::core::convert::From<CMTSA_FeeTooHigh> for TSAErrors {
        fn from(value: CMTSA_FeeTooHigh) -> Self {
            Self::CMTSA_FeeTooHigh(value)
        }
    }
    impl ::core::convert::From<CMTSA_InvalidAsset> for TSAErrors {
        fn from(value: CMTSA_InvalidAsset) -> Self {
            Self::CMTSA_InvalidAsset(value)
        }
    }
    impl ::core::convert::From<CMTSA_MustHaveNegativeCash> for TSAErrors {
        fn from(value: CMTSA_MustHaveNegativeCash) -> Self {
            Self::CMTSA_MustHaveNegativeCash(value)
        }
    }
    impl ::core::convert::From<CMTSA_MustHavePositiveCash> for TSAErrors {
        fn from(value: CMTSA_MustHavePositiveCash) -> Self {
            Self::CMTSA_MustHavePositiveCash(value)
        }
    }
    impl ::core::convert::From<CMTSA_PositionInsolvent> for TSAErrors {
        fn from(value: CMTSA_PositionInsolvent) -> Self {
            Self::CMTSA_PositionInsolvent(value)
        }
    }
    impl ::core::convert::From<CMTSA_SellingTooMuchCollateral> for TSAErrors {
        fn from(value: CMTSA_SellingTooMuchCollateral) -> Self {
            Self::CMTSA_SellingTooMuchCollateral(value)
        }
    }
    impl ::core::convert::From<CMTSA_SpotLimitPriceTooHigh> for TSAErrors {
        fn from(value: CMTSA_SpotLimitPriceTooHigh) -> Self {
            Self::CMTSA_SpotLimitPriceTooHigh(value)
        }
    }
    impl ::core::convert::From<CMTSA_SpotLimitPriceTooLow> for TSAErrors {
        fn from(value: CMTSA_SpotLimitPriceTooLow) -> Self {
            Self::CMTSA_SpotLimitPriceTooLow(value)
        }
    }
    impl ::core::convert::From<ERC20InsufficientAllowance> for TSAErrors {
        fn from(value: ERC20InsufficientAllowance) -> Self {
            Self::ERC20InsufficientAllowance(value)
        }
    }
    impl ::core::convert::From<ERC20InsufficientBalance> for TSAErrors {
        fn from(value: ERC20InsufficientBalance) -> Self {
            Self::ERC20InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidApprover> for TSAErrors {
        fn from(value: ERC20InvalidApprover) -> Self {
            Self::ERC20InvalidApprover(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidReceiver> for TSAErrors {
        fn from(value: ERC20InvalidReceiver) -> Self {
            Self::ERC20InvalidReceiver(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidSender> for TSAErrors {
        fn from(value: ERC20InvalidSender) -> Self {
            Self::ERC20InvalidSender(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidSpender> for TSAErrors {
        fn from(value: ERC20InvalidSpender) -> Self {
            Self::ERC20InvalidSpender(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for TSAErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for TSAErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for TSAErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for TSAErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<PPT_InvalidActionExpiry> for TSAErrors {
        fn from(value: PPT_InvalidActionExpiry) -> Self {
            Self::PPT_InvalidActionExpiry(value)
        }
    }
    impl ::core::convert::From<PPT_InvalidAsset> for TSAErrors {
        fn from(value: PPT_InvalidAsset) -> Self {
            Self::PPT_InvalidAsset(value)
        }
    }
    impl ::core::convert::From<PPT_InvalidBaseBalance> for TSAErrors {
        fn from(value: PPT_InvalidBaseBalance) -> Self {
            Self::PPT_InvalidBaseBalance(value)
        }
    }
    impl ::core::convert::From<PPT_InvalidDesiredAmount> for TSAErrors {
        fn from(value: PPT_InvalidDesiredAmount) -> Self {
            Self::PPT_InvalidDesiredAmount(value)
        }
    }
    impl ::core::convert::From<PPT_InvalidHighStrikeAmount> for TSAErrors {
        fn from(value: PPT_InvalidHighStrikeAmount) -> Self {
            Self::PPT_InvalidHighStrikeAmount(value)
        }
    }
    impl ::core::convert::From<PPT_InvalidModule> for TSAErrors {
        fn from(value: PPT_InvalidModule) -> Self {
            Self::PPT_InvalidModule(value)
        }
    }
    impl ::core::convert::From<PPT_InvalidParams> for TSAErrors {
        fn from(value: PPT_InvalidParams) -> Self {
            Self::PPT_InvalidParams(value)
        }
    }
    impl ::core::convert::From<PPT_InvalidTradeAmount> for TSAErrors {
        fn from(value: PPT_InvalidTradeAmount) -> Self {
            Self::PPT_InvalidTradeAmount(value)
        }
    }
    impl ::core::convert::From<PPT_MarkValueNotWithinBounds> for TSAErrors {
        fn from(value: PPT_MarkValueNotWithinBounds) -> Self {
            Self::PPT_MarkValueNotWithinBounds(value)
        }
    }
    impl ::core::convert::From<PPT_OptionExpiryOutOfBounds> for TSAErrors {
        fn from(value: PPT_OptionExpiryOutOfBounds) -> Self {
            Self::PPT_OptionExpiryOutOfBounds(value)
        }
    }
    impl ::core::convert::From<PPT_StrikePriceOutsideOfDiff> for TSAErrors {
        fn from(value: PPT_StrikePriceOutsideOfDiff) -> Self {
            Self::PPT_StrikePriceOutsideOfDiff(value)
        }
    }
    impl ::core::convert::From<PPT_TotalCostBelowTolerance> for TSAErrors {
        fn from(value: PPT_TotalCostBelowTolerance) -> Self {
            Self::PPT_TotalCostBelowTolerance(value)
        }
    }
    impl ::core::convert::From<PPT_TotalCostOverTolerance> for TSAErrors {
        fn from(value: PPT_TotalCostOverTolerance) -> Self {
            Self::PPT_TotalCostOverTolerance(value)
        }
    }
    impl ::core::convert::From<PPT_TradeDataDoesNotMatchOrderHash> for TSAErrors {
        fn from(value: PPT_TradeDataDoesNotMatchOrderHash) -> Self {
            Self::PPT_TradeDataDoesNotMatchOrderHash(value)
        }
    }
    impl ::core::convert::From<PPT_TradeTooLarge> for TSAErrors {
        fn from(value: PPT_TradeTooLarge) -> Self {
            Self::PPT_TradeTooLarge(value)
        }
    }
    impl ::core::convert::From<PPT_WithdrawingUtilisedCollateral> for TSAErrors {
        fn from(value: PPT_WithdrawingUtilisedCollateral) -> Self {
            Self::PPT_WithdrawingUtilisedCollateral(value)
        }
    }
    impl ::core::convert::From<PPT_WithdrawingWithOpenTrades> for TSAErrors {
        fn from(value: PPT_WithdrawingWithOpenTrades) -> Self {
            Self::PPT_WithdrawingWithOpenTrades(value)
        }
    }
    impl ::core::convert::From<PPT_WrongInputSpread> for TSAErrors {
        fn from(value: PPT_WrongInputSpread) -> Self {
            Self::PPT_WrongInputSpread(value)
        }
    }
    impl ::core::convert::From<ReentrancyGuardReentrantCall> for TSAErrors {
        fn from(value: ReentrancyGuardReentrantCall) -> Self {
            Self::ReentrancyGuardReentrantCall(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "ActionSigned",
        abi = "ActionSigned(address,bytes32,(uint256,uint256,address,bytes,uint256,address,address))"
    )]
    pub struct ActionSignedFilter {
        #[ethevent(indexed)]
        pub signer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub hash: [u8; 32],
        pub action: Action,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "DepositInitiated", abi = "DepositInitiated(uint256,address,uint256)")]
    pub struct DepositInitiatedFilter {
        #[ethevent(indexed)]
        pub deposit_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "DepositProcessed", abi = "DepositProcessed(uint256,address,bool,uint256)")]
    pub struct DepositProcessedFilter {
        #[ethevent(indexed)]
        pub deposit_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub success: bool,
        pub shares: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "FeeCollected", abi = "FeeCollected(address,uint256,uint256,uint256)")]
    pub struct FeeCollectedFilter {
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub timestamp: ::ethers::core::types::U256,
        pub total_supply: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint64)")]
    pub struct InitializedFilter {
        pub version: u64,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "ModuleApproved", abi = "ModuleApproved(address)")]
    pub struct ModuleApprovedFilter {
        pub module: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "OwnershipTransferStarted",
        abi = "OwnershipTransferStarted(address,address)"
    )]
    pub struct OwnershipTransferStartedFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "OwnershipTransferred", abi = "OwnershipTransferred(address,address)")]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "PPTSAParamsSet",
        abi = "PPTSAParamsSet((uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),(uint256,int256,uint256,uint256))"
    )]
    pub struct PptsaparamsSetFilter {
        pub params: Pptsaparams,
        pub collateral_management_params: CollateralManagementParams,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "ShareKeeperUpdated", abi = "ShareKeeperUpdated(address,bool)")]
    pub struct ShareKeeperUpdatedFilter {
        pub keeper: ::ethers::core::types::Address,
        pub is_keeper: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SignatureRevoked", abi = "SignatureRevoked(address,bytes32)")]
    pub struct SignatureRevokedFilter {
        #[ethevent(indexed)]
        pub signer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub hash: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SignaturesDisabledUpdated", abi = "SignaturesDisabledUpdated(bool)")]
    pub struct SignaturesDisabledUpdatedFilter {
        pub enabled: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SignerUpdated", abi = "SignerUpdated(address,bool)")]
    pub struct SignerUpdatedFilter {
        #[ethevent(indexed)]
        pub signer: ::ethers::core::types::Address,
        pub is_signer: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "TSAParamsUpdated",
        abi = "TSAParamsUpdated((uint256,uint256,uint256,uint256,uint256,address))"
    )]
    pub struct TsaparamsUpdatedFilter {
        pub params: Tsaparams,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "WithdrawalProcessed",
        abi = "WithdrawalProcessed(uint256,address,bool,uint256,uint256)"
    )]
    pub struct WithdrawalProcessedFilter {
        #[ethevent(indexed)]
        pub withdrawal_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub beneficiary: ::ethers::core::types::Address,
        pub complete: bool,
        pub shares_processed: ::ethers::core::types::U256,
        pub amount_received: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "WithdrawalRequested", abi = "WithdrawalRequested(uint256,address,uint256)")]
    pub struct WithdrawalRequestedFilter {
        #[ethevent(indexed)]
        pub withdrawal_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub beneficiary: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TSAEvents {
        ActionSignedFilter(ActionSignedFilter),
        ApprovalFilter(ApprovalFilter),
        DepositInitiatedFilter(DepositInitiatedFilter),
        DepositProcessedFilter(DepositProcessedFilter),
        FeeCollectedFilter(FeeCollectedFilter),
        InitializedFilter(InitializedFilter),
        ModuleApprovedFilter(ModuleApprovedFilter),
        OwnershipTransferStartedFilter(OwnershipTransferStartedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PptsaparamsSetFilter(PptsaparamsSetFilter),
        ShareKeeperUpdatedFilter(ShareKeeperUpdatedFilter),
        SignatureRevokedFilter(SignatureRevokedFilter),
        SignaturesDisabledUpdatedFilter(SignaturesDisabledUpdatedFilter),
        SignerUpdatedFilter(SignerUpdatedFilter),
        TsaparamsUpdatedFilter(TsaparamsUpdatedFilter),
        TransferFilter(TransferFilter),
        WithdrawalProcessedFilter(WithdrawalProcessedFilter),
        WithdrawalRequestedFilter(WithdrawalRequestedFilter),
    }
    impl ::ethers::contract::EthLogDecode for TSAEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ActionSignedFilter::decode_log(log) {
                return Ok(TSAEvents::ActionSignedFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(TSAEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = DepositInitiatedFilter::decode_log(log) {
                return Ok(TSAEvents::DepositInitiatedFilter(decoded));
            }
            if let Ok(decoded) = DepositProcessedFilter::decode_log(log) {
                return Ok(TSAEvents::DepositProcessedFilter(decoded));
            }
            if let Ok(decoded) = FeeCollectedFilter::decode_log(log) {
                return Ok(TSAEvents::FeeCollectedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(TSAEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = ModuleApprovedFilter::decode_log(log) {
                return Ok(TSAEvents::ModuleApprovedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferStartedFilter::decode_log(log) {
                return Ok(TSAEvents::OwnershipTransferStartedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(TSAEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PptsaparamsSetFilter::decode_log(log) {
                return Ok(TSAEvents::PptsaparamsSetFilter(decoded));
            }
            if let Ok(decoded) = ShareKeeperUpdatedFilter::decode_log(log) {
                return Ok(TSAEvents::ShareKeeperUpdatedFilter(decoded));
            }
            if let Ok(decoded) = SignatureRevokedFilter::decode_log(log) {
                return Ok(TSAEvents::SignatureRevokedFilter(decoded));
            }
            if let Ok(decoded) = SignaturesDisabledUpdatedFilter::decode_log(log) {
                return Ok(TSAEvents::SignaturesDisabledUpdatedFilter(decoded));
            }
            if let Ok(decoded) = SignerUpdatedFilter::decode_log(log) {
                return Ok(TSAEvents::SignerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = TsaparamsUpdatedFilter::decode_log(log) {
                return Ok(TSAEvents::TsaparamsUpdatedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(TSAEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalProcessedFilter::decode_log(log) {
                return Ok(TSAEvents::WithdrawalProcessedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalRequestedFilter::decode_log(log) {
                return Ok(TSAEvents::WithdrawalRequestedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for TSAEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ActionSignedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositInitiatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositProcessedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeCollectedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ModuleApprovedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferStartedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PptsaparamsSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ShareKeeperUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignatureRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignaturesDisabledUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SignerUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TsaparamsUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalProcessedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalRequestedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ActionSignedFilter> for TSAEvents {
        fn from(value: ActionSignedFilter) -> Self {
            Self::ActionSignedFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalFilter> for TSAEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<DepositInitiatedFilter> for TSAEvents {
        fn from(value: DepositInitiatedFilter) -> Self {
            Self::DepositInitiatedFilter(value)
        }
    }
    impl ::core::convert::From<DepositProcessedFilter> for TSAEvents {
        fn from(value: DepositProcessedFilter) -> Self {
            Self::DepositProcessedFilter(value)
        }
    }
    impl ::core::convert::From<FeeCollectedFilter> for TSAEvents {
        fn from(value: FeeCollectedFilter) -> Self {
            Self::FeeCollectedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for TSAEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<ModuleApprovedFilter> for TSAEvents {
        fn from(value: ModuleApprovedFilter) -> Self {
            Self::ModuleApprovedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferStartedFilter> for TSAEvents {
        fn from(value: OwnershipTransferStartedFilter) -> Self {
            Self::OwnershipTransferStartedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for TSAEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PptsaparamsSetFilter> for TSAEvents {
        fn from(value: PptsaparamsSetFilter) -> Self {
            Self::PptsaparamsSetFilter(value)
        }
    }
    impl ::core::convert::From<ShareKeeperUpdatedFilter> for TSAEvents {
        fn from(value: ShareKeeperUpdatedFilter) -> Self {
            Self::ShareKeeperUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<SignatureRevokedFilter> for TSAEvents {
        fn from(value: SignatureRevokedFilter) -> Self {
            Self::SignatureRevokedFilter(value)
        }
    }
    impl ::core::convert::From<SignaturesDisabledUpdatedFilter> for TSAEvents {
        fn from(value: SignaturesDisabledUpdatedFilter) -> Self {
            Self::SignaturesDisabledUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<SignerUpdatedFilter> for TSAEvents {
        fn from(value: SignerUpdatedFilter) -> Self {
            Self::SignerUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<TsaparamsUpdatedFilter> for TSAEvents {
        fn from(value: TsaparamsUpdatedFilter) -> Self {
            Self::TsaparamsUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for TSAEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalProcessedFilter> for TSAEvents {
        fn from(value: WithdrawalProcessedFilter) -> Self {
            Self::WithdrawalProcessedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalRequestedFilter> for TSAEvents {
        fn from(value: WithdrawalRequestedFilter) -> Self {
            Self::WithdrawalRequestedFilter(value)
        }
    }
    ///Container type for all input parameters for the `acceptOwnership` function with signature `acceptOwnership()` and selector `0x79ba5097`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership()")]
    pub struct AcceptOwnershipCall;
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `approveModule` function with signature `approveModule(address,uint256)` and selector `0x5c69d2d3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "approveModule", abi = "approveModule(address,uint256)")]
    pub struct ApproveModuleCall {
        pub module: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `collectFee` function with signature `collectFee()` and selector `0xd4d5d32a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "collectFee", abi = "collectFee()")]
    pub struct CollectFeeCall;
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `getAccountValue` function with signature `getAccountValue(bool)` and selector `0xbd4dd391`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getAccountValue", abi = "getAccountValue(bool)")]
    pub struct GetAccountValueCall {
        pub include_pending: bool,
    }
    ///Container type for all input parameters for the `getActionTypedDataHash` function with signature `getActionTypedDataHash((uint256,uint256,address,bytes,uint256,address,address))` and selector `0x1ee037f4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getActionTypedDataHash",
        abi = "getActionTypedDataHash((uint256,uint256,address,bytes,uint256,address,address))"
    )]
    pub struct GetActionTypedDataHashCall {
        pub action: Action,
    }
    ///Container type for all input parameters for the `getBasePrice` function with signature `getBasePrice()` and selector `0xb49f4afd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getBasePrice", abi = "getBasePrice()")]
    pub struct GetBasePriceCall;
    ///Container type for all input parameters for the `getBaseTSAAddresses` function with signature `getBaseTSAAddresses()` and selector `0x3d43038c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getBaseTSAAddresses", abi = "getBaseTSAAddresses()")]
    pub struct GetBaseTSAAddressesCall;
    ///Container type for all input parameters for the `getCollateralManagementParams` function with signature `getCollateralManagementParams()` and selector `0x805230f5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getCollateralManagementParams", abi = "getCollateralManagementParams()")]
    pub struct GetCollateralManagementParamsCall;
    ///Container type for all input parameters for the `getNumShares` function with signature `getNumShares(uint256)` and selector `0x855cb21b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getNumShares", abi = "getNumShares(uint256)")]
    pub struct GetNumSharesCall {
        pub deposit_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getPPTSAAddresses` function with signature `getPPTSAAddresses()` and selector `0x88466ce4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPPTSAAddresses", abi = "getPPTSAAddresses()")]
    pub struct GetPPTSAAddressesCall;
    ///Container type for all input parameters for the `getPPTSAParams` function with signature `getPPTSAParams()` and selector `0x5ae7dd4e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPPTSAParams", abi = "getPPTSAParams()")]
    pub struct GetPPTSAParamsCall;
    ///Container type for all input parameters for the `getSharesValue` function with signature `getSharesValue(uint256)` and selector `0xdea0e746`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getSharesValue", abi = "getSharesValue(uint256)")]
    pub struct GetSharesValueCall {
        pub num_shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getSubAccountStats` function with signature `getSubAccountStats()` and selector `0xc3d5ce56`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getSubAccountStats", abi = "getSubAccountStats()")]
    pub struct GetSubAccountStatsCall;
    ///Container type for all input parameters for the `getTSAParams` function with signature `getTSAParams()` and selector `0xadd9d6af`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getTSAParams", abi = "getTSAParams()")]
    pub struct GetTSAParamsCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,(address,address,address,address,address,address,string,string),(address,address,address,address,address,address,bool,bool))` and selector `0x86d58cd5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "initialize",
        abi = "initialize(address,(address,address,address,address,address,address,string,string),(address,address,address,address,address,address,bool,bool))"
    )]
    pub struct InitializeCall {
        pub initial_owner: ::ethers::core::types::Address,
        pub init_params: BaseTSAInitParams,
        pub pp_init_params: PptsainitParams,
    }
    ///Container type for all input parameters for the `initiateDeposit` function with signature `initiateDeposit(uint256,address)` and selector `0x4b9b708b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "initiateDeposit", abi = "initiateDeposit(uint256,address)")]
    pub struct InitiateDepositCall {
        pub amount: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isActionSigned` function with signature `isActionSigned((uint256,uint256,address,bytes,uint256,address,address))` and selector `0x5e17c823`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "isActionSigned",
        abi = "isActionSigned((uint256,uint256,address,bytes,uint256,address,address))"
    )]
    pub struct IsActionSignedCall {
        pub action: Action,
    }
    ///Container type for all input parameters for the `isBlocked` function with signature `isBlocked()` and selector `0xd75f0da7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isBlocked", abi = "isBlocked()")]
    pub struct IsBlockedCall;
    ///Container type for all input parameters for the `isSigner` function with signature `isSigner(address)` and selector `0x7df73e27`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isSigner", abi = "isSigner(address)")]
    pub struct IsSignerCall {
        pub signer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isValidSignature` function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isValidSignature", abi = "isValidSignature(bytes32,bytes)")]
    pub struct IsValidSignatureCall {
        pub hash: [u8; 32],
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `lastFeeCollected` function with signature `lastFeeCollected()` and selector `0x750290da`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "lastFeeCollected", abi = "lastFeeCollected()")]
    pub struct LastFeeCollectedCall;
    ///Container type for all input parameters for the `lastSeenHash` function with signature `lastSeenHash()` and selector `0x8043cc18`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "lastSeenHash", abi = "lastSeenHash()")]
    pub struct LastSeenHashCall;
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pendingOwner` function with signature `pendingOwner()` and selector `0xe30c3978`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pendingOwner", abi = "pendingOwner()")]
    pub struct PendingOwnerCall;
    ///Container type for all input parameters for the `processDeposit` function with signature `processDeposit(uint256)` and selector `0x0cc2055a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "processDeposit", abi = "processDeposit(uint256)")]
    pub struct ProcessDepositCall {
        pub deposit_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `processDeposits` function with signature `processDeposits(uint256[])` and selector `0x2dd2f137`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "processDeposits", abi = "processDeposits(uint256[])")]
    pub struct ProcessDepositsCall {
        pub deposit_ids: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `processWithdrawalRequests` function with signature `processWithdrawalRequests(uint256)` and selector `0xe1bece85`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "processWithdrawalRequests", abi = "processWithdrawalRequests(uint256)")]
    pub struct ProcessWithdrawalRequestsCall {
        pub limit: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `queuedDeposit` function with signature `queuedDeposit(uint256)` and selector `0x8210d616`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "queuedDeposit", abi = "queuedDeposit(uint256)")]
    pub struct QueuedDepositCall {
        pub deposit_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `queuedWithdrawal` function with signature `queuedWithdrawal(uint256)` and selector `0xdc82bcbf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "queuedWithdrawal", abi = "queuedWithdrawal(uint256)")]
    pub struct QueuedWithdrawalCall {
        pub withdrawal_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `requestWithdrawal` function with signature `requestWithdrawal(uint256)` and selector `0x9ee679e8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "requestWithdrawal", abi = "requestWithdrawal(uint256)")]
    pub struct RequestWithdrawalCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `revokeActionSignature` function with signature `revokeActionSignature((uint256,uint256,address,bytes,uint256,address,address))` and selector `0x6adeac95`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "revokeActionSignature",
        abi = "revokeActionSignature((uint256,uint256,address,bytes,uint256,address,address))"
    )]
    pub struct RevokeActionSignatureCall {
        pub action: Action,
    }
    ///Container type for all input parameters for the `revokeSignature` function with signature `revokeSignature(bytes32)` and selector `0xc9fd8ee8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "revokeSignature", abi = "revokeSignature(bytes32)")]
    pub struct RevokeSignatureCall {
        pub typed_data_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `setPPTSAParams` function with signature `setPPTSAParams((uint256,int256,uint256,uint256),(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0xe3fc7570`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "setPPTSAParams",
        abi = "setPPTSAParams((uint256,int256,uint256,uint256),(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct SetPPTSAParamsCall {
        pub new_collateral_mgmt_params: CollateralManagementParams,
        pub pptsa_params: Pptsaparams,
    }
    ///Container type for all input parameters for the `setShareKeeper` function with signature `setShareKeeper(address,bool)` and selector `0x66ee2a1e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setShareKeeper", abi = "setShareKeeper(address,bool)")]
    pub struct SetShareKeeperCall {
        pub keeper: ::ethers::core::types::Address,
        pub is_keeper: bool,
    }
    ///Container type for all input parameters for the `setSignaturesDisabled` function with signature `setSignaturesDisabled(bool)` and selector `0x802e1d6d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setSignaturesDisabled", abi = "setSignaturesDisabled(bool)")]
    pub struct SetSignaturesDisabledCall {
        pub disabled: bool,
    }
    ///Container type for all input parameters for the `setSigner` function with signature `setSigner(address,bool)` and selector `0x31cb6105`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setSigner", abi = "setSigner(address,bool)")]
    pub struct SetSignerCall {
        pub signer: ::ethers::core::types::Address,
        pub is_signer: bool,
    }
    ///Container type for all input parameters for the `setTSAParams` function with signature `setTSAParams((uint256,uint256,uint256,uint256,uint256,address))` and selector `0xd80895ab`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "setTSAParams",
        abi = "setTSAParams((uint256,uint256,uint256,uint256,uint256,address))"
    )]
    pub struct SetTSAParamsCall {
        pub params: Tsaparams,
    }
    ///Container type for all input parameters for the `shareKeeper` function with signature `shareKeeper(address)` and selector `0xf34edc47`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "shareKeeper", abi = "shareKeeper(address)")]
    pub struct ShareKeeperCall {
        pub keeper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `signActionData` function with signature `signActionData((uint256,uint256,address,bytes,uint256,address,address),bytes)` and selector `0xa39cc91b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "signActionData",
        abi = "signActionData((uint256,uint256,address,bytes,uint256,address,address),bytes)"
    )]
    pub struct SignActionDataCall {
        pub action: Action,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `signaturesDisabled` function with signature `signaturesDisabled()` and selector `0x4e3ab0dc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "signaturesDisabled", abi = "signaturesDisabled()")]
    pub struct SignaturesDisabledCall;
    ///Container type for all input parameters for the `signedData` function with signature `signedData(bytes32)` and selector `0x34d5966a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "signedData", abi = "signedData(bytes32)")]
    pub struct SignedDataCall {
        pub hash: [u8; 32],
    }
    ///Container type for all input parameters for the `subAccount` function with signature `subAccount()` and selector `0x377ea3f5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "subAccount", abi = "subAccount()")]
    pub struct SubAccountCall;
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalPendingDeposits` function with signature `totalPendingDeposits()` and selector `0x0a881082`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "totalPendingDeposits", abi = "totalPendingDeposits()")]
    pub struct TotalPendingDepositsCall;
    ///Container type for all input parameters for the `totalPendingWithdrawals` function with signature `totalPendingWithdrawals()` and selector `0xa4563e03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "totalPendingWithdrawals", abi = "totalPendingWithdrawals()")]
    pub struct TotalPendingWithdrawalsCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TSACalls {
        AcceptOwnership(AcceptOwnershipCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        ApproveModule(ApproveModuleCall),
        BalanceOf(BalanceOfCall),
        CollectFee(CollectFeeCall),
        Decimals(DecimalsCall),
        GetAccountValue(GetAccountValueCall),
        GetActionTypedDataHash(GetActionTypedDataHashCall),
        GetBasePrice(GetBasePriceCall),
        GetBaseTSAAddresses(GetBaseTSAAddressesCall),
        GetCollateralManagementParams(GetCollateralManagementParamsCall),
        GetNumShares(GetNumSharesCall),
        GetPPTSAAddresses(GetPPTSAAddressesCall),
        GetPPTSAParams(GetPPTSAParamsCall),
        GetSharesValue(GetSharesValueCall),
        GetSubAccountStats(GetSubAccountStatsCall),
        GetTSAParams(GetTSAParamsCall),
        Initialize(InitializeCall),
        InitiateDeposit(InitiateDepositCall),
        IsActionSigned(IsActionSignedCall),
        IsBlocked(IsBlockedCall),
        IsSigner(IsSignerCall),
        IsValidSignature(IsValidSignatureCall),
        LastFeeCollected(LastFeeCollectedCall),
        LastSeenHash(LastSeenHashCall),
        Name(NameCall),
        Owner(OwnerCall),
        PendingOwner(PendingOwnerCall),
        ProcessDeposit(ProcessDepositCall),
        ProcessDeposits(ProcessDepositsCall),
        ProcessWithdrawalRequests(ProcessWithdrawalRequestsCall),
        QueuedDeposit(QueuedDepositCall),
        QueuedWithdrawal(QueuedWithdrawalCall),
        RenounceOwnership(RenounceOwnershipCall),
        RequestWithdrawal(RequestWithdrawalCall),
        RevokeActionSignature(RevokeActionSignatureCall),
        RevokeSignature(RevokeSignatureCall),
        SetPPTSAParams(SetPPTSAParamsCall),
        SetShareKeeper(SetShareKeeperCall),
        SetSignaturesDisabled(SetSignaturesDisabledCall),
        SetSigner(SetSignerCall),
        SetTSAParams(SetTSAParamsCall),
        ShareKeeper(ShareKeeperCall),
        SignActionData(SignActionDataCall),
        SignaturesDisabled(SignaturesDisabledCall),
        SignedData(SignedDataCall),
        SubAccount(SubAccountCall),
        Symbol(SymbolCall),
        TotalPendingDeposits(TotalPendingDepositsCall),
        TotalPendingWithdrawals(TotalPendingWithdrawalsCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for TSACalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <AcceptOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AcceptOwnership(decoded));
            }
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <ApproveModuleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ApproveModule(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <CollectFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CollectFee(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) =
                <GetAccountValueCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAccountValue(decoded));
            }
            if let Ok(decoded) =
                <GetActionTypedDataHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetActionTypedDataHash(decoded));
            }
            if let Ok(decoded) = <GetBasePriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetBasePrice(decoded));
            }
            if let Ok(decoded) =
                <GetBaseTSAAddressesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetBaseTSAAddresses(decoded));
            }
            if let Ok(decoded) =
                <GetCollateralManagementParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCollateralManagementParams(decoded));
            }
            if let Ok(decoded) = <GetNumSharesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetNumShares(decoded));
            }
            if let Ok(decoded) =
                <GetPPTSAAddressesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPPTSAAddresses(decoded));
            }
            if let Ok(decoded) =
                <GetPPTSAParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPPTSAParams(decoded));
            }
            if let Ok(decoded) =
                <GetSharesValueCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSharesValue(decoded));
            }
            if let Ok(decoded) =
                <GetSubAccountStatsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSubAccountStats(decoded));
            }
            if let Ok(decoded) = <GetTSAParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTSAParams(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <InitiateDepositCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InitiateDeposit(decoded));
            }
            if let Ok(decoded) =
                <IsActionSignedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsActionSigned(decoded));
            }
            if let Ok(decoded) = <IsBlockedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsBlocked(decoded));
            }
            if let Ok(decoded) = <IsSignerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsSigner(decoded));
            }
            if let Ok(decoded) =
                <IsValidSignatureCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsValidSignature(decoded));
            }
            if let Ok(decoded) =
                <LastFeeCollectedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LastFeeCollected(decoded));
            }
            if let Ok(decoded) = <LastSeenHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LastSeenHash(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PendingOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PendingOwner(decoded));
            }
            if let Ok(decoded) =
                <ProcessDepositCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProcessDeposit(decoded));
            }
            if let Ok(decoded) =
                <ProcessDepositsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProcessDeposits(decoded));
            }
            if let Ok(decoded) =
                <ProcessWithdrawalRequestsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProcessWithdrawalRequests(decoded));
            }
            if let Ok(decoded) = <QueuedDepositCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::QueuedDeposit(decoded));
            }
            if let Ok(decoded) =
                <QueuedWithdrawalCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::QueuedWithdrawal(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RequestWithdrawalCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequestWithdrawal(decoded));
            }
            if let Ok(decoded) =
                <RevokeActionSignatureCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevokeActionSignature(decoded));
            }
            if let Ok(decoded) =
                <RevokeSignatureCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevokeSignature(decoded));
            }
            if let Ok(decoded) =
                <SetPPTSAParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetPPTSAParams(decoded));
            }
            if let Ok(decoded) =
                <SetShareKeeperCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetShareKeeper(decoded));
            }
            if let Ok(decoded) =
                <SetSignaturesDisabledCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetSignaturesDisabled(decoded));
            }
            if let Ok(decoded) = <SetSignerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetSigner(decoded));
            }
            if let Ok(decoded) = <SetTSAParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetTSAParams(decoded));
            }
            if let Ok(decoded) = <ShareKeeperCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ShareKeeper(decoded));
            }
            if let Ok(decoded) =
                <SignActionDataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignActionData(decoded));
            }
            if let Ok(decoded) =
                <SignaturesDisabledCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignaturesDisabled(decoded));
            }
            if let Ok(decoded) = <SignedDataCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SignedData(decoded));
            }
            if let Ok(decoded) = <SubAccountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SubAccount(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalPendingDepositsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TotalPendingDeposits(decoded));
            }
            if let Ok(decoded) =
                <TotalPendingWithdrawalsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TotalPendingWithdrawals(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TSACalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AcceptOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Allowance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ApproveModule(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CollectFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Decimals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAccountValue(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetActionTypedDataHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBasePrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBaseTSAAddresses(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCollateralManagementParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNumShares(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPPTSAAddresses(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPPTSAParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSharesValue(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSubAccountStats(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTSAParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InitiateDeposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsActionSigned(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsBlocked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsSigner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsValidSignature(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LastFeeCollected(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LastSeenHash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingOwner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProcessDeposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProcessDeposits(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProcessWithdrawalRequests(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QueuedDeposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QueuedWithdrawal(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RequestWithdrawal(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevokeActionSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeSignature(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPPTSAParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetShareKeeper(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetSignaturesDisabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSigner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetTSAParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ShareKeeper(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SignActionData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SignaturesDisabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignedData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SubAccount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalPendingDeposits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalPendingWithdrawals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Transfer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for TSACalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AcceptOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproveModule(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::CollectFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAccountValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetActionTypedDataHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBasePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBaseTSAAddresses(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCollateralManagementParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetNumShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPPTSAAddresses(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPPTSAParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSharesValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSubAccountStats(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTSAParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitiateDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsActionSigned(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsBlocked(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsValidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastFeeCollected(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastSeenHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessDeposits(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessWithdrawalRequests(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueuedDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueuedWithdrawal(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestWithdrawal(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeActionSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPPTSAParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetShareKeeper(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSignaturesDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTSAParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::ShareKeeper(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignActionData(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignaturesDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedData(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubAccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalPendingDeposits(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalPendingWithdrawals(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AcceptOwnershipCall> for TSACalls {
        fn from(value: AcceptOwnershipCall) -> Self {
            Self::AcceptOwnership(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for TSACalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for TSACalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<ApproveModuleCall> for TSACalls {
        fn from(value: ApproveModuleCall) -> Self {
            Self::ApproveModule(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for TSACalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<CollectFeeCall> for TSACalls {
        fn from(value: CollectFeeCall) -> Self {
            Self::CollectFee(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for TSACalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<GetAccountValueCall> for TSACalls {
        fn from(value: GetAccountValueCall) -> Self {
            Self::GetAccountValue(value)
        }
    }
    impl ::core::convert::From<GetActionTypedDataHashCall> for TSACalls {
        fn from(value: GetActionTypedDataHashCall) -> Self {
            Self::GetActionTypedDataHash(value)
        }
    }
    impl ::core::convert::From<GetBasePriceCall> for TSACalls {
        fn from(value: GetBasePriceCall) -> Self {
            Self::GetBasePrice(value)
        }
    }
    impl ::core::convert::From<GetBaseTSAAddressesCall> for TSACalls {
        fn from(value: GetBaseTSAAddressesCall) -> Self {
            Self::GetBaseTSAAddresses(value)
        }
    }
    impl ::core::convert::From<GetCollateralManagementParamsCall> for TSACalls {
        fn from(value: GetCollateralManagementParamsCall) -> Self {
            Self::GetCollateralManagementParams(value)
        }
    }
    impl ::core::convert::From<GetNumSharesCall> for TSACalls {
        fn from(value: GetNumSharesCall) -> Self {
            Self::GetNumShares(value)
        }
    }
    impl ::core::convert::From<GetPPTSAAddressesCall> for TSACalls {
        fn from(value: GetPPTSAAddressesCall) -> Self {
            Self::GetPPTSAAddresses(value)
        }
    }
    impl ::core::convert::From<GetPPTSAParamsCall> for TSACalls {
        fn from(value: GetPPTSAParamsCall) -> Self {
            Self::GetPPTSAParams(value)
        }
    }
    impl ::core::convert::From<GetSharesValueCall> for TSACalls {
        fn from(value: GetSharesValueCall) -> Self {
            Self::GetSharesValue(value)
        }
    }
    impl ::core::convert::From<GetSubAccountStatsCall> for TSACalls {
        fn from(value: GetSubAccountStatsCall) -> Self {
            Self::GetSubAccountStats(value)
        }
    }
    impl ::core::convert::From<GetTSAParamsCall> for TSACalls {
        fn from(value: GetTSAParamsCall) -> Self {
            Self::GetTSAParams(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for TSACalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<InitiateDepositCall> for TSACalls {
        fn from(value: InitiateDepositCall) -> Self {
            Self::InitiateDeposit(value)
        }
    }
    impl ::core::convert::From<IsActionSignedCall> for TSACalls {
        fn from(value: IsActionSignedCall) -> Self {
            Self::IsActionSigned(value)
        }
    }
    impl ::core::convert::From<IsBlockedCall> for TSACalls {
        fn from(value: IsBlockedCall) -> Self {
            Self::IsBlocked(value)
        }
    }
    impl ::core::convert::From<IsSignerCall> for TSACalls {
        fn from(value: IsSignerCall) -> Self {
            Self::IsSigner(value)
        }
    }
    impl ::core::convert::From<IsValidSignatureCall> for TSACalls {
        fn from(value: IsValidSignatureCall) -> Self {
            Self::IsValidSignature(value)
        }
    }
    impl ::core::convert::From<LastFeeCollectedCall> for TSACalls {
        fn from(value: LastFeeCollectedCall) -> Self {
            Self::LastFeeCollected(value)
        }
    }
    impl ::core::convert::From<LastSeenHashCall> for TSACalls {
        fn from(value: LastSeenHashCall) -> Self {
            Self::LastSeenHash(value)
        }
    }
    impl ::core::convert::From<NameCall> for TSACalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for TSACalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PendingOwnerCall> for TSACalls {
        fn from(value: PendingOwnerCall) -> Self {
            Self::PendingOwner(value)
        }
    }
    impl ::core::convert::From<ProcessDepositCall> for TSACalls {
        fn from(value: ProcessDepositCall) -> Self {
            Self::ProcessDeposit(value)
        }
    }
    impl ::core::convert::From<ProcessDepositsCall> for TSACalls {
        fn from(value: ProcessDepositsCall) -> Self {
            Self::ProcessDeposits(value)
        }
    }
    impl ::core::convert::From<ProcessWithdrawalRequestsCall> for TSACalls {
        fn from(value: ProcessWithdrawalRequestsCall) -> Self {
            Self::ProcessWithdrawalRequests(value)
        }
    }
    impl ::core::convert::From<QueuedDepositCall> for TSACalls {
        fn from(value: QueuedDepositCall) -> Self {
            Self::QueuedDeposit(value)
        }
    }
    impl ::core::convert::From<QueuedWithdrawalCall> for TSACalls {
        fn from(value: QueuedWithdrawalCall) -> Self {
            Self::QueuedWithdrawal(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for TSACalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RequestWithdrawalCall> for TSACalls {
        fn from(value: RequestWithdrawalCall) -> Self {
            Self::RequestWithdrawal(value)
        }
    }
    impl ::core::convert::From<RevokeActionSignatureCall> for TSACalls {
        fn from(value: RevokeActionSignatureCall) -> Self {
            Self::RevokeActionSignature(value)
        }
    }
    impl ::core::convert::From<RevokeSignatureCall> for TSACalls {
        fn from(value: RevokeSignatureCall) -> Self {
            Self::RevokeSignature(value)
        }
    }
    impl ::core::convert::From<SetPPTSAParamsCall> for TSACalls {
        fn from(value: SetPPTSAParamsCall) -> Self {
            Self::SetPPTSAParams(value)
        }
    }
    impl ::core::convert::From<SetShareKeeperCall> for TSACalls {
        fn from(value: SetShareKeeperCall) -> Self {
            Self::SetShareKeeper(value)
        }
    }
    impl ::core::convert::From<SetSignaturesDisabledCall> for TSACalls {
        fn from(value: SetSignaturesDisabledCall) -> Self {
            Self::SetSignaturesDisabled(value)
        }
    }
    impl ::core::convert::From<SetSignerCall> for TSACalls {
        fn from(value: SetSignerCall) -> Self {
            Self::SetSigner(value)
        }
    }
    impl ::core::convert::From<SetTSAParamsCall> for TSACalls {
        fn from(value: SetTSAParamsCall) -> Self {
            Self::SetTSAParams(value)
        }
    }
    impl ::core::convert::From<ShareKeeperCall> for TSACalls {
        fn from(value: ShareKeeperCall) -> Self {
            Self::ShareKeeper(value)
        }
    }
    impl ::core::convert::From<SignActionDataCall> for TSACalls {
        fn from(value: SignActionDataCall) -> Self {
            Self::SignActionData(value)
        }
    }
    impl ::core::convert::From<SignaturesDisabledCall> for TSACalls {
        fn from(value: SignaturesDisabledCall) -> Self {
            Self::SignaturesDisabled(value)
        }
    }
    impl ::core::convert::From<SignedDataCall> for TSACalls {
        fn from(value: SignedDataCall) -> Self {
            Self::SignedData(value)
        }
    }
    impl ::core::convert::From<SubAccountCall> for TSACalls {
        fn from(value: SubAccountCall) -> Self {
            Self::SubAccount(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for TSACalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalPendingDepositsCall> for TSACalls {
        fn from(value: TotalPendingDepositsCall) -> Self {
            Self::TotalPendingDeposits(value)
        }
    }
    impl ::core::convert::From<TotalPendingWithdrawalsCall> for TSACalls {
        fn from(value: TotalPendingWithdrawalsCall) -> Self {
            Self::TotalPendingWithdrawals(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for TSACalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for TSACalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for TSACalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for TSACalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `getAccountValue` function with signature `getAccountValue(bool)` and selector `0xbd4dd391`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetAccountValueReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getActionTypedDataHash` function with signature `getActionTypedDataHash((uint256,uint256,address,bytes,uint256,address,address))` and selector `0x1ee037f4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetActionTypedDataHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getBasePrice` function with signature `getBasePrice()` and selector `0xb49f4afd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetBasePriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getBaseTSAAddresses` function with signature `getBaseTSAAddresses()` and selector `0x3d43038c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetBaseTSAAddressesReturn(pub BaseTSAAddresses);
    ///Container type for all return fields from the `getCollateralManagementParams` function with signature `getCollateralManagementParams()` and selector `0x805230f5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetCollateralManagementParamsReturn(pub CollateralManagementParams);
    ///Container type for all return fields from the `getNumShares` function with signature `getNumShares(uint256)` and selector `0x855cb21b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetNumSharesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPPTSAAddresses` function with signature `getPPTSAAddresses()` and selector `0x88466ce4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPPTSAAddressesReturn(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all return fields from the `getPPTSAParams` function with signature `getPPTSAParams()` and selector `0x5ae7dd4e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPPTSAParamsReturn(pub Pptsaparams);
    ///Container type for all return fields from the `getSharesValue` function with signature `getSharesValue(uint256)` and selector `0xdea0e746`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetSharesValueReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getSubAccountStats` function with signature `getSubAccountStats()` and selector `0xc3d5ce56`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetSubAccountStatsReturn {
        pub open_spreads: ::ethers::core::types::U256,
        pub base_balance: ::ethers::core::types::U256,
        pub cash_balance: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `getTSAParams` function with signature `getTSAParams()` and selector `0xadd9d6af`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetTSAParamsReturn(pub Tsaparams);
    ///Container type for all return fields from the `initiateDeposit` function with signature `initiateDeposit(uint256,address)` and selector `0x4b9b708b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct InitiateDepositReturn {
        pub deposit_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `isActionSigned` function with signature `isActionSigned((uint256,uint256,address,bytes,uint256,address,address))` and selector `0x5e17c823`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsActionSignedReturn(pub bool);
    ///Container type for all return fields from the `isBlocked` function with signature `isBlocked()` and selector `0xd75f0da7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsBlockedReturn(pub bool);
    ///Container type for all return fields from the `isSigner` function with signature `isSigner(address)` and selector `0x7df73e27`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsSignerReturn(pub bool);
    ///Container type for all return fields from the `isValidSignature` function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsValidSignatureReturn {
        pub magic_value: [u8; 4],
    }
    ///Container type for all return fields from the `lastFeeCollected` function with signature `lastFeeCollected()` and selector `0x750290da`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LastFeeCollectedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lastSeenHash` function with signature `lastSeenHash()` and selector `0x8043cc18`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LastSeenHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pendingOwner` function with signature `pendingOwner()` and selector `0xe30c3978`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PendingOwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `queuedDeposit` function with signature `queuedDeposit(uint256)` and selector `0x8210d616`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct QueuedDepositReturn(pub DepositRequest);
    ///Container type for all return fields from the `queuedWithdrawal` function with signature `queuedWithdrawal(uint256)` and selector `0xdc82bcbf`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct QueuedWithdrawalReturn(pub WithdrawalRequest);
    ///Container type for all return fields from the `requestWithdrawal` function with signature `requestWithdrawal(uint256)` and selector `0x9ee679e8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RequestWithdrawalReturn {
        pub withdrawal_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `shareKeeper` function with signature `shareKeeper(address)` and selector `0xf34edc47`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ShareKeeperReturn(pub bool);
    ///Container type for all return fields from the `signaturesDisabled` function with signature `signaturesDisabled()` and selector `0x4e3ab0dc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignaturesDisabledReturn(pub bool);
    ///Container type for all return fields from the `signedData` function with signature `signedData(bytes32)` and selector `0x34d5966a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedDataReturn(pub bool);
    ///Container type for all return fields from the `subAccount` function with signature `subAccount()` and selector `0x377ea3f5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SubAccountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalPendingDeposits` function with signature `totalPendingDeposits()` and selector `0x0a881082`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TotalPendingDepositsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalPendingWithdrawals` function with signature `totalPendingWithdrawals()` and selector `0xa4563e03`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TotalPendingWithdrawalsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TransferFromReturn(pub bool);
    ///`BaseTSAAddresses(address,address,address,address,address,address,address)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BaseTSAAddresses {
        pub sub_accounts: ::ethers::core::types::Address,
        pub auction: ::ethers::core::types::Address,
        pub wrapped_deposit_asset: ::ethers::core::types::Address,
        pub cash: ::ethers::core::types::Address,
        pub deposit_asset: ::ethers::core::types::Address,
        pub manager: ::ethers::core::types::Address,
        pub matching: ::ethers::core::types::Address,
    }
    ///`BaseTSAInitParams(address,address,address,address,address,address,string,string)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BaseTSAInitParams {
        pub sub_accounts: ::ethers::core::types::Address,
        pub auction: ::ethers::core::types::Address,
        pub cash: ::ethers::core::types::Address,
        pub wrapped_deposit_asset: ::ethers::core::types::Address,
        pub manager: ::ethers::core::types::Address,
        pub matching: ::ethers::core::types::Address,
        pub symbol: ::std::string::String,
        pub name: ::std::string::String,
    }
    ///`DepositRequest(address,uint256,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DepositRequest {
        pub recipient: ::ethers::core::types::Address,
        pub amount_deposit_asset: ::ethers::core::types::U256,
        pub timestamp: ::ethers::core::types::U256,
        pub shares_received: ::ethers::core::types::U256,
    }
    ///`Tsaparams(uint256,uint256,uint256,uint256,uint256,address)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Tsaparams {
        pub deposit_cap: ::ethers::core::types::U256,
        pub min_deposit_value: ::ethers::core::types::U256,
        pub deposit_scale: ::ethers::core::types::U256,
        pub withdraw_scale: ::ethers::core::types::U256,
        pub management_fee: ::ethers::core::types::U256,
        pub fee_recipient: ::ethers::core::types::Address,
    }
    ///`WithdrawalRequest(address,uint256,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct WithdrawalRequest {
        pub beneficiary: ::ethers::core::types::Address,
        pub amount_shares: ::ethers::core::types::U256,
        pub timestamp: ::ethers::core::types::U256,
        pub assets_received: ::ethers::core::types::U256,
    }
    ///`CollateralManagementParams(uint256,int256,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CollateralManagementParams {
        pub fee_factor: ::ethers::core::types::U256,
        pub spot_transaction_leniency: ::ethers::core::types::I256,
        pub worst_spot_sell_price: ::ethers::core::types::U256,
        pub worst_spot_buy_price: ::ethers::core::types::U256,
    }
    ///`Action(uint256,uint256,address,bytes,uint256,address,address)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Action {
        pub subaccount_id: ::ethers::core::types::U256,
        pub nonce: ::ethers::core::types::U256,
        pub module: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
        pub expiry: ::ethers::core::types::U256,
        pub owner: ::ethers::core::types::Address,
        pub signer: ::ethers::core::types::Address,
    }
    ///`PptsainitParams(address,address,address,address,address,address,bool,bool)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PptsainitParams {
        pub base_feed: ::ethers::core::types::Address,
        pub deposit_module: ::ethers::core::types::Address,
        pub withdrawal_module: ::ethers::core::types::Address,
        pub trade_module: ::ethers::core::types::Address,
        pub rfq_module: ::ethers::core::types::Address,
        pub option_asset: ::ethers::core::types::Address,
        pub is_call_spread: bool,
        pub is_long_spread: bool,
    }
    ///`Pptsaparams(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Pptsaparams {
        pub max_mark_value_to_strike_diff_ratio: ::ethers::core::types::U256,
        pub min_mark_value_to_strike_diff_ratio: ::ethers::core::types::U256,
        pub strike_diff: ::ethers::core::types::U256,
        pub max_total_cost_tolerance: ::ethers::core::types::U256,
        pub max_loss_percent_of_tvl: ::ethers::core::types::U256,
        pub neg_max_cash_tolerance: ::ethers::core::types::U256,
        pub min_signature_expiry: ::ethers::core::types::U256,
        pub max_signature_expiry: ::ethers::core::types::U256,
        pub option_min_time_to_expiry: ::ethers::core::types::U256,
        pub option_max_time_to_expiry: ::ethers::core::types::U256,
    }
}

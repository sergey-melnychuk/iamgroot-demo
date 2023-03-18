fn main() {
    call(
        1,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getBlockWithTxHashes",
            "params": {
                "block_id": {"block_hash": "0x3f65ef25e87a83d92f32f5e4869a33580f9db47ec980c1ff27bdb5151914de5"}
            }
        }),
    );

    call(
        2,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getBlockWithTxs",
            "params": {
                "block_id": {"block_number": 123456}
            }
        }),
    );

    call(
        3,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getStateUpdate",
            "params": {
                "block_id": {"block_number": 123456}
            }
        }),
    );

    call(
        401,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getStorageAt",
            "params": {
                "contract_address": "0x3e10411edafd29dfe6d427d03e35cb261b7a5efeee61bf73909ada048c029b9",
                "key": "0x02",
                "block_id": {"block_number": 123456},
            }
        }),
    );

    call(
        402,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getStorageAt",
            "params": [
                "0x3e10411edafd29dfe6d427d03e35cb261b7a5efeee61bf73909ada048c029b9",
                "0x02",
                {"block_number": 123456}
            ]
        }),
    );

    call(
        5,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getTransactionByHash",
            "params": {
                "transaction_hash": "0x131f1c81dc28ed4ec07971dbe95ddbf46e814a3f8bddbffb5d4a1b7717d729a",
            }
        }),
    );

    call(
        6,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getTransactionByHash",
            "params": {
                "transaction_hash": "0x131f1c81dc28ed4ec07971dbe95ddbf46e814a3f8bddbffb5d4a1b7717d729a",
            }
        }),
    );

    call(
        7,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getTransactionByBlockIdAndIndex",
            "params": {
                "block_id": {"block_number": 123456},
                "index": 32
            }
        }),
    );

    call(
        8,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getTransactionReceipt",
            "params": {
                "transaction_hash": "0x131f1c81dc28ed4ec07971dbe95ddbf46e814a3f8bddbffb5d4a1b7717d729a"
            }
        }),
    );

    call(
        9,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getClass",
            "params": {
                "block_id": {"block_number": 123456},
                "class_hash": "0x71c3c99f5cf76fc19945d4b8b7d34c7c5528f22730d56192b50c6bbfd338a64"
            }
        }),
    );

    call(
        10,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getClassAt",
            "params": {
                "block_id": {"block_number": 123456},
                "contract_address": "0x3e10411edafd29dfe6d427d03e35cb261b7a5efeee61bf73909ada048c029b9"
            }
        }),
    );

    call(
        11,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getClassHashAt",
            "params": {
                "block_id": {"block_number": 123456},
                "contract_address": "0x3e10411edafd29dfe6d427d03e35cb261b7a5efeee61bf73909ada048c029b9"
            }
        }),
    );

    call(
        12,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getBlockTransactionCount",
            "params": {
                "block_id": "latest"
            }
        }),
    );

    call(
        13,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_call",
            "params": [
                {
                    "entry_point_selector": "0x79dc0da7c54b95f10aa182ad0a46400db63156920adb65eca2654c0945a463",
                    "calldata": ["0x2"],
                    "contract_address": "0x3e10411edafd29dfe6d427d03e35cb261b7a5efeee61bf73909ada048c029b9"
                },
                {"block_number": 123456}
            ]
        }),
    );

    call(
        1401,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_estimateFee",
            "params": {
                "request": {
                    "nonce": "0x1",
                    "version": "0x1",
                    "max_fee": "0x3",
                    "signature": [
                        "0x4",
                        "0x5"
                    ],
                    "calldata": [
                        "0x6",
                        "0x7"
                    ],
                    "entry_point_selector": "0x8",
                    "contract_address": "0x3e10411edafd29dfe6d427d03e35cb261b7a5efeee61bf73909ada048c029b9",
                    "type": "INVOKE"
                },
                "block_id": {"block_number": 123456}
            },
        }),
    );

    call(
        1402,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_estimateFee",
            "params": {
                "request": {
                    "nonce": "0x1",
                    "version": "0x1",
                    "max_fee": "0x3",
                    "signature": [
                        "0x4",
                        "0x5"
                    ],
                    "sender_address": "0xA",
                    "calldata": [
                        "0xB",
                        "0xC"
                    ],
                    "type": "INVOKE"
                },
                "block_id": {"block_number": 123456}
            },
        }),
    );

    call(
        15,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_blockNumber"
        }),
    );

    call(
        16,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_blockHashAndNumber"
        }),
    );

    call(
        17,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_chainId"
        }),
    );

    call(
        18,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_pendingTransactions"
        }),
    );

    call(
        19,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_syncing"
        }),
    );

    call(
        20,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getEvents",
            "params": {
                "filter": {
                    "to_block": {"block_number": 200},
                    "from_block": {"block_number": 100},
                    "address": "0xA",
                    "keys": ["0x1", "0x2", "0x3", "0x4"],
                    "chunk_size": 42
                }
            }
        }),
    );

    call(
        21,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getNonce",
            "params": {
                "block_id": {"block_number": 123456},
                "contract_address": "0x3e10411edafd29dfe6d427d03e35cb261b7a5efeee61bf73909ada048c029b9"
            }
        }),
    );

    call(
        2201,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_addInvokeTransaction",
            "params": {
                "invoke_transaction": {
                    "max_fee": "0x1",
                    "version": "0x1",
                    "nonce": "0x3",
                    "signature": [
                        "0x4"
                    ],
                    "type": "INVOKE",
                    "calldata": [
                        "0x6",
                        "0x7"
                    ],
                    "entry_point_selector": "0x8",
                    "contract_address": "0x3e10411edafd29dfe6d427d03e35cb261b7a5efeee61bf73909ada048c029b9"
                }
            }
        }),
    );

    call(
        2202,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_addInvokeTransaction",
            "params": {
                "invoke_transaction": {
                    "max_fee": "0x1",
                    "version": "0x1",
                    "nonce": "0x3",
                    "signature": [
                        "0x4"
                    ],
                    "type": "INVOKE",
                    "sender_address": "0xA",
                    "calldata": [
                        "0xB",
                        "0xC"
                    ]
                }
            }
        }),
    );

    call(
        2301,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_addDeclareTransaction",
            "params": {
                "declare_transaction": {
                    "max_fee": "0x1",
                    "version": "0x1",
                    "nonce": "0x3",
                    "signature": [
                        "0x4"
                    ],
                    "contract_class": {
                        "abi": [],
                        "entry_points_by_type": {
                            "CONSTRUCTOR": [],
                            "EXTERNAL": [
                                {
                                    "offset": "0x3a",
                                    "selector": "0x362398bec32bc0ebb411203221a35a0301193a96f317ebe5e40be9f60d15320"
                                },
                                {
                                    "offset": "0x5b",
                                    "selector": "0x39e11d48192e4333233c7eb19d10ad67c362bb28580c604d67884c85da39695"
                                }
                            ],
                            "L1_HANDLER": []
                        },
                        "program": "just-some-string"
                    },
                    "sender_address": "0xA",
                    "type": "DECLARE"
                }
            }
        }),
    );

    call(
        2302,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_addDeclareTransaction",
            "params": {
                "declare_transaction": {
                    "max_fee": "0x1",
                    "version": "0x0",
                    "nonce": "0x3",
                    "signature": [
                        "0x4"
                    ],
                    "contract_class": {
                        "abi": "just-another-string",
                        "entry_points_by_type": {
                            "CONSTRUCTOR": [],
                            "EXTERNAL": [
                                {
                                    "offset": "0x3a",
                                    "selector": "0x362398bec32bc0ebb411203221a35a0301193a96f317ebe5e40be9f60d15320"
                                },
                                {
                                    "offset": "0x5b",
                                    "selector": "0x39e11d48192e4333233c7eb19d10ad67c362bb28580c604d67884c85da39695"
                                }
                            ],
                            "L1_HANDLER": []
                        },
                        "sierra_program": [
                            "0xAA",
                            "0xBB",
                            "0xCC"
                        ],
                        "sierra_version": "some-version"
                    },
                    "sender_address": "0xC",
                    "type": "DECLARE"
                }
            }
        }),
    );

    call(
        24,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_addDeployAccountTransaction",
            "params": {
                "deploy_account_transaction": {
                    "max_fee": "0x1",
                    "version": "0x1",
                    "nonce": "0x3",
                    "signature": [
                        "0x4"
                    ],
                    "contract_address_salt": "0x5",
                    "type": "DEPLOY_ACCOUNT",
                    "class_hash": "0x7",
                    "constructor_calldata": [
                        "0x8"
                    ]
                }
            }
        }),
    );
}

const URL: &str = "http://localhost:9000/api";
// const URL: &str = concat!(
//     "https://starknet-goerli.infura.io/v3/",
//     env!("INFURA_TOKEN")
// );

fn call(id: i64, json: serde_json::Value) {
    use iamgroot::jsonrpc;

    let req: jsonrpc::Request = serde_json::from_value(json).unwrap();
    let req = req.with_id(jsonrpc::Id::Number(id));
    let json = serde_json::to_string(&req).unwrap();
    println!("\n>>> {}", json);

    let client = reqwest::blocking::Client::new();
    let res: jsonrpc::Response = client.post(URL).json(&req).send().unwrap().json().unwrap();
    println!("<<< {}", serde_json::to_string(&res).unwrap());
}

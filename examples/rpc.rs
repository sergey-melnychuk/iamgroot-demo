fn main() {
    call(
        1,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getBlockWithTxHashes",
            "params": {
                "block_id": "0xFACE"
            }
        }),
    );

    call(
        2,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getBlockWithTxs",
            "params": {
                "block_id": 123456789
            }
        }),
    );

    call(
        3,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getStateUpdate",
            "params": {
                "block_id": "PENDING"
            }
        }),
    );

    call(
        4,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getStorageAt",
            "params": {
                "contract_address": "0x1",
                "key": "0x02",
                "block_id": 42,
            }
        }),
    );

    call(
        5,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getTransactionByHash",
            "params": {
                "transaction_hash": "0xcafebabe",
            }
        }),
    );

    call(
        6,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getTransactionByHash",
            "params": {
                "transaction_hash": "0xcafebabe",
            }
        }),
    );

    call(
        7,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getTransactionByBlockIdAndIndex",
            "params": {
                "block_id": 42,
                "index": 24
            }
        }),
    );

    call(
        8,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getTransactionReceipt",
            "params": {
                "transaction_hash": "0x1"
            }
        }),
    );

    call(
        9,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getClass",
            "params": {
                "block_id": 1,
                "class_hash": "0x1"
            }
        }),
    );

    call(
        10,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getClassAt",
            "params": {
                "block_id": 42,
                "contract_address": "0xFF"
            }
        }),
    );

    call(
        11,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getClassHashAt",
            "params": {
                "block_id": "PENDING",
                "contract_address": "0x1"
            }
        }),
    );

    call(
        12,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getBlockTransactionCount",
            "params": {
                "block_id": "LATEST"
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
                    "entry_point_selector": "0x1",
                    "calldata": ["0x2"],
                    "contract_address": "0x3"
                },
                42
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
                    "version": "0x2",
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
                    "contract_address": "0x9",
                    "type": "INVOKE"
                },
                "block_id": 1
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
                    "version": "0x2",
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
                "block_id": 1
            },
        }),
    );

    call(
        15,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_blockNumber",
            "params": []
        }),
    );

    call(
        16,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_blockHashAndNumber",
            "params": []
        }),
    );

    call(
        17,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_chainId",
            "params": []
        }),
    );

    call(
        18,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_pendingTransactions",
            "params": {}
        }),
    );

    call(
        19,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_syncing",
            "params": []
        }),
    );

    call(
        20,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_getEvents",
            "params": {
                "filter": {
                    "to_block": 200,
                    "from_block": 100,
                    "address": "0xA",
                    "keys": [
                        ["0x1", "0x2"],
                        ["0x3", "0x4"]
                    ],
                    "continuation_token": "req-token-0",
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
                "block_id": 12,
                "contract_address": "0x1"
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
                    "version": "0x2",
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
                    "contract_address": "0x9"
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
                    "version": "0x2",
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
        23,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_addDeclareTransaction",
            "params": {
                "max_fee": "0x1",
                "version": "0x2",
                "nonce": "0x3",
                "signature": [
                    "0x4"
                ],
                "contract_address_salt": "0x5",
                "type": "DEPLOYACCOUNT",
                "class_hash": "0x7",
                "constructor_calldata": [
                    "0x8"
                ]
            }
        }),
    );

    call(
        24,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "starknet_addDeployAccountTransaction",
            "params": {
                "max_fee": "0x1",
                "version": "0x2",
                "nonce": "0x3",
                "signature": [
                    "0x4"
                ],
                "contract_address_salt": "0x5",
                "type": "DEPLOYACCOUNT",
                "class_hash": "0x7",
                "constructor_calldata": [
                    "0x8"
                ]
            }
        }),
    );
}

fn call(id: i64, json: serde_json::Value) {
    use iamgroot::jsonrpc;

    let req: jsonrpc::Request = serde_json::from_value(json).unwrap();
    let req = req.with_id(jsonrpc::Id::Number(id));
    let json = serde_json::to_string(&req).unwrap();
    println!("\n>>> {}", json);

    let client = reqwest::blocking::Client::new();
    let res: jsonrpc::Response = client
        .post("http://localhost:3000/api")
        .json(&req)
        .send()
        .unwrap()
        .json()
        .unwrap();
    println!("<<< {}", serde_json::to_string(&res).unwrap());
}

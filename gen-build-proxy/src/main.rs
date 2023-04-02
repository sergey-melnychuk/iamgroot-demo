use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use dotenv::dotenv;
use iamgroot::jsonrpc;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::task::spawn_blocking;

#[derive(Clone)]
struct Context {
    starknet: Arc<gen::client::Client>,
    ethereum: Arc<ethereum::gen::client::Client>,
}

// curl -X POST --data '{"jsonrpc":"2.0","id":1,"method":"eth_getBlockByNumber","params":["0x10165f5",true]}' https://mainnet.infura.io/v3/${INFURA_TOKEN}
// curl -X POST -H 'Content-type: application/json' --data '{"jsonrpc":"2.0","id":1,"method":"eth_getBlockByNumber","params":["0x10165f5",true]}' http://localhost:9000/ethereum/proxy

// curl -X POST --data '{"jsonrpc":"2.0","id":1,"method":"eth_getTransactionByBlockNumberAndIndex","params":["0x10165f5","0x1"]}' https://mainnet.infura.io/v3/${INFURA_TOKEN}
// curl -X POST -H 'Content-type: application/json' --data '{"jsonrpc":"2.0","id":1,"method":"eth_getTransactionByBlockNumberAndIndex","params":["0x10165f5","0x1"]}' http://localhost:9000/ethereum/proxy

#[tokio::main]
async fn main() {
    dotenv().ok();

    let starknet_url = std::env::var("STARKNET_URL").expect("STARKNET_URL");
    let ethereum_url = std::env::var("ETHEREUM_URL").expect("ETHEREUM_URL");

    let starknet = gen::client::Client::new(&starknet_url);
    let ethereum = ethereum::gen::client::Client::new(&ethereum_url);
    let ctx = Context {
        starknet: Arc::new(starknet),
        ethereum: Arc::new(ethereum),
    };

    let app = Router::new()
        .route("/api", post(handle_request))
        .route("/starknet/proxy", post(proxy_request_starknet))
        .route("/ethereum/proxy", post(proxy_request_ethereum))
        .with_state(ctx);

    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum Request {
    Single(jsonrpc::Request),
    Batch(Vec<jsonrpc::Request>),
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum Response {
    Single(jsonrpc::Response),
    Batch(Vec<jsonrpc::Response>),
}

async fn handle_request(
    State(state): State<Context>,
    Json(req): Json<Request>,
) -> impl IntoResponse {
    match req {
        Request::Single(req) => {
            println!("method: {}", req.method);
            let res = gen::handle(&state, &req);
            Json(Response::Single(res))
        }
        Request::Batch(req) => {
            let res = req
                .into_iter()
                .map(|req| gen::handle(&state, &req))
                .collect::<Vec<_>>();
            Json(Response::Batch(res))
        }
    }
}

async fn proxy_request_starknet(
    State(state): State<Context>,
    Json(req): Json<Request>,
) -> impl IntoResponse {
    match req {
        Request::Single(req) => {
            println!("proxy: {}", req.method);
            let rpc = state.starknet.clone();
            let res = spawn_blocking(move || gen::handle(rpc.as_ref(), &req))
                .await
                .unwrap();
            Json(Response::Single(res))
        }
        Request::Batch(requests) => {
            let mut responses = Vec::with_capacity(requests.len());
            for req in requests {
                println!("proxy: {}", req.method);
                let rpc = state.starknet.clone();
                let res = spawn_blocking(move || gen::handle(rpc.as_ref(), &req))
                    .await
                    .unwrap();
                responses.push(res);
            }
            Json(Response::Batch(responses))
        }
    }
}

async fn proxy_request_ethereum(
    State(state): State<Context>,
    Json(req): Json<Request>,
) -> impl IntoResponse {
    match req {
        Request::Single(req) => {
            println!("proxy: {}", req.method);
            let rpc = state.ethereum.clone();
            let res = spawn_blocking(move || ethereum::gen::handle(rpc.as_ref(), &req))
                .await
                .unwrap();
            Json(Response::Single(res))
        }
        Request::Batch(requests) => {
            let mut responses = Vec::with_capacity(requests.len());
            for req in requests {
                println!("proxy: {}", req.method);
                let rpc = state.ethereum.clone();
                let res = spawn_blocking(move || ethereum::gen::handle(rpc.as_ref(), &req))
                    .await
                    .unwrap();
                responses.push(res);
            }
            Json(Response::Batch(responses))
        }
    }
}

mod starknet {
    include!(concat!(env!("OUT_DIR"), "/rpc.rs"));
}

use starknet::gen;

mod ethereum {
    include!(concat!(env!("OUT_DIR"), "/eth.rs"));
}

impl gen::Rpc for Context {
    fn getBlockWithTxHashes(
        &self,
        _block_id: gen::BlockId,
    ) -> std::result::Result<gen::GetBlockWithTxHashesResult, jsonrpc::Error> {
        Ok(gen::GetBlockWithTxHashesResult::BlockWithTxHashes(
            gen::BlockWithTxHashes {
                status: gen::BlockStatus::Pending,
                block_header: gen::BlockHeader {
                    block_hash: gen::BlockHash(gen::Felt::try_new("0x1")?),
                    timestamp: 1042,
                    sequencer_address: gen::Felt::try_new("0x2")?,
                    block_number: gen::BlockNumber(42),
                    new_root: gen::Felt::try_new("0x3")?,
                    parent_hash: gen::BlockHash(gen::Felt::try_new("0x4")?),
                },
                block_body_with_tx_hashes: gen::BlockBodyWithTxHashes {
                    transactions: vec![gen::Felt::try_new("0x5")?, gen::Felt::try_new("0x6")?],
                },
            },
        ))
    }

    fn getBlockWithTxs(
        &self,
        _block_id: gen::BlockId,
    ) -> std::result::Result<gen::GetBlockWithTxsResult, jsonrpc::Error> {
        Ok(gen::GetBlockWithTxsResult::BlockWithTxs(
            gen::BlockWithTxs {
                status: gen::BlockStatus::AcceptedOnL1,
                block_header: gen::BlockHeader {
                    block_hash: gen::BlockHash(gen::Felt::try_new("0x1")?),
                    timestamp: 1042,
                    sequencer_address: gen::Felt::try_new("0x2")?,
                    block_number: gen::BlockNumber(42),
                    new_root: gen::Felt::try_new("0x3")?,
                    parent_hash: gen::BlockHash(gen::Felt::try_new("0x4")?),
                },
                block_body_with_txs: gen::BlockBodyWithTxs {
                    transactions: vec![
                        gen::Txn::InvokeTxn(gen::InvokeTxn {
                            common_txn_properties: gen::CommonTxnProperties {
                                transaction_hash: gen::TxnHash(gen::Felt::try_new("0x1")?),
                                broadcasted_txn_common_properties:
                                    gen::BroadcastedTxnCommonProperties {
                                        nonce: gen::Felt::try_new("0x1")?,
                                        version: gen::NumAsHex::try_new("0x1")?,
                                        max_fee: gen::Felt::try_new("0x1")?,
                                        signature: gen::Signature(vec![gen::Felt::try_new("0x1")?]),
                                    },
                            },
                            r#type: gen::InvokeTxnType::Invoke,
                            invoke_txn_kind: gen::InvokeTxnKind::FunctionCall(gen::FunctionCall {
                                calldata: vec![gen::Felt::try_new("0x1")?],
                                entry_point_selector: gen::Felt::try_new("0x1")?,
                                contract_address: gen::Address(gen::Felt::try_new("0x1")?),
                            }),
                        }),
                        gen::Txn::InvokeTxn(gen::InvokeTxn {
                            common_txn_properties: gen::CommonTxnProperties {
                                transaction_hash: gen::TxnHash(gen::Felt::try_new("0x2")?),
                                broadcasted_txn_common_properties:
                                    gen::BroadcastedTxnCommonProperties {
                                        nonce: gen::Felt::try_new("0x2")?,
                                        version: gen::NumAsHex::try_new("0x2")?,
                                        max_fee: gen::Felt::try_new("0x2")?,
                                        signature: gen::Signature(vec![gen::Felt::try_new("0x2")?]),
                                    },
                            },
                            r#type: gen::InvokeTxnType::Invoke,
                            invoke_txn_kind: gen::InvokeTxnKind::InvokeTxnV1(gen::InvokeTxnV1 {
                                sender_address: gen::Address(gen::Felt::try_new("0x1")?),
                                calldata: vec![gen::Felt::try_new("0x1")?],
                            }),
                        }),
                    ],
                },
            },
        ))
    }

    fn getStateUpdate(
        &self,
        _block_id: gen::BlockId,
    ) -> std::result::Result<gen::GetStateUpdateResult, jsonrpc::Error> {
        Ok(gen::GetStateUpdateResult::StateUpdate(gen::StateUpdate {
            new_root: gen::Felt::try_new("0xcafebabe")?,
            block_hash: gen::BlockHash(gen::Felt::try_new("0xdeadbeef")?),
            pending_state_update: gen::PendingStateUpdate {
                state_diff: gen::StateDiff {
                    nonces: vec![gen::NoncesItem {
                        nonce: Some(gen::Felt::try_new("0x1")?),
                        contract_address: Some(gen::Address(gen::Felt::try_new("0x2")?)),
                    }],
                    deprecated_declared_classes: vec![gen::Felt::try_new("0x3")?],
                    deployed_contracts: vec![gen::DeployedContractItem {
                        address: gen::Felt::try_new("0x4")?,
                        class_hash: gen::Felt::try_new("0x5")?,
                    }],
                    replaced_classes: vec![gen::ReplacedClassesItem {
                        class_hash: Some(gen::Felt::try_new("0x101")?),
                        contract_address: Some(gen::Address(gen::Felt::try_new("0x102")?)),
                    }],
                    declared_classes: vec![gen::DeclaredClassesItem {
                        compiled_class_hash: Some(gen::Felt::try_new("0x6")?),
                        class_hash: Some(gen::Felt::try_new("0x7")?),
                    }],
                    storage_diffs: vec![gen::ContractStorageDiffItem {
                        address: gen::Felt::try_new("0x8")?,
                        storage_entries: vec![gen::StorageEntriesItem {
                            key: Some(gen::Felt::try_new("0x9")?),
                            value: Some(gen::Felt::try_new("0xA")?),
                        }],
                    }],
                },
                old_root: gen::Felt::try_new("0xFACE")?,
            },
        }))
    }

    fn getStorageAt(
        &self,
        _contract_address: gen::Address,
        _key: gen::StorageKey,
        _block_id: gen::BlockId,
    ) -> std::result::Result<gen::Felt, jsonrpc::Error> {
        Ok(gen::Felt::try_new("0xcafebabe")?)
    }

    fn getTransactionByHash(
        &self,
        _transaction_hash: gen::TxnHash,
    ) -> std::result::Result<gen::Txn, jsonrpc::Error> {
        Ok(gen::Txn::L1HandlerTxn(gen::L1HandlerTxn {
            r#type: gen::L1HandlerTxnType::L1Handler,
            transaction_hash: gen::TxnHash(gen::Felt::try_new("0xA")?),
            version: gen::NumAsHex::try_new("0x0")?,
            nonce: gen::NumAsHex::try_new("0x0")?,
            function_call: gen::FunctionCall {
                calldata: vec![gen::Felt::try_new("0x1")?],
                entry_point_selector: gen::Felt::try_new("0x1")?,
                contract_address: gen::Address(gen::Felt::try_new("0x1")?),
            },
        }))
    }

    fn getTransactionByBlockIdAndIndex(
        &self,
        _block_id: gen::BlockId,
        _index: gen::Index,
    ) -> std::result::Result<gen::Txn, jsonrpc::Error> {
        Ok(gen::Txn::DeclareTxn(gen::DeclareTxn::DeclareTxnV2(
            gen::DeclareTxnV2 {
                declare_txn_v1: gen::DeclareTxnV1 {
                    common_txn_properties: gen::CommonTxnProperties {
                        transaction_hash: gen::TxnHash(gen::Felt::try_new("0x1")?),
                        broadcasted_txn_common_properties: gen::BroadcastedTxnCommonProperties {
                            nonce: gen::Felt::try_new("0x1")?,
                            version: gen::NumAsHex::try_new("0x1")?,
                            max_fee: gen::Felt::try_new("0x1")?,
                            signature: gen::Signature(vec![gen::Felt::try_new("0x1")?]),
                        },
                    },
                    class_hash: gen::Felt::try_new("0x1")?,
                    sender_address: gen::Address(gen::Felt::try_new("0x1")?),
                    r#type: gen::DeclareTxnV1Type::Declare,
                },
                compiled_class_hash: Some(gen::Felt::try_new("0x1")?),
            },
        )))
    }

    fn getTransactionReceipt(
        &self,
        _transaction_hash: gen::TxnHash,
    ) -> std::result::Result<gen::TxnReceipt, jsonrpc::Error> {
        Ok(gen::TxnReceipt::DeployTxnReceipt(gen::DeployTxnReceipt {
            common_receipt_properties: gen::CommonReceiptProperties {
                messages_sent: vec![gen::MsgToL1 {
                    to_address: gen::Felt::try_new("0x1")?,
                    payload: vec![gen::Felt::try_new("0x1")?, gen::Felt::try_new("0x1")?],
                }],
                events: vec![gen::Event {
                    from_address: gen::Address(gen::Felt::try_new("0x1")?),
                    event_content: gen::EventContent {
                        data: vec![gen::Felt::try_new("0x1")?, gen::Felt::try_new("0x1")?],
                        keys: vec![gen::Felt::try_new("0x1")?, gen::Felt::try_new("0x1")?],
                    },
                }],
                transaction_hash: gen::TxnHash(gen::Felt::try_new("0x1")?),
                actual_fee: gen::Felt::try_new("0x1")?,
                status: gen::TxnStatus::AcceptedOnL2,
                block_hash: gen::BlockHash(gen::Felt::try_new("0x1")?),
                block_number: gen::BlockNumber(42),
            },
            contract_address: gen::Felt::try_new("0x1")?,
            r#type: gen::DeployTxnReceiptType::Deploy,
        }))
    }

    fn getClass(
        &self,
        _block_id: gen::BlockId,
        _class_hash: gen::Felt,
    ) -> std::result::Result<gen::GetClassResult, jsonrpc::Error> {
        Ok(gen::GetClassResult::ContractClass(gen::ContractClass {
            entry_points_by_type: gen::ContractClassEntryPoint {
                constructor: Some(vec![gen::SierraEntryPoint {
                    selector: Some(gen::Felt::try_new("0x11")?),
                    function_idx: Some(1),
                }]),
                external: Some(vec![gen::SierraEntryPoint {
                    selector: Some(gen::Felt::try_new("0x22")?),
                    function_idx: Some(2),
                }]),
                l1_handler: Some(vec![gen::SierraEntryPoint {
                    selector: Some(gen::Felt::try_new("0x33")?),
                    function_idx: Some(3),
                }]),
            },
            abi: Some("abi".to_string()),
            sierra_program: vec![gen::Felt::try_new("0xABCD")?],
            contract_class_version: "0".to_string(),
        }))
    }

    fn getClassHashAt(
        &self,
        _block_id: gen::BlockId,
        _contract_address: gen::Address,
    ) -> std::result::Result<gen::Felt, jsonrpc::Error> {
        Ok(gen::Felt::try_new("0xF")?)
    }

    fn getClassAt(
        &self,
        _block_id: gen::BlockId,
        _contract_address: gen::Address,
    ) -> std::result::Result<gen::GetClassAtResult, jsonrpc::Error> {
        Ok(gen::GetClassAtResult::ContractClass(gen::ContractClass {
            entry_points_by_type: gen::ContractClassEntryPoint {
                constructor: Some(vec![gen::SierraEntryPoint {
                    selector: Some(gen::Felt::try_new("0x11")?),
                    function_idx: Some(1),
                }]),
                external: Some(vec![gen::SierraEntryPoint {
                    selector: Some(gen::Felt::try_new("0x22")?),
                    function_idx: Some(2),
                }]),
                l1_handler: Some(vec![gen::SierraEntryPoint {
                    selector: Some(gen::Felt::try_new("0x33")?),
                    function_idx: Some(3),
                }]),
            },
            abi: Some("abi".to_string()),
            sierra_program: vec![gen::Felt::try_new("0x44")?],
            contract_class_version: "0".to_string(),
        }))
    }

    fn getBlockTransactionCount(
        &self,
        _block_id: gen::BlockId,
    ) -> std::result::Result<gen::GetBlockTransactionCountResult, jsonrpc::Error> {
        Ok(gen::GetBlockTransactionCountResult(42))
    }

    fn call(
        &self,
        _request: gen::FunctionCall,
        _block_id: gen::BlockId,
    ) -> std::result::Result<gen::CallResult, jsonrpc::Error> {
        Ok(gen::CallResult(vec![gen::Felt::try_new("0x0")?]))
    }

    fn estimateFee(
        &self,
        _request: gen::Request,
        _block_id: gen::BlockId,
    ) -> std::result::Result<gen::EstimateFeeResult, jsonrpc::Error> {
        Ok(gen::EstimateFeeResult(vec![gen::FeeEstimate {
            gas_consumed: Some(gen::NumAsHex::try_new("0xAA")?),
            gas_price: Some(gen::NumAsHex::try_new("0xBB")?),
            overall_fee: Some(gen::NumAsHex::try_new("0xCC")?),
        }]))
    }

    fn blockNumber(&self) -> std::result::Result<gen::BlockNumber, jsonrpc::Error> {
        Ok(gen::BlockNumber(42))
    }

    fn blockHashAndNumber(
        &self,
    ) -> std::result::Result<gen::BlockHashAndNumberResult, jsonrpc::Error> {
        Ok(gen::BlockHashAndNumberResult {
            block_number: Some(gen::BlockNumber(42)),
            block_hash: Some(gen::BlockHash(gen::Felt::try_new("0xface")?)),
        })
    }

    fn chainId(&self) -> std::result::Result<gen::ChainId, jsonrpc::Error> {
        Ok(gen::ChainId::try_new("0xdeadbeef")?)
    }

    fn pendingTransactions(
        &self,
    ) -> std::result::Result<gen::PendingTransactionsResult, jsonrpc::Error> {
        Ok(gen::PendingTransactionsResult(vec![gen::Txn::DeployTxn(
            gen::DeployTxn {
                transaction_hash: gen::TxnHash(gen::Felt::try_new("0x1")?),
                class_hash: gen::Felt::try_new("0x2")?,
                deploy_txn_properties: gen::DeployTxnProperties {
                    r#type: gen::DeployTxnPropertiesType::Deploy,
                    version: gen::NumAsHex::try_new("0x3")?,
                    contract_address_salt: gen::Felt::try_new("0x4")?,
                    constructor_calldata: vec![gen::Felt::try_new("0x5")?],
                },
            },
        )]))
    }

    fn syncing(&self) -> std::result::Result<gen::SyncingSyncing, jsonrpc::Error> {
        Ok(gen::SyncingSyncing::SyncStatus(gen::SyncStatus {
            starting_block_num: gen::NumAsHex::try_new("0x1")?,
            current_block_hash: gen::BlockHash(gen::Felt::try_new("0x2")?),
            starting_block_hash: gen::BlockHash(gen::Felt::try_new("0x3")?),
            current_block_num: gen::NumAsHex::try_new("0x4")?,
            highest_block_hash: gen::BlockHash(gen::Felt::try_new("0x5")?),
            highest_block_num: gen::NumAsHex::try_new("0x6")?,
        }))
    }

    fn getEvents(
        &self,
        _filter: gen::Filter,
    ) -> std::result::Result<gen::EventsChunk, jsonrpc::Error> {
        Ok(gen::EventsChunk {
            continuation_token: Some("token-0".to_string()),
            events: vec![gen::EmittedEvent {
                event: gen::Event {
                    from_address: gen::Address(gen::Felt::try_new("0x5")?),
                    event_content: gen::EventContent {
                        keys: vec![gen::Felt::try_new("0x4")?],
                        data: vec![gen::Felt::try_new("0x3")?],
                    },
                },
                block_hash: gen::BlockHash(gen::Felt::try_new("0x2")?),
                block_number: gen::BlockNumber(42),
                transaction_hash: gen::TxnHash(gen::Felt::try_new("0x1")?),
            }],
        })
    }

    fn getNonce(
        &self,
        _block_id: gen::BlockId,
        _contract_address: gen::Address,
    ) -> std::result::Result<gen::Felt, jsonrpc::Error> {
        Ok(gen::Felt::try_new("0xA")?)
    }

    fn addInvokeTransaction(
        &self,
        _invoke_transaction: gen::BroadcastedInvokeTxn,
    ) -> std::result::Result<gen::AddInvokeTransactionResult, jsonrpc::Error> {
        Ok(gen::AddInvokeTransactionResult {
            transaction_hash: Some(gen::TxnHash(gen::Felt::try_new("0x1")?)),
        })
    }

    fn addDeclareTransaction(
        &self,
        _declare_transaction: gen::BroadcastedDeclareTxn,
    ) -> std::result::Result<gen::AddDeclareTransactionResult, jsonrpc::Error> {
        Ok(gen::AddDeclareTransactionResult {
            class_hash: Some(gen::Felt::try_new(
                "0x371b5f7c5517d84205365a87f02dcef230efa7b4dd91a9e4ba7e04c5b69d69b",
            )?),
            transaction_hash: Some(gen::TxnHash(gen::Felt::try_new(
                "0x5aa2a3468bfe8942e321dde6fbebc2c0359da311340c7e35b0bb9b089d4d469",
            )?)),
        })
    }

    fn addDeployAccountTransaction(
        &self,
        _deploy_account_transaction: gen::BroadcastedDeployAccountTxn,
    ) -> std::result::Result<gen::AddDeployAccountTransactionResult, jsonrpc::Error> {
        Ok(gen::AddDeployAccountTransactionResult {
            transaction_hash: Some(gen::TxnHash(gen::Felt::try_new("0x1")?)),
            contract_address: Some(gen::Felt::try_new("0x2")?),
        })
    }

    fn traceTransaction(
        &self,
        _transaction_hash: gen::TxnHash,
    ) -> std::result::Result<gen::TransactionTrace, jsonrpc::Error> {
        todo!()
    }

    fn simulateTransaction(
        &self,
        _block_id: gen::BlockId,
        _transaction: gen::Transaction,
        _simulation_flags: gen::SimulationFlags,
    ) -> std::result::Result<gen::SimulateTransactionSimulatedTransactions, jsonrpc::Error> {
        todo!()
    }

    fn traceBlockTransactions(
        &self,
        _block_hash: gen::BlockHash,
    ) -> std::result::Result<gen::TraceBlockTransactionsTraces, jsonrpc::Error> {
        todo!()
    }
}

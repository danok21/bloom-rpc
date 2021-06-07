
use crate::rpc::*;

use jsonrpc_core::{Result, BoxFuture};
use jsonrpc_core::futures::future;

use ethereum_types::{H64, H160, H256, U64, U256};
//extern crate hex_literal;
// use  hex_literal::hex;
use serde::{Serialize, Serializer};
use crate::rpc::types::sync::{SyncStatus, SyncInfo};


pub struct BloomClient {}

impl BloomClient {
    pub fn new() -> Self {
        BloomClient {}
    }
}

impl RpcMethod for BloomClient {
    type Metadata = Metadata;
    fn eth_syncing(&self) -> Result<SyncStatus> {

           let info = SyncInfo {

               /// Current block
               current_block: U256::from(133),
               /// Highest block seen so far
               highest_block: U256::from(134),
               /// Starting block
               starting_block: U256::from(0x432),
           };
           Ok(SyncStatus::Info(info))
    }
    fn accounts(&self) -> Result<Vec<H160>> {
        Ok(Vec::new())
    }

    fn gas_price(&self) -> BoxFuture<U256> {
        let bal = U256::zero();
        Box::new(future::done(Ok(bal)))
    }

    fn block_number(&self) -> Result<U256>{
        let bal = U256::zero();
        Ok(bal)
    }


    fn balance(&self, address: H160, num: Option<BlockNumber>) -> BoxFuture<U256> {
        let bal = U256::zero();
        Box::new(future::done(Ok(bal)))
    }

    fn send_transaction(&self, tx: TransactionRequest) -> BoxFuture<H256> {
        println!("{}",tx);
        Box::new(future::done(Ok(H256::default())))
    }

    fn send_raw_transaction(&self, tx: Bytes) -> Result<H256> {
        println!("send_raw_transaction:{:?}",tx);
        Ok(H256::default())
    }

    fn storage_at(&self, _: H160, _: U256, _: Option<BlockNumber>) -> BoxFuture<H256>{
        Box::new(future::done(Ok(H256::default())))
    }

    fn code_at(&self, _: H160, _: Option<BlockNumber>) -> BoxFuture<Bytes>{
        Box::new(future::done(Ok(Bytes::default())))
    }

    fn call(&self, _: CallRequest, _: Option<BlockNumber>) -> BoxFuture<Bytes>{
        Box::new(future::done(Ok(Bytes::default())))
    }

}
use fresh_credit_primitives::Balance;
use fresh_credit_runtime::{opaque::Block, AccountId, Index};
use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use std::sync::Arc;

pub struct FullDeps<C, P> {
  pub client: Arc<C>,
  pub deny_unsafe: DenyUnsafe,
  pub pool: Arc<P>,
}

pub fn create_full<C, P>(deps: FullDeps<C, P>) -> jsonrpc_core::IoHandler<sc_rpc::Metadata>
where
  C: ProvideRuntimeApi<Block>,
  C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
  C: Send + Sync + 'static,
  C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
  C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
  C::Api: BlockBuilder<Block>,
  P: TransactionPool + 'static,
{
  use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
  use substrate_frame_rpc_system::{FullSystem, SystemApi};

  let mut io = jsonrpc_core::IoHandler::default();
  let FullDeps { client, pool, deny_unsafe } = deps;

  io.extend_with(SystemApi::to_delegate(FullSystem::new(client.clone(), pool, deny_unsafe)));
  io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(client)));

  io
}

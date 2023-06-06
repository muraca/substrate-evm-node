// Substrate
use sc_executor::{NativeElseWasmExecutor, NativeExecutionDispatch, NativeVersion};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_runtime::traits::BlakeTwo256;
// Local
use substrate_evm_runtime::{opaque::Block, AccountId, Balance, Index};

use crate::eth::EthCompatRuntimeApiCollection;

/// Full backend.
pub type FullBackend = sc_service::TFullBackend<Block>;
/// Full client.
pub type FullClient<RuntimeApi, Executor> =
    sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>;

pub type Client = FullClient<substrate_evm_runtime::RuntimeApi, SubstrateEVMRuntimeExecutor>;

/// Only enable the benchmarking host functions when we actually want to benchmark.
#[cfg(feature = "runtime-benchmarks")]
pub type HostFunctions = frame_benchmarking::benchmarking::HostFunctions;
/// Otherwise we use empty host functions for ext host functions.
#[cfg(not(feature = "runtime-benchmarks"))]
pub type HostFunctions = ();

pub struct SubstrateEVMRuntimeExecutor;
impl NativeExecutionDispatch for SubstrateEVMRuntimeExecutor {
    type ExtendHostFunctions = HostFunctions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        substrate_evm_runtime::api::dispatch(method, data)
    }

    fn native_version() -> NativeVersion {
        substrate_evm_runtime::native_version()
    }
}

/// A set of APIs that every runtimes must implement.
pub trait BaseRuntimeApiCollection:
    sp_api::ApiExt<Block>
    + sp_api::Metadata<Block>
    + sp_block_builder::BlockBuilder<Block>
    + sp_offchain::OffchainWorkerApi<Block>
    + sp_session::SessionKeys<Block>
    + sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
where
    <Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

impl<Api> BaseRuntimeApiCollection for Api
where
    Api: sp_api::ApiExt<Block>
        + sp_api::Metadata<Block>
        + sp_block_builder::BlockBuilder<Block>
        + sp_offchain::OffchainWorkerApi<Block>
        + sp_session::SessionKeys<Block>
        + sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>,
    <Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

/// A set of APIs that Substrate EVM runtime must implement.
pub trait RuntimeApiCollection:
    BaseRuntimeApiCollection
    + EthCompatRuntimeApiCollection
    + sp_consensus_aura::AuraApi<Block, AuraId>
    + sp_finality_grandpa::GrandpaApi<Block>
    + frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index>
    + pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
where
    <Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

impl<Api> RuntimeApiCollection for Api
where
    Api: BaseRuntimeApiCollection
        + EthCompatRuntimeApiCollection
        + sp_consensus_aura::AuraApi<Block, AuraId>
        + sp_finality_grandpa::GrandpaApi<Block>
        + frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index>
        + pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>,
    <Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

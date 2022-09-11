use crate::{Aura, BlockNumber, Grandpa};
use alloc::vec::Vec;
use sp_runtime::{generic, impl_opaque_keys, traits::BlakeTwo256};

pub type Block = generic::Block<Header, sp_runtime::OpaqueExtrinsic>;
pub type BlockId = generic::BlockId<Block>;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

impl_opaque_keys! {
  pub struct SessionKeys {
    pub aura: Aura,
    pub grandpa: Grandpa,
  }
}

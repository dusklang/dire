mod hir;
mod ty;
mod arch;
mod index_counter;
mod source_info;

use index_vec::{IndexVec, define_index_type};

use hir::{HirCode, ItemId};

define_index_type!(pub struct OpId = u32;);
define_index_type!(pub struct BlockId = u32;);

pub enum Op {
    HirItem(ItemId),
}

pub struct Block {
    ops: IndexVec<OpId, Op>,
}

pub struct Code {
    blocks: IndexVec<BlockId, Block>,
    hir_code: HirCode,
}
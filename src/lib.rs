pub mod hir;
pub mod ty;
pub mod arch;
pub mod index_counter;
pub mod source_info;
pub mod mir;

use index_vec::{IndexVec, define_index_type};

use hir::{HirCode, ItemId};
use mir::{MirCode, FuncId, InstrId};

define_index_type!(pub struct OpId = u32;);
define_index_type!(pub struct BlockId = u32;);

#[derive(Copy, Clone)]
pub enum Op {
    HirItem(ItemId),
    MirFunc(FuncId),
    MirInstr(InstrId),
}

impl Op {
    pub fn as_mir_instr(self) -> Option<InstrId> {
        match self {
            Op::MirInstr(instr) => Some(instr),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct Block {
    pub ops: IndexVec<OpId, Op>,
}

#[derive(Default)]
pub struct Code {
    pub blocks: IndexVec<BlockId, Block>,
    pub hir_code: HirCode,
    pub mir_code: MirCode,
}
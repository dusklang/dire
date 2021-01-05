pub mod hir;
pub mod ty;
pub mod arch;
pub mod index_counter;
pub mod source_info;
pub mod mir;

use index_vec::{IndexVec, define_index_type};
use display_adapter::display_adapter;

use hir::{HirCode, Item};
use mir::{MirCode, InstrId};

define_index_type!(pub struct OpId = u32;);
define_index_type!(pub struct BlockId = u32;);

#[derive(Copy, Clone, Debug)]
pub enum Op {
    HirItem(Item),
    MirInstr(InstrId),
}

impl Op {
    pub fn as_mir_instr(self) -> Option<InstrId> {
        match self {
            Op::MirInstr(instr) => Some(instr),
            _ => None,
        }
    }

    pub fn as_hir_item(self) -> Option<Item> {
        match self {
            Op::HirItem(item) => Some(item),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct Block {
    pub ops: Vec<OpId>,
}

#[derive(Default)]
pub struct Code {
    pub blocks: IndexVec<BlockId, Block>,
    pub ops: IndexVec<OpId, Op>,
    pub hir_code: HirCode,
    pub mir_code: MirCode,
}

impl Code {
    #[display_adapter]
    pub fn display_block(&self, block: BlockId, w: &mut Formatter) {
        let block = &self.blocks[block];
        for &id in &block.ops {
            write!(w, "    %op{}", id.index())?;
            let op = self.ops[id];
            match op {
                Op::MirInstr(instr) => {
                    write!(w, "(%instr{}) = mir.", instr.index())?;
                    let instr = &self.mir_code.instrs[instr];
                    writeln!(w, "{:?}", instr)?;
                },
                Op::HirItem(item) => {
                    match item {
                        Item::Expr(expr) => {
                            write!(w, "(%expr{}) = hir.", expr.index())?;
                            let expr = &self.hir_code.exprs[expr];
                            writeln!(w, "{:?}", expr)?;
                        },
                        Item::Decl(decl) => {
                            write!(w, "(%decl{}) = hir.", decl.index())?;
                            let decl = &self.hir_code.decls[decl];
                            writeln!(w, "{:?}", decl)?;
                        }
                    }
                },
            }
        }
        Ok(())
    }
}
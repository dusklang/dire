pub mod hir;
pub mod ty;
pub mod arch;
pub mod index_counter;
pub mod source_info;
pub mod dil;

use index_vec::{IndexVec, index_vec, define_index_type};
use display_adapter::display_adapter;

use hir::{HirCode, Item};
use dil::{DilCode, Instr, VOID_INSTR};
use source_info::SourceRange;

define_index_type!(pub struct OpId = u32;);
define_index_type!(pub struct BlockId = u32;);

#[derive(Clone, Debug)]
pub enum Op {
    HirItem(Item),
    DilInstr(Instr),
}

impl Op {
    pub fn as_dil_instr(&self) -> Option<&Instr> {
        match self {
            Op::DilInstr(instr) => Some(instr),
            _ => None,
        }
    }

    pub fn as_hir_item(&self) -> Option<Item> {
        match self {
            &Op::HirItem(item) => Some(item),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct Block {
    pub ops: Vec<OpId>,
}
pub struct Code {
    pub blocks: IndexVec<BlockId, Block>,
    pub ops: IndexVec<OpId, Op>,
    pub hir_code: HirCode,
    pub dil_code: DilCode,
}

impl Default for Code {
    fn default() -> Self {
        let mut val = Code {
            blocks: IndexVec::default(),
            ops: index_vec![Op::DilInstr(Instr::Void)],
            hir_code: HirCode::default(),
            dil_code: DilCode::default(),
        };
        val.dil_code.source_ranges.insert(VOID_INSTR, SourceRange::default());
        val.dil_code.instr_names.insert(VOID_INSTR, "void".to_string());
        val
    }
}

impl Code {
    #[display_adapter]
    pub fn display_block(&self, block: BlockId, w: &mut Formatter) {
        let block = &self.blocks[block];
        for &id in &block.ops {
            write!(w, "    %op{}", id.index())?;
            match self.ops[id] {
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
                Op::DilInstr(ref instr) => {
                    writeln!(w, " = dil.{:?}", instr)?;
                },
            }
        }
        Ok(())
    }
}
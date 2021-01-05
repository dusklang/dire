use std::collections::HashMap;
use std::ffi::CString;

use index_vec::{IndexVec, define_index_type, index_vec};
use smallvec::SmallVec;
use string_interner::DefaultSymbol as Sym;
use display_adapter::display_adapter;

use crate::hir::{Intrinsic, StructId, ModScopeId};
use crate::ty::Type;
use crate::{Code, BlockId, Block};

define_index_type!(pub struct FuncId = u32;);
define_index_type!(pub struct InstrId = u32;);
define_index_type!(pub struct StaticId = u32;);
define_index_type!(pub struct StrId = u32;);

#[derive(Debug, PartialEq)]
pub enum Instr {
    Void,
    Const(Const),
    Alloca(Type),
    LogicalNot(InstrId),
    Call { arguments: SmallVec<[InstrId; 2]>, func: FuncId },
    Intrinsic { arguments: SmallVec<[InstrId; 2]>, ty: Type, intr: Intrinsic },
    Reinterpret(InstrId, Type),
    Truncate(InstrId, Type),
    SignExtend(InstrId, Type),
    ZeroExtend(InstrId, Type),
    FloatCast(InstrId, Type),
    FloatToInt(InstrId, Type),
    IntToFloat(InstrId, Type),
    Load(InstrId),
    Store { location: InstrId, value: InstrId },
    AddressOfStatic(StaticId),
    Pointer { op: InstrId, is_mut: bool },
    Struct { fields: SmallVec<[InstrId; 2]>, id: StructId },
    StructLit { fields: SmallVec<[InstrId; 2]>, id: StructId },
    DirectFieldAccess { val: InstrId, index: usize },
    IndirectFieldAccess { val: InstrId, index: usize },
    Ret(InstrId),
    Br(BlockId),
    CondBr { condition: InstrId, true_bb: BlockId, false_bb: BlockId },
    /// Only valid at the beginning of a function, right after the void instruction
    Parameter(Type),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Const {
    Int { lit: u64, ty: Type },
    Float { lit: f64, ty: Type },
    Str { id: StrId, ty: Type },
    Bool(bool),
    Ty(Type),
    Mod(ModScopeId),
    StructLit { fields: Vec<Const>, id: StructId },
}

impl Const {
    pub fn ty(&self) -> Type {
        match self {
            Const::Int { ty, .. } => ty.clone(),
            Const::Float { ty, .. } => ty.clone(),
            Const::Str { ty, .. } => ty.clone(),
            Const::Bool(_) => Type::Bool,
            Const::Ty(_) => Type::Ty,
            Const::Mod(_) => Type::Mod,
            &Const::StructLit { id, .. } => Type::Struct(id),
        }
    }
}

#[derive(Debug, Default)]
pub struct Function {
    pub name: Option<Sym>,
    pub ret_ty: Type,
    /// Index 0 is defined to be the entry block
    pub blocks: Vec<BlockId>,
}

mod private {
    pub trait Sealed {}
}

pub trait GetBlock<'a>: private::Sealed {
    fn get_block(self, code: &'a Code) -> &'a Block;
}

impl private::Sealed for BlockId {}
impl<'a> GetBlock<'a> for BlockId {
    fn get_block(self, code: &'a Code) -> &'a Block {
        &code.blocks[self]
    }
}

impl private::Sealed for &Block {}
impl<'a> GetBlock<'a> for &'a Block {
    fn get_block(self, _code: &'a Code) -> &'a Block {
        self
    }
}

impl Code {
    pub fn get_mir_instr<'a>(&'a self, block: impl GetBlock<'a>, op: usize) -> Option<&'a Instr> {
        let block = block.get_block(self);
        let op = block.ops[op];
        self.ops[op].as_mir_instr().map(|instr| &self.mir_code.instrs[instr])
    }

    pub fn num_parameters(&self, func: &Function) -> usize {
        let entry = func.blocks[0];
        let block = &self.blocks[entry];
        let mut num_parameters = 0;
        for i in 0..block.ops.len() {
            match self.get_mir_instr(block, i).unwrap() {
                Instr::Parameter(_) => num_parameters += 1,
                _ => break,
            }
        }
        num_parameters
    }

    #[display_adapter]
    pub fn display_func(&self, func: &Function, name: &str, w: &mut Formatter) {
        writeln!(w, "fn {}() {{", name)?;
        for &block in &func.blocks {
            write!(w, "%bb{}:\n{}", block.index(), self.display_block(block))?;
        }
        writeln!(w, "}}")?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct Struct {
    pub field_tys: SmallVec<[Type; 2]>,
    pub layout: StructLayout,
}

#[derive(Clone)]
pub struct StructLayout {
    pub field_offsets: SmallVec<[usize; 2]>,
    pub alignment: usize,
    pub size: usize,
    pub stride: usize,
}

#[derive(Debug)]
pub enum BlockState {
    Created,
    Started,
    Ended,
}

pub struct MirCode {
    pub strings: IndexVec<StrId, CString>,
    pub functions: IndexVec<FuncId, Function>,
    pub statics: IndexVec<StaticId, Const>,
    pub structs: HashMap<StructId, Struct>,
    pub instrs: IndexVec<InstrId, Instr>,
    block_states: HashMap<BlockId, BlockState>,
}

#[derive(Debug)]
pub enum StartBlockError {
    BlockEnded,
}

#[derive(Debug)]
pub enum EndBlockError {
    BlockEnded,
    BlockNotStarted,
}

impl MirCode {
    pub fn new() -> Self {
        MirCode {
            strings: IndexVec::new(),
            functions: IndexVec::new(),
            statics: IndexVec::new(),
            structs: HashMap::new(),
            instrs: index_vec![Instr::Void],
            block_states: HashMap::new(),
        }
    }

    fn get_block_state(&mut self, block: BlockId) -> &mut BlockState {
        self.block_states.entry(block).or_insert(BlockState::Created)
    }

    pub fn start_block(&mut self, block: BlockId) -> Result<(), StartBlockError> {
        let state = self.get_block_state(block);
        match state {
            BlockState::Created => {
                *state = BlockState::Started;
                Ok(())
            }
            BlockState::Started => Ok(()),
            BlockState::Ended => Err(StartBlockError::BlockEnded),
        }
    }

    pub fn end_block(&mut self, block: BlockId) -> Result<(), EndBlockError> {
        let state = self.get_block_state(block);
        match state {
            BlockState::Created => Err(EndBlockError::BlockNotStarted),
            BlockState::Started => {
                *state = BlockState::Ended;
                Ok(())
            },
            BlockState::Ended => Err(EndBlockError::BlockEnded),
        }
    }

    pub fn first_unended_block(&self, func: &Function) -> Option<BlockId> {
        func.blocks.iter().find(|&block| {
            let state = &self.block_states[block];
            !matches!(state, BlockState::Ended)
        }).copied()
    }

    pub fn check_all_blocks_ended(&self, func: &Function) {
        if let Some(block) = self.first_unended_block(func) {
            panic!("MIR: Block {} was not ended", block.index());
        }
    }
}

impl Default for MirCode {
    fn default() -> Self { Self::new() }
}
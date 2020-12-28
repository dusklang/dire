use std::collections::HashMap;
use std::ffi::CString;

use index_vec::{IndexVec, define_index_type};
use smallvec::SmallVec;
use string_interner::DefaultSymbol as Sym;

use crate::hir::{Intrinsic, StructId, ModScopeId};
use crate::ty::Type;
use crate::BlockId;

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

#[derive(Debug)]
pub struct Function {
    pub name: Option<Sym>,
    pub ret_ty: Type,
    pub entry: BlockId,
}

impl Function {
    // TODO: Implement this
    // pub fn num_parameters(&self) -> usize {
    //     let raw_code = &self.code.raw;
    //     assert_eq!(raw_code[0], Instr::Void);
    //     let mut num_parameters = 0;
    //     for i in 1..raw_code.len() {
    //         match &raw_code[i] {
    //             Instr::Parameter(_) => num_parameters += 1,
    //             _ => break,
    //         }
    //     }
    //     num_parameters
    // }
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

#[derive(Default)]
pub struct MirCode {
    pub strings: IndexVec<StrId, CString>,
    pub functions: IndexVec<FuncId, Function>,
    pub statics: IndexVec<StaticId, Const>,
    pub structs: HashMap<StructId, Struct>,
}
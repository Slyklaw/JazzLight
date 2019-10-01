#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Op {
    ConstNull,
    ConstInt(i64),
    ConstFalse,
    ConstTrue,
    LoadStatic,
    LoadGlobal(u32),
    LoadField,
    LoadIndex(i32),
    LoadLocal(u32),
    LoadEnv(u32),
    StoreEnv(u32),
    StoreStatic,
    StoreLocal(u32),
    StoreField,

    Branch(u32),
    BranchIfTrue(u32),
    BranchIfFalse(u32),

    /// Invoke function.
    Invoke(u32),
    /// Invoke object method.
    InvokeVirtual(u32),
    /// Tail call.
    TailRec(u32),
    MakeEnv(u32),
    MakeArray(u32),
    Return,
    Pop(u32),

    Throw,
    CatchIp(u32),
    CmpStrictEq,
    CmpStrictNeq,
    CmpEq,
    CmpNeq,
    CmpGt,
    CmpGe,
    CmpLt,
    CmpLe,

    Not,
    Neg,

    Add,
    Sub,
    Div,
    Mul,
    Rem,
    Shr,
    Shl,
    UShr,
    BitOr,
    BitAnd,
    BitXor,
    New,
    Ctor(u32),
    OpCount,
}

impl From<Op> for OpRaw {
    fn from(op: Op) -> Self {
        use OpRaw::*;
        match op {
            Op::ConstNull => ConstNull,
            Op::ConstInt(_) => ConstInt,
            Op::ConstFalse => ConstFalse,
            Op::ConstTrue => ConstTrue,
            Op::LoadStatic => LoadStatic,
            Op::LoadGlobal(_) => LoadGlobal,
            Op::LoadField => LoadField,
            Op::LoadIndex(_) => LoadIndex,
            Op::LoadLocal(_) => LoadLocal,
            Op::LoadEnv(_) => LoadEnv,
            Op::StoreEnv(_) => StoreEnv,
            Op::StoreStatic => StoreStatic,
            Op::StoreLocal(_) => StoreLocal,
            Op::StoreField => StoreField,
            Op::Ctor(_) => Ctor,
            Op::Branch(_) => Branch,
            Op::BranchIfTrue(_) => BranchIfTrue,
            Op::BranchIfFalse(_) => BranchIfFalse,

            Op::Invoke(_) => Invoke,
            Op::InvokeVirtual(_) => InvokeVirtual,
            Op::TailRec(_) => TailRec,
            Op::MakeArray(_) => MakeArray,
            Op::MakeEnv(_) => MakeEnv,
            Op::Return => Return,
            Op::Pop(_) => Pop,

            Op::Throw => Throw,
            Op::CatchIp(_) => CatchIp,
            Op::CmpStrictEq => CmpStrictEq,
            Op::CmpStrictNeq => CmpStrictNeq,
            Op::CmpEq => CmpEq,
            Op::CmpNeq => CmpNeq,
            Op::CmpGt => CmpGt,
            Op::CmpGe => CmpGe,
            Op::CmpLt => CmpLt,
            Op::CmpLe => CmpLe,

            Op::Not => Not,
            Op::Neg => Neg,

            Op::Add => Add,
            Op::Sub => Sub,
            Op::Div => Div,
            Op::Mul => Mul,
            Op::Rem => Rem,
            Op::Shr => Shr,
            Op::Shl => Shl,
            Op::UShr => UShr,
            Op::BitOr => BitOr,
            Op::BitAnd => BitAnd,
            Op::BitXor => BitXor,
            Op::New => New,
            Op::OpCount => OpCount,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OpRaw {
    ConstNull,
    ConstInt,
    ConstFalse,
    ConstTrue,
    LoadStatic,
    LoadGlobal,
    LoadField,
    LoadIndex,
    LoadLocal,
    LoadEnv,
    StoreEnv,
    StoreStatic,
    StoreLocal,
    StoreField,

    Branch,
    BranchIfTrue,
    BranchIfFalse,

    /// Invoke function.
    Invoke,
    /// Invoke object method.
    InvokeVirtual,
    /// Tail call.
    TailRec,
    MakeEnv,
    MakeArray,

    Return,
    Pop,

    Throw,
    CatchIp,
    CmpStrictEq,
    CmpStrictNeq,
    CmpEq,
    CmpNeq,
    CmpGt,
    CmpGe,
    CmpLt,
    CmpLe,

    Not,
    Neg,

    Add,
    Sub,
    Div,
    Mul,
    Rem,
    Shr,
    Shl,
    UShr,
    BitOr,
    BitAnd,
    BitXor,
    New,
    Ctor,
    OpCount,
}

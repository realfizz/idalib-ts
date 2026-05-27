use napi_derive::napi;
use idalib::insn as ida_insn;

#[napi]
pub enum OperandType {
    Reg,
    Mem,
    Phrase,
    Displ,
    Imm,
    Far,
    Near,
    IdpSpec0,
    IdpSpec1,
    IdpSpec2,
    IdpSpec3,
    IdpSpec4,
    IdpSpec5,
}

#[napi]
pub enum OperandDataType {
    Byte,
    Word,
    DWord,
    Float,
    Double,
    TByte,
    PackReal,
    QWord,
    Byte16,
    Code,
    Void,
    FWord,
    Bitfield,
    String,
    Unicode,
    LongDouble,
    Byte32,
    Byte64,
    Half,
}

#[napi]
pub struct Instruction {
    pub(crate) inner: ida_insn::Insn,
}

#[napi]
impl Instruction {
    #[napi(getter)]
    pub fn address(&self) -> u64 {
        self.inner.address()
    }

    #[napi(getter, js_name = "type")]
    pub fn itype(&self) -> u32 {
        self.inner.itype() as u32
    }

    #[napi(getter)]
    pub fn len(&self) -> u32 {
        self.inner.len() as u32
    }

    #[napi(getter)]
    pub fn operand_count(&self) -> u32 {
        self.inner.operand_count() as u32
    }

    #[napi(getter)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[napi(getter)]
    pub fn is_call(&self) -> bool {
        self.inner.is_call()
    }

    #[napi(getter)]
    pub fn is_return(&self) -> bool {
        self.inner.is_ret()
    }

    #[napi(getter)]
    pub fn is_indirect_jump(&self) -> bool {
        self.inner.is_indirect_jump()
    }

    #[napi]
    pub fn is_basic_block_end(&self, include_unknown: Option<bool>) -> bool {
        self.inner.is_basic_block_end(include_unknown.unwrap_or(true))
    }

    #[napi]
    pub fn operand(&self, n: u32) -> Option<Operand> {
        self.inner.operand(n as usize).map(|op| Operand { inner: op })
    }
}

#[napi]
pub struct Operand {
    inner: ida_insn::Operand,
}

#[napi]
impl Operand {
    #[napi(getter, js_name = "type")]
    pub fn operand_type(&self) -> OperandType {
        match self.inner.type_() {
            ida_insn::OperandType::Reg => OperandType::Reg,
            ida_insn::OperandType::Mem => OperandType::Mem,
            ida_insn::OperandType::Phrase => OperandType::Phrase,
            ida_insn::OperandType::Displ => OperandType::Displ,
            ida_insn::OperandType::Imm => OperandType::Imm,
            ida_insn::OperandType::Far => OperandType::Far,
            ida_insn::OperandType::Near => OperandType::Near,
            ida_insn::OperandType::IdpSpec0 => OperandType::IdpSpec0,
            ida_insn::OperandType::IdpSpec1 => OperandType::IdpSpec1,
            ida_insn::OperandType::IdpSpec2 => OperandType::IdpSpec2,
            ida_insn::OperandType::IdpSpec3 => OperandType::IdpSpec3,
            ida_insn::OperandType::IdpSpec4 => OperandType::IdpSpec4,
            ida_insn::OperandType::IdpSpec5 => OperandType::IdpSpec5,
        }
    }

    #[napi(getter)]
    pub fn data_type(&self) -> OperandDataType {
        match self.inner.dtype() {
            ida_insn::OperandDataType::Byte => OperandDataType::Byte,
            ida_insn::OperandDataType::Word => OperandDataType::Word,
            ida_insn::OperandDataType::DWord => OperandDataType::DWord,
            ida_insn::OperandDataType::Float => OperandDataType::Float,
            ida_insn::OperandDataType::Double => OperandDataType::Double,
            ida_insn::OperandDataType::TByte => OperandDataType::TByte,
            ida_insn::OperandDataType::PackReal => OperandDataType::PackReal,
            ida_insn::OperandDataType::QWord => OperandDataType::QWord,
            ida_insn::OperandDataType::Byte16 => OperandDataType::Byte16,
            ida_insn::OperandDataType::Code => OperandDataType::Code,
            ida_insn::OperandDataType::Void => OperandDataType::Void,
            ida_insn::OperandDataType::FWord => OperandDataType::FWord,
            ida_insn::OperandDataType::Bitfield => OperandDataType::Bitfield,
            ida_insn::OperandDataType::String => OperandDataType::String,
            ida_insn::OperandDataType::Unicode => OperandDataType::Unicode,
            ida_insn::OperandDataType::LongDouble => OperandDataType::LongDouble,
            ida_insn::OperandDataType::Byte32 => OperandDataType::Byte32,
            ida_insn::OperandDataType::Byte64 => OperandDataType::Byte64,
            ida_insn::OperandDataType::Half => OperandDataType::Half,
        }
    }

    #[napi(getter)]
    pub fn value(&self) -> Option<u64> {
        self.inner.value()
    }

    #[napi(getter)]
    pub fn address(&self) -> Option<u64> {
        self.inner.address()
    }

    #[napi(getter)]
    pub fn register(&self) -> Option<u32> {
        self.inner.register().and_then(|r| u32::try_from(r).ok())
    }

    #[napi(getter)]
    pub fn phrase(&self) -> Option<u32> {
        self.inner.phrase().and_then(|p| u32::try_from(p).ok())
    }
}

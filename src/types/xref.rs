use napi_derive::napi;

#[napi]
pub struct JsXRef {
    pub(crate) from: i64,
    pub(crate) to: i64,
    pub(crate) is_code: bool,
    pub(crate) type_: String,
    pub(crate) flags: u8,
}

#[napi]
impl JsXRef {
    pub(crate) fn new(xref: &idalib::xref::XRef) -> Self {
        Self {
            from: xref.from() as i64,
            to: xref.to() as i64,
            is_code: xref.is_code(),
            type_: format_type(&xref.type_()),
            flags: xref.flags().bits(),
        }
    }

    #[napi(getter)]
    pub fn get_from(&self) -> u64 {
        self.from as u64
    }

    #[napi(getter)]
    pub fn get_to(&self) -> u64 {
        self.to as u64
    }

    #[napi(getter)]
    pub fn get_is_code(&self) -> bool {
        self.is_code
    }

    #[napi(getter, js_name = "type")]
    pub fn get_type(&self) -> &str {
        &self.type_
    }

    #[napi(getter)]
    pub fn get_is_data(&self) -> bool {
        !self.is_code
    }

    #[napi(getter)]
    pub fn get_flags(&self) -> u8 {
        self.flags
    }
}

fn format_type(type_: &idalib::xref::XRefType) -> String {
    match type_ {
        idalib::xref::XRefType::Code(code) => match code {
            idalib::xref::CodeRef::Unknown => "unknown",
            idalib::xref::CodeRef::FarCall => "far_call",
            idalib::xref::CodeRef::NearCall => "near_call",
            idalib::xref::CodeRef::FarJump => "far_jump",
            idalib::xref::CodeRef::NearJump => "near_jump",
            idalib::xref::CodeRef::Obsolete => "obsolete",
            idalib::xref::CodeRef::Flow => "flow",
        },
        idalib::xref::XRefType::Data(data) => match data {
            idalib::xref::DataRef::Unknown => "unknown",
            idalib::xref::DataRef::Offset => "offset",
            idalib::xref::DataRef::Write => "write",
            idalib::xref::DataRef::Read => "read",
            idalib::xref::DataRef::Text => "text",
            idalib::xref::DataRef::Informational => "informational",
            idalib::xref::DataRef::EnumMember => "enum_member",
        },
    }
    .to_string()
}

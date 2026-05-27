use napi::bindgen_prelude::BigInt;
use napi_derive::napi;

#[napi]
#[derive(Clone)]
pub struct JsBasicBlock {
    pub(crate) start_ea: i64,
    pub(crate) end_ea: i64,
}

#[napi]
impl JsBasicBlock {
    #[napi(getter)]
    pub fn get_start_ea(&self) -> u64 {
        self.start_ea as u64
    }

    #[napi(getter)]
    pub fn get_end_ea(&self) -> u64 {
        self.end_ea as u64
    }
}

#[napi]
pub struct JsFunctionCfg {
    basic_blocks_inner: Vec<JsBasicBlock>,
}

#[napi]
impl JsFunctionCfg {
    #[napi(getter)]
    pub fn basic_blocks(&self) -> Vec<JsBasicBlock> {
        self.basic_blocks_inner.clone()
    }
}

impl JsFunctionCfg {
    pub(crate) fn from_cfg(cfg: &idalib::func::FunctionCFG<'_>) -> Self {
        let basic_blocks = cfg
            .blocks()
            .map(|block| JsBasicBlock {
                start_ea: block.start_address() as i64,
                end_ea: block.end_address() as i64,
            })
            .collect();
        JsFunctionCfg {
            basic_blocks_inner: basic_blocks,
        }
    }
}

#[napi]
pub struct JsFunction {
    pub(crate) start_ea: i64,
    pub(crate) end_ea: i64,
    pub(crate) name: Option<String>,
    pub(crate) comment: Option<String>,
    pub(crate) flags: i64,
    pub(crate) is_far: bool,
    pub(crate) does_return: bool,
    pub(crate) analyzed_sp: bool,
    pub(crate) len: u32,
    pub(crate) is_empty: bool,
}

#[napi]
impl JsFunction {
    #[napi(getter)]
    pub fn get_start_ea(&self) -> u64 {
        self.start_ea as u64
    }

    #[napi(getter)]
    pub fn get_end_ea(&self) -> u64 {
        self.end_ea as u64
    }

    #[napi(getter)]
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    #[napi(getter)]
    pub fn get_comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }

    #[napi(getter)]
    pub fn get_flags(&self) -> u64 {
        self.flags as u64
    }

    #[napi(getter)]
    pub fn get_is_far(&self) -> bool {
        self.is_far
    }

    #[napi(getter)]
    pub fn get_does_return(&self) -> bool {
        self.does_return
    }

    #[napi(getter)]
    pub fn get_analyzed_sp(&self) -> bool {
        self.analyzed_sp
    }

    #[napi(getter)]
    pub fn get_len(&self) -> u32 {
        self.len
    }

    #[napi(getter)]
    pub fn get_is_empty(&self) -> bool {
        self.is_empty
    }

    #[napi]
    pub fn contains_address(&self, ea: BigInt) -> napi::Result<bool> {
        let (sign, ea, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        Ok(ea >= self.start_ea as u64 && ea < self.end_ea as u64)
    }
}

impl JsFunction {
    pub(crate) fn from_func(func: &idalib::func::Function<'_>) -> Self {
        JsFunction {
            start_ea: func.start_address() as i64,
            end_ea: func.end_address() as i64,
            name: func.name(),
            comment: func.get_cmt(),
            flags: func.flags().bits() as i64,
            is_far: func.is_far(),
            does_return: func.does_return(),
            analyzed_sp: func.analyzed_sp(),
            len: func.len() as u32,
            is_empty: func.is_empty(),
        }
    }
}

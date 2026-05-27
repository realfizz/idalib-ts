use napi_derive::napi;

#[napi]
pub struct JsStringEntry {
    pub(crate) index: u32,
    pub(crate) address: u64,
    pub(crate) value: String,
    pub(crate) length: u32,
    pub(crate) str_type: u32,
}

#[napi]
impl JsStringEntry {
    #[napi(getter)]
    pub fn get_index(&self) -> u32 {
        self.index
    }

    #[napi(getter)]
    pub fn get_address(&self) -> u64 {
        self.address
    }

    #[napi(getter)]
    pub fn get_value(&self) -> &str {
        &self.value
    }

    #[napi(getter)]
    pub fn get_length(&self) -> u32 {
        self.length
    }

    #[napi(getter, js_name = "type")]
    pub fn get_str_type(&self) -> u32 {
        self.str_type
    }
}

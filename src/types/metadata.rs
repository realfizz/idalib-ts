use napi_derive::napi;

#[napi(object)]
pub struct JsMetadata {
    pub abi: String,
    pub abi_features: String,
    pub analysis_flags: u32,
    pub compiler: String,
    pub file_type: String,
    pub image_base: i64,
}

#[napi(object)]
pub struct JsEntryPoint {
    pub ordinal: u32,
    pub address: i64,
}

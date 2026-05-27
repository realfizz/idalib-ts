use napi_derive::napi;

#[napi(object)]
pub struct JsBookmark {
    pub address: i64,
    pub description: String,
    pub is_enabled: bool,
}

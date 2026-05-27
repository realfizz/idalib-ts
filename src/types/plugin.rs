use napi_derive::napi;

#[napi]
pub struct JsPlugin {
    pub(crate) name: String,
}

#[napi]
impl JsPlugin {
    #[napi(getter)]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

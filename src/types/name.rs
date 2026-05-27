use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
pub struct JsNameProperties {
    pub is_public: bool,
    pub is_weak: bool,
}

#[napi]
pub struct JsNameEntry {
    pub(crate) index: u32,
    pub(crate) address: u64,
    pub(crate) name: String,
    pub(crate) properties: JsNameProperties,
}

#[napi]
impl JsNameEntry {
    #[napi(getter)]
    pub fn get_index(&self) -> u32 {
        self.index
    }

    #[napi(getter)]
    pub fn get_address(&self) -> u64 {
        self.address
    }

    #[napi(getter)]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[napi(getter)]
    pub fn get_properties(&self) -> JsNameProperties {
        self.properties.clone()
    }
}

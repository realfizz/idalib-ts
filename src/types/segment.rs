use napi_derive::napi;

#[napi(object)]
pub struct JsSegmentPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl From<idalib::segment::SegmentPermissions> for JsSegmentPermissions {
    fn from(p: idalib::segment::SegmentPermissions) -> Self {
        Self {
            read: p.is_readable(),
            write: p.is_writable(),
            execute: p.is_executable(),
        }
    }
}

#[napi(object)]
pub struct JsSegment {
    pub start_address: i64,
    pub end_address: i64,
    pub name: String,
    pub permissions: JsSegmentPermissions,
    pub alignment: String,
    #[napi(js_name = "type")]
    pub seg_type: String,
    pub color: u32,
    pub bit_size: u32,
}

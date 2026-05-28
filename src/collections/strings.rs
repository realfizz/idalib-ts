use napi_derive::napi;

use crate::database::IdbHandle;
use crate::types::string_entry::JsStringEntry;

#[napi]
pub struct StringsCollection {
    pub(crate) idb: IdbHandle,
}

#[napi]
impl StringsCollection {
    #[napi]
    pub fn list(&self) -> napi::Result<Vec<JsStringEntry>> {
        let guard = self
            .idb
            .lock()
            .map_err(|e| napi::Error::from_reason(format!("LockError: {}", e)))?;
        let idb = guard
            .as_ref()
            .ok_or_else(|| napi::Error::from_reason("Database is closed".to_string()))?;

        let strlist = idb.0.strings();
        let count = strlist.len();

        let mut entries = Vec::with_capacity(count);
        for idx in 0..count {
            let address = strlist.get_address_by_index(idx);
            let value = strlist.get_by_index(idx);

            if let (Some(addr), Some(val)) = (address, value) {
                let length = unsafe {
                    idalib::ffi::strings::idalib_get_strlist_item_length(idx)
                };
                entries.push(JsStringEntry {
                    index: idx as u32,
                    address: addr,
                    value: val,
                    length: length as u32,
                    str_type: 0,
                });
            }
        }

        Ok(entries)
    }
}

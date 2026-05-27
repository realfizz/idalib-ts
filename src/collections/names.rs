use napi_derive::napi;

use crate::database::IdbHandle;
use crate::types::name::{JsNameEntry, JsNameProperties};

#[napi]
pub struct NamesCollection {
    pub(crate) idb: IdbHandle,
}

#[napi]
impl NamesCollection {
    #[napi]
    pub async fn list(&self) -> napi::Result<Vec<JsNameEntry>> {
        let guard = self.idb.lock().map_err(|e| {
            napi::Error::from_reason(format!("LockError: {}", e))
        })?;
        let idb = guard.as_ref().ok_or_else(|| {
            napi::Error::from_reason("Database is closed".to_string())
        })?;

        let name_list = idb.0.names();
        let len = name_list.len();
        let mut entries = Vec::with_capacity(len);
        for idx in 0..len {
            if let Some(name) = name_list.get_by_index(idx) {
                entries.push(JsNameEntry {
                    index: idx as u32,
                    address: name.address(),
                    name: name.name().to_string(),
                    properties: JsNameProperties {
                        is_public: name.is_public(),
                        is_weak: name.is_weak(),
                    },
                });
            }
        }
        Ok(entries)
    }
}

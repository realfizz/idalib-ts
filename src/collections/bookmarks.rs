use napi_derive::napi;

use crate::types::bookmark::JsBookmark;

#[napi]
pub struct BookmarksCollection {
    pub(crate) idb: crate::database::IdbHandle,
}

#[napi]
impl BookmarksCollection {
    #[napi]
    pub async fn list(&self) -> napi::Result<Vec<JsBookmark>> {
        let guard = self.idb.lock().map_err(|e| {
            napi::Error::from_reason(format!("LockError: {}", e))
        })?;
        let idb = guard
            .as_ref()
            .ok_or_else(|| napi::Error::from_reason("Database is closed".to_string()))?;

        let bookmarks = idb.0.bookmarks();
        let len = bookmarks.len();
        let mut result = Vec::with_capacity(len as usize);

        for idx in 0..len {
            if let Some(address) = bookmarks.get_address(idx) {
                let description = bookmarks.get_description_by_index(idx).unwrap_or_default();
                result.push(JsBookmark {
                    address: address as i64,
                    description,
                    is_enabled: true,
                });
            }
        }

        Ok(result)
    }
}
